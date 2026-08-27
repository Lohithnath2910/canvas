use crate::fractional::{first_key, key_between};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Any, Array, ArrayRef, Doc, Map, MapPrelim, MapRef, Out, ReadTxn, StateVector, Transact, Update,
};

pub const MAX_PRIORITY: i32 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub id: String,
    pub text: String,
    pub done: bool,
    pub priority: i32,
    pub pos: String,
}

pub struct TodoListDoc {
    doc: Doc,
    items_ref: ArrayRef,
}

impl Default for TodoListDoc {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoListDoc {
    pub fn new() -> Self {
        let doc = Doc::new();
        let items_ref = doc.get_or_insert_array("items");
        Self { doc, items_ref }
    }

    pub fn load(snapshot: &[u8]) -> Self {
        let doc = Doc::new();
        let items_ref = doc.get_or_insert_array("items");
        {
            let mut txn = doc.transact_mut();
            if let Ok(update) = Update::decode_v1(snapshot) {
                txn.apply_update(update);
            }
        }
        Self { doc, items_ref }
    }

    pub fn items(&self) -> Vec<TodoItem> {
        let txn = self.doc.transact();
        let mut items = Vec::new();
        for value in self.items_ref.iter(&txn) {
            let Out::YMap(map) = value else {
                continue;
            };
            items.push(TodoItem {
                id: string_field(&txn, &map, "id"),
                text: string_field(&txn, &map, "text"),
                done: matches!(map.get(&txn, "done"), Some(Out::Any(Any::Bool(true)))),
                priority: int_field(&txn, &map, "priority"),
                pos: string_field(&txn, &map, "pos"),
            });
        }
        items.sort_by(|a, b| a.pos.cmp(&b.pos));
        items
    }

    pub fn is_empty(&self) -> bool {
        let txn = self.doc.transact();
        self.items_ref.len(&txn) == 0
    }

    pub fn counts(&self) -> (usize, usize) {
        let items = self.items();
        let done = items.iter().filter(|i| i.done).count();
        (items.len() - done, done)
    }

    pub fn add_item(&self, text: &str) -> (String, Vec<u8>) {
        let id = uuid::Uuid::new_v4().to_string();
        let update = self.insert_item(&id, text);
        (id, update)
    }

    fn insert_item(&self, id: &str, text: &str) -> Vec<u8> {
        let tail = self.items().last().map(|i| i.pos.clone());
        let pos = key_between(tail.as_deref(), None).unwrap_or_else(first_key);
        let mut txn = self.doc.transact_mut();
        let len = self.items_ref.len(&txn);
        self.items_ref.insert(
            &mut txn,
            len,
            MapPrelim::from([
                ("id", Any::String(id.into())),
                ("text", Any::String(text.into())),
                ("done", Any::Bool(false)),
                ("priority", Any::BigInt(0)),
                ("pos", Any::String(pos.as_str().into())),
            ]),
        );
        txn.encode_update_v1()
    }

    pub fn set_text(&self, id: &str, text: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.items_ref, id)?;
        if string_field(&txn, &target, "text") == text {
            return None;
        }
        target.insert(&mut txn, "text", Any::String(text.into()));
        Some(txn.encode_update_v1())
    }

