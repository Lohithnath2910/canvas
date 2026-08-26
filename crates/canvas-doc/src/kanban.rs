use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Any, Array, ArrayRef, Doc, Map, MapPrelim, MapRef, Out, ReadTxn, StateVector, Transact, Update,
};

use crate::fractional::{first_key, key_between};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KanbanColumn {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KanbanCard {
    pub id: String,
    pub column: String,
    pub title: String,
    pub assignee: String,
    pub due: i64,
    pub pos: String,
}

pub struct KanbanDoc {
    doc: Doc,
    columns_ref: ArrayRef,
    cards_ref: MapRef,
}

impl Default for KanbanDoc {
    fn default() -> Self {
        Self::new()
    }
}

impl KanbanDoc {
    pub fn new() -> Self {
        let doc = Doc::new();
        let columns_ref = doc.get_or_insert_array("columns");
        let cards_ref = doc.get_or_insert_map("cards");
        Self {
            doc,
            columns_ref,
            cards_ref,
        }
    }

    pub fn load(snapshot: &[u8]) -> Self {
        let doc = Doc::new();
        let columns_ref = doc.get_or_insert_array("columns");
        let cards_ref = doc.get_or_insert_map("cards");
        {
            let mut txn = doc.transact_mut();
            if let Ok(update) = Update::decode_v1(snapshot) {
                txn.apply_update(update);
            }
        }
        Self {
            doc,
            columns_ref,
            cards_ref,
        }
    }

    pub fn columns(&self) -> Vec<KanbanColumn> {
        let txn = self.doc.transact();
        let mut columns = Vec::new();
        for value in self.columns_ref.iter(&txn) {
            let Out::YMap(map) = value else {
                continue;
            };
            columns.push(KanbanColumn {
                id: string_field(&txn, &map, "id"),
                title: string_field(&txn, &map, "title"),
            });
        }
        columns
    }

    pub const DEFAULT_COLUMNS: [&str; 3] = ["TODO", "DOING", "DONE"];

    pub fn seeded() -> Self {
        let doc = Self::new();
        for title in Self::DEFAULT_COLUMNS {
            doc.add_column(title);
        }
        doc
    }

    pub fn add_column(&self, title: &str) -> (String, Vec<u8>) {
        let id = uuid::Uuid::new_v4().to_string();
        let mut txn = self.doc.transact_mut();
        let len = self.columns_ref.len(&txn);
        self.columns_ref.insert(
            &mut txn,
            len,
            MapPrelim::from([
                ("id", Any::String(id.as_str().into())),
                ("title", Any::String(title.into())),
            ]),
        );
        (id, txn.encode_update_v1())
    }

    pub fn cards(&self) -> Vec<KanbanCard> {
        let txn = self.doc.transact();
        let mut cards = Vec::new();
        for (_, value) in self.cards_ref.iter(&txn) {
            let Out::YMap(map) = value else {
                continue;
            };
            cards.push(KanbanCard {
                id: string_field(&txn, &map, "id"),
                column: string_field(&txn, &map, "column"),
                title: string_field(&txn, &map, "title"),
                assignee: string_field(&txn, &map, "assignee"),
                due: int_field(&txn, &map, "due"),
                pos: string_field(&txn, &map, "pos"),
            });
        }
        cards.sort_by(|a, b| {
            a.column
                .cmp(&b.column)
                .then_with(|| a.pos.cmp(&b.pos))
                .then_with(|| a.id.cmp(&b.id))
        });
        cards
    }

    pub fn cards_in(&self, column: &str) -> Vec<KanbanCard> {
        self.cards()
            .into_iter()
            .filter(|c| c.column == column)
            .collect()
    }

    pub fn card_count(&self, column: &str) -> usize {
        self.cards().iter().filter(|c| c.column == column).count()
    }

    pub fn is_empty(&self) -> bool {
        let txn = self.doc.transact();
        self.cards_ref.len(&txn) == 0
    }

    pub fn add_card(&self, column: &str, title: &str) -> (String, Vec<u8>) {
        let id = uuid::Uuid::new_v4().to_string();
        let existing = self.cards_in(column);
        let pos =
            key_between(existing.last().map(|c| c.pos.as_str()), None).unwrap_or_else(first_key);

        let mut txn = self.doc.transact_mut();
        self.cards_ref.insert(
            &mut txn,
            id.clone(),
            MapPrelim::from([
                ("id", Any::String(id.as_str().into())),
                ("column", Any::String(column.into())),
                ("title", Any::String(title.into())),
                ("assignee", Any::String(String::new().into())),
                ("due", Any::BigInt(0)),
                ("pos", Any::String(pos.as_str().into())),
            ]),
        );
        (id, txn.encode_update_v1())
    }

    pub fn set_title(&self, id: &str, title: &str) -> Option<Vec<u8>> {
        self.set_string(id, "title", title)
    }

    pub fn set_assignee(&self, id: &str, assignee: &str) -> Option<Vec<u8>> {
        self.set_string(id, "assignee", assignee)
    }

    fn set_string(&self, id: &str, key: &str, value: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let Some(Out::YMap(card)) = self.cards_ref.get(&txn, id) else {
            return None;
        };
        if string_field(&txn, &card, key) == value {
            return None;
        }
        card.insert(&mut txn, key.to_string(), Any::String(value.into()));
        Some(txn.encode_update_v1())
    }

    pub fn set_due(&self, id: &str, due: i64) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let Some(Out::YMap(card)) = self.cards_ref.get(&txn, id) else {
            return None;
        };
        if int_field(&txn, &card, "due") == due {
            return None;
        }
        card.insert(&mut txn, "due", Any::BigInt(due));
        Some(txn.encode_update_v1())
    }

    pub fn move_card(&self, id: &str, column: &str, index: usize) -> Option<Vec<u8>> {
        let neighbours: Vec<KanbanCard> = self
            .cards_in(column)
            .into_iter()
            .filter(|c| c.id != id)
            .collect();
        let index = index.min(neighbours.len());
        let lower = index
            .checked_sub(1)
            .and_then(|i| neighbours.get(i))
            .map(|c| c.pos.clone());
        let upper = neighbours.get(index).map(|c| c.pos.clone());
        let pos = key_between(lower.as_deref(), upper.as_deref())?;

        let mut txn = self.doc.transact_mut();
        let Some(Out::YMap(card)) = self.cards_ref.get(&txn, id) else {
            return None;
        };
        card.insert(&mut txn, "column", Any::String(column.into()));
        card.insert(&mut txn, "pos", Any::String(pos.as_str().into()));
        Some(txn.encode_update_v1())
    }

    pub fn delete_card(&self, id: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        self.cards_ref.get(&txn, id)?;
        self.cards_ref.remove(&mut txn, id);
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

fn int_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> i64 {
    match map.get(txn, key) {
        Some(Out::Any(Any::BigInt(n))) => n,
        Some(Out::Any(Any::Number(n))) => n as i64,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board() -> (KanbanDoc, String, String, String) {
        let doc = KanbanDoc::new();
        let (todo, _) = doc.add_column("TODO");
        let (doing, _) = doc.add_column("DOING");
        let (done, _) = doc.add_column("DONE");
        (doc, todo, doing, done)
    }

    fn titles(doc: &KanbanDoc, column: &str) -> Vec<String> {
        doc.cards_in(column).into_iter().map(|c| c.title).collect()
    }

    #[test]
    fn a_new_board_is_empty() {
        let doc = KanbanDoc::new();
        assert!(doc.is_empty());
        assert!(doc.columns().is_empty());
    }

    #[test]
    fn columns_keep_insertion_order() {
        let (doc, _, _, _) = board();
        let names: Vec<String> = doc.columns().into_iter().map(|c| c.title).collect();
        assert_eq!(names, ["TODO", "DOING", "DONE"]);
    }

    #[test]
    fn cards_append_to_the_end_of_their_column() {
        let (doc, todo, _, _) = board();
        doc.add_card(&todo, "first");
        doc.add_card(&todo, "second");
        doc.add_card(&todo, "third");
        assert_eq!(titles(&doc, &todo), ["first", "second", "third"]);
        assert_eq!(doc.card_count(&todo), 3);
    }

    #[test]
    fn round_trips_through_snapshot() {
        let (doc, todo, doing, _) = board();
        let (id, _) = doc.add_card(&todo, "carried");
        doc.set_assignee(&id, "AB");
        doc.set_due(&id, 1_756_000_000_123);
        doc.move_card(&id, &doing, 0);

        let restored = KanbanDoc::load(&doc.snapshot());
        assert_eq!(restored.cards(), doc.cards());
        assert_eq!(restored.columns(), doc.columns());
        assert_eq!(restored.cards()[0].assignee, "AB");
        assert_eq!(restored.cards()[0].due, 1_756_000_000_123);
    }

    #[test]
    fn empty_board_round_trips() {
        let doc = KanbanDoc::new();
        assert!(KanbanDoc::load(&doc.snapshot()).is_empty());
    }

    #[test]
    fn a_card_moved_between_columns_keeps_its_identity() {
        let (doc, todo, doing, _) = board();
        let (id, _) = doc.add_card(&todo, "travelling");
        doc.set_assignee(&id, "CD");

        doc.move_card(&id, &doing, 0);

        assert_eq!(doc.card_count(&todo), 0);
        assert_eq!(doc.card_count(&doing), 1);
        let moved = &doc.cards_in(&doing)[0];
        assert_eq!(moved.id, id);
        assert_eq!(moved.title, "travelling");
        assert_eq!(moved.assignee, "CD");
    }

    #[test]
    fn moving_within_a_column_lands_at_the_requested_index() {
        let (doc, todo, _, _) = board();
        doc.add_card(&todo, "a");
        doc.add_card(&todo, "b");
        let (c, _) = doc.add_card(&todo, "c");

        doc.move_card(&c, &todo, 0);
        assert_eq!(titles(&doc, &todo), ["c", "a", "b"]);

        doc.move_card(&c, &todo, 1);
        assert_eq!(titles(&doc, &todo), ["a", "c", "b"]);

        doc.move_card(&c, &todo, 2);
        assert_eq!(titles(&doc, &todo), ["a", "b", "c"]);
    }

    #[test]
    fn an_index_past_the_end_lands_last_rather_than_failing() {
        let (doc, todo, _, _) = board();
        let (a, _) = doc.add_card(&todo, "a");
        doc.add_card(&todo, "b");
        assert!(doc.move_card(&a, &todo, 99).is_some());
        assert_eq!(titles(&doc, &todo), ["b", "a"]);
    }

    #[test]
    fn many_moves_into_the_same_gap_stay_ordered() {
        let (doc, todo, _, _) = board();
        doc.add_card(&todo, "top");
        doc.add_card(&todo, "bottom");
        for n in 0..200 {
            let (id, _) = doc.add_card(&todo, &format!("card {n}"));
            assert!(doc.move_card(&id, &todo, 1).is_some(), "move {n} failed");
        }
        let ordered = doc.cards_in(&todo);
        assert_eq!(ordered.len(), 202);
        assert_eq!(ordered.first().unwrap().title, "top");
        assert_eq!(ordered.last().unwrap().title, "bottom");
        for pair in ordered.windows(2) {
            assert!(pair[0].pos < pair[1].pos, "positions went out of order");
        }
    }

    #[test]
    fn setting_the_same_value_produces_no_update() {
        let (doc, todo, _, _) = board();
        let (id, _) = doc.add_card(&todo, "fixed");
        assert!(doc.set_title(&id, "fixed").is_none());
        assert!(doc.set_assignee(&id, "").is_none());
        assert!(doc.set_due(&id, 0).is_none());
        assert!(doc.set_title(&id, "renamed").is_some());
    }

    #[test]
    fn touching_an_unknown_card_is_a_no_op() {
        let (doc, todo, _, _) = board();
        doc.add_card(&todo, "only");
        assert!(doc.set_title("no-such-id", "x").is_none());
        assert!(doc.set_due("no-such-id", 1).is_none());
        assert!(doc.move_card("no-such-id", &todo, 0).is_none());
        assert!(doc.delete_card("no-such-id").is_none());
        assert_eq!(doc.cards().len(), 1);
    }

    #[test]
    fn delete_removes_only_the_named_card() {
        let (doc, todo, _, _) = board();
        doc.add_card(&todo, "keep");
        let (drop, _) = doc.add_card(&todo, "drop");
        doc.delete_card(&drop);
        assert_eq!(titles(&doc, &todo), ["keep"]);
    }

    #[test]
    fn two_docs_moving_the_same_card_converge_without_duplication_or_loss() {
        let (a, todo, doing, done) = board();
        let (id, _) = a.add_card(&todo, "contested");
        a.add_card(&todo, "bystander");
        let b = KanbanDoc::load(&a.snapshot());

        let from_a = a.move_card(&id, &doing, 0).unwrap();
        let from_b = b.move_card(&id, &done, 0).unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.cards(), b.cards(), "the two boards diverged");
        assert_eq!(a.cards().len(), 2, "a card was duplicated or lost");
        assert_eq!(
            a.cards().iter().filter(|c| c.id == id).count(),
            1,
            "the contested card exists more than once"
        );

        let survivor = a.cards().into_iter().find(|c| c.id == id).unwrap();
        assert!(survivor.column == doing || survivor.column == done);
        assert_eq!(
            a.card_count(&todo),
            1,
            "the bystander should not have moved"
        );
    }

    #[test]
    fn concurrent_moves_of_different_cards_both_land() {
        let (a, todo, doing, _) = board();
        let (one, _) = a.add_card(&todo, "one");
        let (two, _) = a.add_card(&todo, "two");
        let b = KanbanDoc::load(&a.snapshot());

        let from_a = a.move_card(&one, &doing, 0).unwrap();
        let from_b = b.move_card(&two, &doing, 0).unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.cards(), b.cards());
        assert_eq!(a.card_count(&doing), 2);
        assert_eq!(a.card_count(&todo), 0);
    }

    #[test]
    fn a_concurrent_move_and_rename_both_survive() {
        let (a, todo, doing, _) = board();
        let (id, _) = a.add_card(&todo, "planning");
        let b = KanbanDoc::load(&a.snapshot());

        let from_a = a.move_card(&id, &doing, 0).unwrap();
        let from_b = b.set_title(&id, "planning, renamed").unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.cards(), b.cards());
        assert_eq!(a.cards()[0].column, doing);
        assert_eq!(a.cards()[0].title, "planning, renamed");
    }

    #[test]
    fn concurrent_adds_to_the_same_column_both_survive_in_a_stable_order() {
        let (a, todo, _, _) = board();
        a.add_card(&todo, "shared");
        let b = KanbanDoc::load(&a.snapshot());

        let from_a = a.add_card(&todo, "from A").1;
        let from_b = b.add_card(&todo, "from B").1;

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.cards(), b.cards());
        assert_eq!(a.card_count(&todo), 3);
        assert_eq!(titles(&a, &todo), titles(&b, &todo));
    }

    #[test]
    fn three_way_concurrent_moves_converge() {
        let (a, todo, doing, done) = board();
        let (one, _) = a.add_card(&todo, "one");
        let (two, _) = a.add_card(&todo, "two");
        let (three, _) = a.add_card(&todo, "three");
        let b = KanbanDoc::load(&a.snapshot());
        let c = KanbanDoc::load(&a.snapshot());

        let ua = a.move_card(&one, &doing, 0).unwrap();
        let ub = b.move_card(&two, &done, 0).unwrap();
        let uc = c.move_card(&three, &doing, 0).unwrap();

        for (doc, updates) in [
            (&a, vec![&ub, &uc]),
            (&b, vec![&ua, &uc]),
            (&c, vec![&ua, &ub]),
        ] {
            for u in updates {
                doc.apply_update(u);
            }
        }

        assert_eq!(a.cards(), b.cards());
        assert_eq!(b.cards(), c.cards());
        assert_eq!(a.cards().len(), 3);
    }

    #[test]
    fn a_concurrent_delete_and_move_do_not_resurrect_the_card() {
        let (a, todo, doing, _) = board();
        let (id, _) = a.add_card(&todo, "doomed");
        a.add_card(&todo, "survivor");
        let b = KanbanDoc::load(&a.snapshot());

        let from_a = a.delete_card(&id).unwrap();
        let from_b = b.move_card(&id, &doing, 0).unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.cards(), b.cards());
        assert_eq!(a.cards().len(), 1);
        assert_eq!(a.cards()[0].title, "survivor");
    }

    #[test]
    fn updates_are_idempotent() {
        let (a, todo, _, _) = board();
        a.add_card(&todo, "one");
        let b = KanbanDoc::load(&a.snapshot());
        let update = a.add_card(&todo, "two").1;

        b.apply_update(&update);
        b.apply_update(&update);
        b.apply_update(&update);

        assert_eq!(b.cards().len(), 2);
    }

    #[test]
    fn diff_since_sends_only_what_is_missing() {
        let (a, todo, _, _) = board();
        for n in 0..6 {
            a.add_card(&todo, &format!("card {n}"));
        }
        let b = KanbanDoc::load(&a.snapshot());

        a.add_card(&todo, "new");
        let diff = a.diff_since(&b.state_vector());
        assert!(diff.len() < a.snapshot().len());

        b.apply_update(&diff);
        assert_eq!(a.cards(), b.cards());
    }

    #[test]
    fn garbage_update_is_rejected_without_panicking() {
        let (doc, todo, _, _) = board();
        doc.add_card(&todo, "intact");
        assert!(!doc.apply_update(&[0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(doc.cards()[0].title, "intact");
    }

    #[test]
    fn titles_handle_unicode_without_panicking() {
        let (doc, todo, _, _) = board();
        let (id, _) = doc.add_card(&todo, "héllo 🎉");
        doc.set_title(&id, "héllo 🎉🎉");
        assert_eq!(doc.cards()[0].title, "héllo 🎉🎉");
        doc.set_title(&id, "");
        assert_eq!(doc.cards()[0].title, "");
    }

    #[test]
    fn a_seeded_board_opens_with_three_named_columns() {
        let doc = KanbanDoc::seeded();
        let titles: Vec<String> = doc.columns().into_iter().map(|c| c.title).collect();
        assert_eq!(titles, KanbanDoc::DEFAULT_COLUMNS);
        assert!(doc.cards().is_empty());
    }

    #[test]
    fn a_seeded_board_keeps_its_column_ids_across_a_reload() {
        let doc = KanbanDoc::seeded();
        let before: Vec<String> = doc.columns().into_iter().map(|c| c.id).collect();
        let reloaded = KanbanDoc::load(&doc.snapshot());
        let after: Vec<String> = reloaded.columns().into_iter().map(|c| c.id).collect();
        assert_eq!(before, after);
    }

    #[test]
    fn seeding_twice_never_produces_the_same_column_ids() {
        let a: Vec<String> = KanbanDoc::seeded()
            .columns()
            .into_iter()
            .map(|c| c.id)
            .collect();
        let b: Vec<String> = KanbanDoc::seeded()
            .columns()
            .into_iter()
            .map(|c| c.id)
            .collect();
        for id in &a {
            assert!(!b.contains(id));
        }
    }
}