    pub fn set_done(&self, id: &str, done: bool) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.items_ref, id)?;
        if matches!(target.get(&txn, "done"), Some(Out::Any(Any::Bool(current))) if current == done)
        {
            return None;
        }
        target.insert(&mut txn, "done", Any::Bool(done));
        Some(txn.encode_update_v1())
    }

    pub fn set_priority(&self, id: &str, level: i32) -> Option<Vec<u8>> {
        let level = level.clamp(0, MAX_PRIORITY);
        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.items_ref, id)?;
        if int_field(&txn, &target, "priority") == level {
            return None;
        }
        target.insert(&mut txn, "priority", Any::BigInt(level as i64));
        Some(txn.encode_update_v1())
    }

    pub fn cycle_priority(&self, id: &str) -> Option<Vec<u8>> {
        let current = {
            let txn = self.doc.transact();
            let target = find_map(&txn, &self.items_ref, id)?;
            int_field(&txn, &target, "priority")
        };
        let next = if !(0..MAX_PRIORITY).contains(&current) {
            0
        } else {
            current + 1
        };
        self.set_priority(id, next)
    }

    pub fn toggle_item(&self, id: &str) -> Option<Vec<u8>> {
        let current = {
            let txn = self.doc.transact();
            let target = find_map(&txn, &self.items_ref, id)?;
            matches!(target.get(&txn, "done"), Some(Out::Any(Any::Bool(true))))
        };
        self.set_done(id, !current)
    }

    pub fn move_item(&self, id: &str, index: usize) -> Option<Vec<u8>> {
        let neighbours: Vec<TodoItem> = self.items().into_iter().filter(|i| i.id != id).collect();
        let index = index.min(neighbours.len());
        let lower = index
            .checked_sub(1)
            .and_then(|i| neighbours.get(i))
            .map(|i| i.pos.clone());
        let upper = neighbours.get(index).map(|i| i.pos.clone());
        let pos = key_between(lower.as_deref(), upper.as_deref())?;

        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.items_ref, id)?;
        target.insert(&mut txn, "pos", Any::String(pos.as_str().into()));
        Some(txn.encode_update_v1())
    }

    pub fn backfill_positions(&self) -> Option<Vec<u8>> {
        let items = self.items();
        if items.iter().all(|i| !i.pos.is_empty()) {
            return None;
        }
        let mut previous: Option<String> = items
            .iter()
            .take_while(|i| !i.pos.is_empty())
            .last()
            .map(|i| i.pos.clone());

        let mut txn = self.doc.transact_mut();
        let mut wrote = false;
        for item in items.iter().filter(|i| i.pos.is_empty()) {
            let Some(pos) = key_between(previous.as_deref(), None) else {
                break;
            };
            let Some(target) = find_map(&txn, &self.items_ref, &item.id) else {
                continue;
            };
            target.insert(&mut txn, "pos", Any::String(pos.as_str().into()));
            previous = Some(pos);
            wrote = true;
        }
        if !wrote {
            return None;
        }
        Some(txn.encode_update_v1())
    }

    pub fn delete_item(&self, id: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let index = find_index(&txn, &self.items_ref, id)?;
        self.items_ref.remove(&mut txn, index);
        Some(txn.encode_update_v1())
    }

    pub fn apply_update(&self, payload: &[u8]) -> bool {
        let Ok(update) = Update::decode_v1(payload) else {
            return false;
        };
        let mut txn = self.doc.transact_mut();
        txn.apply_update(update);
        true
    }

    pub fn snapshot(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&StateVector::default())
    }

    pub fn state_vector(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.state_vector().encode_v1()
    }

    pub fn diff_since(&self, remote_sv: &[u8]) -> Vec<u8> {
        let Ok(sv) = StateVector::decode_v1(remote_sv) else {
            return self.snapshot();
        };
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&sv)
    }
}

fn string_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> String {
    match map.get(txn, key) {
        Some(Out::Any(Any::String(s))) => s.to_string(),
        _ => String::new(),
    }
}

fn int_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> i32 {
    match map.get(txn, key) {
        Some(Out::Any(Any::BigInt(n))) => n as i32,
        Some(Out::Any(Any::Number(n))) => n as i32,
        _ => 0,
    }
}

fn find_map<T: ReadTxn>(txn: &T, items: &ArrayRef, id: &str) -> Option<MapRef> {
    for value in items.iter(txn) {
        if let Out::YMap(map) = value {
            if string_field(txn, &map, "id") == id {
                return Some(map);
            }
        }
    }
    None
}

fn find_index<T: ReadTxn>(txn: &T, items: &ArrayRef, id: &str) -> Option<u32> {
    for (index, value) in items.iter(txn).enumerate() {
        if let Out::YMap(map) = value {
            if string_field(txn, &map, "id") == id {
                return Some(index as u32);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded(texts: &[&str]) -> (TodoListDoc, Vec<String>) {
        let doc = TodoListDoc::new();
        let ids = texts.iter().map(|t| doc.add_item(t).0).collect();
        (doc, ids)
    }

    #[test]
    fn a_new_list_is_empty() {
        let doc = TodoListDoc::new();
        assert!(doc.is_empty());
        assert_eq!(doc.counts(), (0, 0));
        assert!(doc.items().is_empty());
    }

    #[test]
    fn items_keep_insertion_order() {
        let (doc, _) = seeded(&["first", "second", "third"]);
        let texts: Vec<String> = doc.items().into_iter().map(|i| i.text).collect();
        assert_eq!(texts, ["first", "second", "third"]);
    }

    #[test]
    fn round_trips_through_snapshot() {
        let (doc, ids) = seeded(&["buy milk", "write tests"]);
        doc.set_done(&ids[1], true);

        let restored = TodoListDoc::load(&doc.snapshot());
        assert_eq!(restored.items(), doc.items());
        assert_eq!(restored.counts(), (1, 1));
    }

    #[test]
    fn empty_list_round_trips() {
        let doc = TodoListDoc::new();
        let restored = TodoListDoc::load(&doc.snapshot());
        assert!(restored.is_empty());
    }

    #[test]
    fn toggle_flips_and_reports_counts() {
        let (doc, ids) = seeded(&["a", "b", "c"]);
        assert_eq!(doc.counts(), (3, 0));
        doc.toggle_item(&ids[0]);
        doc.toggle_item(&ids[2]);
        assert_eq!(doc.counts(), (1, 2));
        doc.toggle_item(&ids[0]);
        assert_eq!(doc.counts(), (2, 1));
    }

    #[test]
    fn setting_the_same_value_produces_no_update() {
        let (doc, ids) = seeded(&["unchanged"]);
        assert!(doc.set_text(&ids[0], "unchanged").is_none());
        assert!(doc.set_done(&ids[0], false).is_none());
        assert!(doc.set_text(&ids[0], "changed").is_some());
    }

    #[test]
    fn editing_an_unknown_id_is_a_no_op() {
        let (doc, _) = seeded(&["a"]);
        assert!(doc.set_text("no-such-id", "x").is_none());
        assert!(doc.set_done("no-such-id", true).is_none());
        assert!(doc.toggle_item("no-such-id").is_none());
        assert!(doc.delete_item("no-such-id").is_none());
        assert_eq!(doc.items().len(), 1);
    }

    #[test]
    fn delete_removes_only_the_named_item() {
        let (doc, ids) = seeded(&["a", "b", "c"]);
        doc.delete_item(&ids[1]);
        let texts: Vec<String> = doc.items().into_iter().map(|i| i.text).collect();
        assert_eq!(texts, ["a", "c"]);
    }

    #[test]
    fn item_text_handles_unicode_without_panicking() {
        let (doc, ids) = seeded(&["héllo wörld 🎉"]);
        doc.set_text(&ids[0], "héllo wörld 🎉🎉");
        assert_eq!(doc.items()[0].text, "héllo wörld 🎉🎉");
        doc.set_text(&ids[0], "");
        assert_eq!(doc.items()[0].text, "");
    }

    #[test]
    fn concurrent_adds_converge_without_loss() {
        let a = TodoListDoc::new();
        a.add_item("shared");
        let b = TodoListDoc::load(&a.snapshot());

        let from_a = a.add_item("from A").1;
        let from_b = b.add_item("from B").1;

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.items(), b.items());
        assert_eq!(a.items().len(), 3, "an add was lost");
    }

    #[test]
    fn concurrent_toggles_of_the_same_item_converge() {
        let a = TodoListDoc::new();
        let (id, _) = a.add_item("contested");
        let b = TodoListDoc::load(&a.snapshot());

        let from_a = a.set_done(&id, true).unwrap();
        let from_b = b.set_text(&id, "contested, renamed").unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.items(), b.items());
        assert!(a.items()[0].done);
        assert_eq!(a.items()[0].text, "contested, renamed");
    }

    #[test]
    fn a_concurrent_delete_and_edit_converge() {
        let a = TodoListDoc::new();
        let (keep, _) = a.add_item("keep");
        let (drop, _) = a.add_item("drop");
        let b = TodoListDoc::load(&a.snapshot());

        let from_a = a.delete_item(&drop).unwrap();
        let from_b = b.set_text(&keep, "keep, edited").unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.items(), b.items());
        assert_eq!(a.items().len(), 1);
        assert_eq!(a.items()[0].text, "keep, edited");
    }

    #[test]
    fn three_way_concurrent_adds_converge() {
        let a = TodoListDoc::new();
        a.add_item("base");
        let b = TodoListDoc::load(&a.snapshot());
        let c = TodoListDoc::load(&a.snapshot());

        let ua = a.add_item("from A").1;
        let ub = b.add_item("from B").1;
        let uc = c.add_item("from C").1;

        for (doc, updates) in [
            (&a, vec![&ub, &uc]),
            (&b, vec![&ua, &uc]),
            (&c, vec![&ua, &ub]),
        ] {
            for u in updates {
                doc.apply_update(u);
            }
        }

        assert_eq!(a.items(), b.items());
        assert_eq!(b.items(), c.items());
        assert_eq!(a.items().len(), 4);
    }

    #[test]
    fn updates_are_idempotent() {
        let a = TodoListDoc::new();
        a.add_item("one");
        let b = TodoListDoc::load(&a.snapshot());
        let update = a.add_item("two").1;

        b.apply_update(&update);
        b.apply_update(&update);
        b.apply_update(&update);

        assert_eq!(b.items().len(), 2);
    }

    #[test]
    fn diff_since_sends_only_what_is_missing() {
        let a = TodoListDoc::new();
        for text in ["one", "two", "three", "four", "five"] {
            a.add_item(text);
        }
        let b = TodoListDoc::load(&a.snapshot());

        a.add_item("six");
        let diff = a.diff_since(&b.state_vector());
        assert!(diff.len() < a.snapshot().len());

        b.apply_update(&diff);
        assert_eq!(a.items(), b.items());
    }

    #[test]
    fn garbage_update_is_rejected_without_panicking() {
        let (doc, _) = seeded(&["intact"]);
        assert!(!doc.apply_update(&[0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(doc.items()[0].text, "intact");
    }

    #[test]
    fn a_new_item_has_no_priority() {
        let doc = TodoListDoc::new();
        doc.add_item("plain");
        assert_eq!(doc.items()[0].priority, 0);
    }

    #[test]
    fn priority_cycles_up_and_wraps_to_none() {
        let doc = TodoListDoc::new();
        let (id, _) = doc.add_item("cycle me");
        for expected in [1, 2, 3, 0, 1] {
            doc.cycle_priority(&id);
            assert_eq!(doc.items()[0].priority, expected);
        }
    }

    #[test]
    fn priority_is_clamped_to_the_scale() {
        let doc = TodoListDoc::new();
        let (id, _) = doc.add_item("clamp me");
        doc.set_priority(&id, 99);
        assert_eq!(doc.items()[0].priority, MAX_PRIORITY);
        doc.set_priority(&id, -5);
        assert_eq!(doc.items()[0].priority, 0);
    }

    #[test]
    fn setting_the_same_priority_produces_no_update() {
        let doc = TodoListDoc::new();
        let (id, _) = doc.add_item("steady");
        assert!(doc.set_priority(&id, 2).is_some());
        assert!(doc.set_priority(&id, 2).is_none());
    }

    #[test]
    fn priority_survives_a_snapshot() {
        let doc = TodoListDoc::new();
        let (a, _) = doc.add_item("high");
        let (b, _) = doc.add_item("low");
        doc.set_priority(&a, 3);
        doc.set_priority(&b, 1);

        let restored = TodoListDoc::load(&doc.snapshot());
        assert_eq!(restored.items(), doc.items());
        assert_eq!(restored.items()[0].priority, 3);
        assert_eq!(restored.items()[1].priority, 1);
    }

    #[test]
    fn a_list_written_before_priority_existed_still_opens() {
        let legacy = Doc::new();
        let items = legacy.get_or_insert_array("items");
        {
            let mut txn = legacy.transact_mut();
            items.insert(
                &mut txn,
                0,
                MapPrelim::from([
                    ("id", Any::String("old-1".into())),
                    ("text", Any::String("written last week".into())),
                    ("done", Any::Bool(false)),
                ]),
            );
        }
        let snapshot = {
            let txn = legacy.transact();
            txn.encode_state_as_update_v1(&StateVector::default())
        };

        let doc = TodoListDoc::load(&snapshot);
        let items = doc.items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].text, "written last week");
        assert_eq!(items[0].priority, 0);

        doc.set_priority("old-1", 2);
        assert_eq!(doc.items()[0].priority, 2);
    }

    #[test]
    fn two_docs_setting_priority_on_different_items_converge() {
        let a = TodoListDoc::new();
        let (one, _) = a.add_item("one");
        let (two, _) = a.add_item("two");
        let b = TodoListDoc::load(&a.snapshot());

        let from_a = a.set_priority(&one, 3).unwrap();
        let from_b = b.set_priority(&two, 1).unwrap();
        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.items(), b.items());
        assert_eq!(a.items()[0].priority, 3);
        assert_eq!(a.items()[1].priority, 1);
    }

    #[test]
    fn priority_on_an_unknown_item_is_a_no_op() {
        let doc = TodoListDoc::new();
        doc.add_item("real");
        assert!(doc.set_priority("ghost", 2).is_none());
        assert!(doc.cycle_priority("ghost").is_none());
    }

    fn texts(doc: &TodoListDoc) -> Vec<String> {
        doc.items().into_iter().map(|i| i.text).collect()
    }

    #[test]
    fn new_items_get_positions_and_keep_insertion_order() {
        let doc = TodoListDoc::new();
        doc.add_item("first");
        doc.add_item("second");
        doc.add_item("third");
        assert_eq!(texts(&doc), ["first", "second", "third"]);
        assert!(doc.items().iter().all(|i| !i.pos.is_empty()));
    }

    #[test]
    fn an_item_moves_to_the_front() {
        let doc = TodoListDoc::new();
        doc.add_item("a");
        doc.add_item("b");
        let (c, _) = doc.add_item("c");
        doc.move_item(&c, 0).unwrap();
        assert_eq!(texts(&doc), ["c", "a", "b"]);
    }

    #[test]
    fn an_item_moves_into_the_middle_and_to_the_end() {
        let doc = TodoListDoc::new();
        let (a, _) = doc.add_item("a");
        doc.add_item("b");
        doc.add_item("c");
        doc.move_item(&a, 1).unwrap();
        assert_eq!(texts(&doc), ["b", "a", "c"]);
        doc.move_item(&a, 2).unwrap();
        assert_eq!(texts(&doc), ["b", "c", "a"]);
    }

    #[test]
    fn a_move_past_the_end_lands_at_the_end() {
        let doc = TodoListDoc::new();
        let (a, _) = doc.add_item("a");
        doc.add_item("b");
        doc.move_item(&a, 99).unwrap();
        assert_eq!(texts(&doc), ["b", "a"]);
    }

    #[test]
    fn a_new_order_survives_a_snapshot() {
        let doc = TodoListDoc::new();
        doc.add_item("a");
        doc.add_item("b");
        let (c, _) = doc.add_item("c");
        doc.move_item(&c, 0);

        let restored = TodoListDoc::load(&doc.snapshot());
        assert_eq!(texts(&restored), ["c", "a", "b"]);
        assert_eq!(restored.items(), doc.items());
    }

    #[test]
    fn many_moves_into_the_same_gap_stay_ordered() {
        let doc = TodoListDoc::new();
        doc.add_item("head");
        doc.add_item("tail");
        for n in 0..24 {
            let (id, _) = doc.add_item(&format!("wedge {n}"));
            doc.move_item(&id, 1).unwrap();
        }
        let order = texts(&doc);
        assert_eq!(order.len(), 26);
        assert_eq!(order[0], "head");
        assert_eq!(order[25], "tail");
        assert_eq!(order[1], "wedge 23");
    }

    #[test]
    fn two_docs_moving_different_items_converge() {
        let a = TodoListDoc::new();
        let (one, _) = a.add_item("one");
        a.add_item("two");
        let (three, _) = a.add_item("three");
        let b = TodoListDoc::load(&a.snapshot());

        let from_a = a.move_item(&three, 0).unwrap();
        let from_b = b.move_item(&one, 2).unwrap();
        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.items(), b.items());
        assert_eq!(texts(&a).len(), 3);
    }

    #[test]
    fn a_list_written_before_positions_existed_keeps_its_order() {
        let legacy = Doc::new();
        let items = legacy.get_or_insert_array("items");
        {
            let mut txn = legacy.transact_mut();
            for (i, text) in ["alpha", "beta", "gamma"].iter().enumerate() {
                items.insert(
                    &mut txn,
                    i as u32,
                    MapPrelim::from([
                        ("id", Any::String(format!("old-{i}").into())),
                        ("text", Any::String((*text).into())),
                        ("done", Any::Bool(false)),
                    ]),
                );
            }
        }
        let snapshot = {
            let txn = legacy.transact();
            txn.encode_state_as_update_v1(&StateVector::default())
        };

        let doc = TodoListDoc::load(&snapshot);
        assert_eq!(texts(&doc), ["alpha", "beta", "gamma"]);

        assert!(doc.backfill_positions().is_some());
        assert_eq!(texts(&doc), ["alpha", "beta", "gamma"]);
        assert!(doc.items().iter().all(|i| !i.pos.is_empty()));
        assert!(doc.backfill_positions().is_none());

        doc.move_item("old-2", 0).unwrap();
        assert_eq!(texts(&doc), ["gamma", "alpha", "beta"]);
    }

    #[test]
    fn backfill_leaves_an_already_positioned_list_alone() {
        let doc = TodoListDoc::new();
        doc.add_item("a");
        doc.add_item("b");
        assert!(doc.backfill_positions().is_none());
    }

    #[test]
    fn a_backfilled_list_appends_after_the_old_items() {
        let legacy = Doc::new();
        let items = legacy.get_or_insert_array("items");
        {
            let mut txn = legacy.transact_mut();
            items.insert(
                &mut txn,
                0,
                MapPrelim::from([
                    ("id", Any::String("old-1".into())),
                    ("text", Any::String("was here first".into())),
                    ("done", Any::Bool(false)),
                ]),
            );
        }
        let snapshot = {
            let txn = legacy.transact();
            txn.encode_state_as_update_v1(&StateVector::default())
        };

        let doc = TodoListDoc::load(&snapshot);
        doc.backfill_positions();
        doc.add_item("added after");
        assert_eq!(texts(&doc), ["was here first", "added after"]);
    }

    #[test]
    fn moving_an_unknown_item_is_a_no_op() {
        let doc = TodoListDoc::new();
        doc.add_item("real");
        assert!(doc.move_item("ghost", 0).is_none());
    }
}
