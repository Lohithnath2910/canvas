use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Any, Array, ArrayRef, Doc, Map, MapPrelim, MapRef, Out, ReadTxn, StateVector, Transact, Update,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub start: i64,
    pub end: i64,
    pub location: String,
    pub notes: String,
    pub attendees: Vec<String>,
}

impl CalendarEvent {
    pub fn duration(&self) -> i64 {
        (self.end - self.start).max(0)
    }
}

pub struct CalendarDoc {
    doc: Doc,
    events_ref: ArrayRef,
}

impl Default for CalendarDoc {
    fn default() -> Self {
        Self::new()
    }
}

impl CalendarDoc {
    pub fn new() -> Self {
        let doc = Doc::new();
        let events_ref = doc.get_or_insert_array("events");
        Self { doc, events_ref }
    }

    pub fn load(snapshot: &[u8]) -> Self {
        let doc = Doc::new();
        let events_ref = doc.get_or_insert_array("events");
        {
            let mut txn = doc.transact_mut();
            if let Ok(update) = Update::decode_v1(snapshot) {
                txn.apply_update(update);
            }
        }
        Self { doc, events_ref }
    }

    pub fn events(&self) -> Vec<CalendarEvent> {
        let txn = self.doc.transact();
        let mut events = Vec::new();
        for value in self.events_ref.iter(&txn) {
            let Out::YMap(map) = value else {
                continue;
            };
            events.push(CalendarEvent {
                id: string_field(&txn, &map, "id"),
                title: string_field(&txn, &map, "title"),
                start: int_field(&txn, &map, "start"),
                end: int_field(&txn, &map, "end"),
                location: string_field(&txn, &map, "location"),
                notes: string_field(&txn, &map, "notes"),
                attendees: list_field(&txn, &map, "attendees"),
            });
        }
        events.sort_by_key(|e| (e.start, e.id.clone()));
        events
    }

    pub fn events_between(&self, from: i64, until: i64) -> Vec<CalendarEvent> {
        self.events()
            .into_iter()
            .filter(|e| e.start < until && e.end > from)
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        let txn = self.doc.transact();
        self.events_ref.len(&txn) == 0
    }

    pub fn add_event(&self, title: &str, start: i64, end: i64) -> (String, Vec<u8>) {
        let id = uuid::Uuid::new_v4().to_string();
        let mut txn = self.doc.transact_mut();
        let len = self.events_ref.len(&txn);
        self.events_ref.insert(
            &mut txn,
            len,
            MapPrelim::from([
                ("id", Any::String(id.as_str().into())),
                ("title", Any::String(title.into())),
                ("start", Any::BigInt(start)),
                ("end", Any::BigInt(end.max(start))),
                ("location", Any::String("".into())),
                ("notes", Any::String("".into())),
                ("attendees", Any::Array(Vec::new().into())),
            ]),
        );
        (id, txn.encode_update_v1())
    }

    pub fn set_title(&self, id: &str, title: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.events_ref, id)?;
        if string_field(&txn, &target, "title") == title {
            return None;
        }
        target.insert(&mut txn, "title", Any::String(title.into()));
        Some(txn.encode_update_v1())
    }

    pub fn set_span(&self, id: &str, start: i64, end: i64) -> Option<Vec<u8>> {
        let end = end.max(start);
        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.events_ref, id)?;
        if int_field(&txn, &target, "start") == start && int_field(&txn, &target, "end") == end {
            return None;
        }
        target.insert(&mut txn, "start", Any::BigInt(start));
        target.insert(&mut txn, "end", Any::BigInt(end));
        Some(txn.encode_update_v1())
    }

    pub fn set_location(&self, id: &str, location: &str) -> Option<Vec<u8>> {
        self.set_string(id, "location", location)
    }

    pub fn set_notes(&self, id: &str, notes: &str) -> Option<Vec<u8>> {
        self.set_string(id, "notes", notes)
    }

    fn set_string(&self, id: &str, key: &str, value: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.events_ref, id)?;
        if string_field(&txn, &target, key) == value {
            return None;
        }
        target.insert(&mut txn, key, Any::String(value.into()));
        Some(txn.encode_update_v1())
    }

    pub fn set_attendees(&self, id: &str, attendees: &[String]) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let target = find_map(&txn, &self.events_ref, id)?;
        if list_field(&txn, &target, "attendees") == attendees {
            return None;
        }
        let encoded: Vec<Any> = attendees
            .iter()
            .map(|a| Any::String(a.as_str().into()))
            .collect();
        target.insert(&mut txn, "attendees", Any::Array(encoded.into()));
        Some(txn.encode_update_v1())
    }

    pub fn delete_event(&self, id: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let index = find_index(&txn, &self.events_ref, id)?;
        self.events_ref.remove(&mut txn, index);
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

fn list_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> Vec<String> {
    match map.get(txn, key) {
        Some(Out::Any(Any::Array(items))) => items
            .iter()
            .filter_map(|a| match a {
                Any::String(s) => Some(s.to_string()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn find_map<T: ReadTxn>(txn: &T, events: &ArrayRef, id: &str) -> Option<MapRef> {
    for value in events.iter(txn) {
        if let Out::YMap(map) = value {
            if string_field(txn, &map, "id") == id {
                return Some(map);
            }
        }
    }
    None
}

fn find_index<T: ReadTxn>(txn: &T, events: &ArrayRef, id: &str) -> Option<u32> {
    for (index, value) in events.iter(txn).enumerate() {
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

    const HOUR: i64 = 3_600_000;
    const DAY: i64 = 24 * HOUR;

    #[test]
    fn a_new_calendar_is_empty() {
        let doc = CalendarDoc::new();
        assert!(doc.is_empty());
        assert!(doc.events().is_empty());
    }

    #[test]
    fn events_come_back_sorted_by_start() {
        let doc = CalendarDoc::new();
        doc.add_event("late", 5 * HOUR, 6 * HOUR);
        doc.add_event("early", HOUR, 2 * HOUR);
        doc.add_event("middle", 3 * HOUR, 4 * HOUR);

        let titles: Vec<String> = doc.events().into_iter().map(|e| e.title).collect();
        assert_eq!(titles, ["early", "middle", "late"]);
    }

    #[test]
    fn round_trips_through_snapshot() {
        let doc = CalendarDoc::new();
        let (id, _) = doc.add_event("standup", 9 * HOUR, 9 * HOUR + 30 * 60_000);
        doc.set_attendees(&id, &["AB".to_string(), "CD".to_string()]);

        let restored = CalendarDoc::load(&doc.snapshot());
        assert_eq!(restored.events(), doc.events());
        assert_eq!(restored.events()[0].attendees, ["AB", "CD"]);
    }

    #[test]
    fn empty_calendar_round_trips() {
        let doc = CalendarDoc::new();
        assert!(CalendarDoc::load(&doc.snapshot()).is_empty());
    }

    #[test]
    fn millisecond_timestamps_survive_exactly() {
        let doc = CalendarDoc::new();
        let start = 1_756_000_000_123i64;
        let (_, _) = doc.add_event("precise", start, start + HOUR);

        let restored = CalendarDoc::load(&doc.snapshot());
        assert_eq!(restored.events()[0].start, start);
        assert_eq!(restored.events()[0].end, start + HOUR);
    }

    #[test]
    fn duration_drives_the_chip_height_input() {
        let doc = CalendarDoc::new();
        doc.add_event("half hour", 0, 30 * 60_000);
        doc.add_event("two hours", DAY, DAY + 2 * HOUR);

        let events = doc.events();
        assert_eq!(events[0].duration(), 30 * 60_000);
        assert_eq!(events[1].duration(), 2 * HOUR);
    }

    #[test]
    fn an_end_before_its_start_is_clamped_not_stored() {
        let doc = CalendarDoc::new();
        let (id, _) = doc.add_event("backwards", 5 * HOUR, 2 * HOUR);
        assert_eq!(doc.events()[0].end, 5 * HOUR);
        assert_eq!(doc.events()[0].duration(), 0);

        doc.set_span(&id, 8 * HOUR, HOUR);
        assert_eq!(doc.events()[0].end, 8 * HOUR);
    }

    #[test]
    fn events_between_selects_anything_overlapping_the_window() {
        let doc = CalendarDoc::new();
        doc.add_event("before", 0, HOUR);
        doc.add_event("straddling the start", HOUR, 4 * HOUR);
        doc.add_event("inside", 5 * HOUR, 6 * HOUR);
        doc.add_event("after", 20 * HOUR, 21 * HOUR);

        let titles: Vec<String> = doc
            .events_between(2 * HOUR, 10 * HOUR)
            .into_iter()
            .map(|e| e.title)
            .collect();
        assert_eq!(titles, ["straddling the start", "inside"]);
    }

    #[test]
    fn setting_the_same_value_produces_no_update() {
        let doc = CalendarDoc::new();
        let (id, _) = doc.add_event("fixed", HOUR, 2 * HOUR);
        assert!(doc.set_title(&id, "fixed").is_none());
        assert!(doc.set_span(&id, HOUR, 2 * HOUR).is_none());
        assert!(doc.set_attendees(&id, &[]).is_none());
        assert!(doc.set_title(&id, "moved").is_some());
    }

    #[test]
    fn editing_an_unknown_id_is_a_no_op() {
        let doc = CalendarDoc::new();
        doc.add_event("only", 0, HOUR);
        assert!(doc.set_title("no-such-id", "x").is_none());
        assert!(doc.set_span("no-such-id", 0, HOUR).is_none());
        assert!(doc.delete_event("no-such-id").is_none());
        assert_eq!(doc.events().len(), 1);
    }

    #[test]
    fn delete_removes_only_the_named_event() {
        let doc = CalendarDoc::new();
        doc.add_event("keep", 0, HOUR);
        let (drop, _) = doc.add_event("drop", HOUR, 2 * HOUR);
        doc.delete_event(&drop);

        let titles: Vec<String> = doc.events().into_iter().map(|e| e.title).collect();
        assert_eq!(titles, ["keep"]);
    }

    #[test]
    fn concurrent_adds_converge_without_loss() {
        let a = CalendarDoc::new();
        a.add_event("shared", 0, HOUR);
        let b = CalendarDoc::load(&a.snapshot());

        let from_a = a.add_event("from A", HOUR, 2 * HOUR).1;
        let from_b = b.add_event("from B", 2 * HOUR, 3 * HOUR).1;

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.events(), b.events());
        assert_eq!(a.events().len(), 3);
    }

    #[test]
    fn a_concurrent_move_and_rename_of_one_event_converge() {
        let a = CalendarDoc::new();
        let (id, _) = a.add_event("planning", 9 * HOUR, 10 * HOUR);
        let b = CalendarDoc::load(&a.snapshot());

        let from_a = a.set_span(&id, 14 * HOUR, 15 * HOUR).unwrap();
        let from_b = b.set_title(&id, "planning, moved").unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.events(), b.events());
        assert_eq!(a.events()[0].start, 14 * HOUR);
        assert_eq!(a.events()[0].title, "planning, moved");
    }

    #[test]
    fn three_way_concurrent_adds_converge() {
        let a = CalendarDoc::new();
        a.add_event("base", 0, HOUR);
        let b = CalendarDoc::load(&a.snapshot());
        let c = CalendarDoc::load(&a.snapshot());

        let ua = a.add_event("from A", HOUR, 2 * HOUR).1;
        let ub = b.add_event("from B", 2 * HOUR, 3 * HOUR).1;
        let uc = c.add_event("from C", 3 * HOUR, 4 * HOUR).1;

        for (doc, updates) in [
            (&a, vec![&ub, &uc]),
            (&b, vec![&ua, &uc]),
            (&c, vec![&ua, &ub]),
        ] {
            for u in updates {
                doc.apply_update(u);
            }
        }

        assert_eq!(a.events(), b.events());
        assert_eq!(b.events(), c.events());
        assert_eq!(a.events().len(), 4);
    }

    #[test]
    fn updates_are_idempotent() {
        let a = CalendarDoc::new();
        a.add_event("one", 0, HOUR);
        let b = CalendarDoc::load(&a.snapshot());
        let update = a.add_event("two", HOUR, 2 * HOUR).1;

        b.apply_update(&update);
        b.apply_update(&update);
        b.apply_update(&update);

        assert_eq!(b.events().len(), 2);
    }

    #[test]
    fn diff_since_sends_only_what_is_missing() {
        let a = CalendarDoc::new();
        for hour in 0..6 {
            a.add_event("filler", hour * HOUR, (hour + 1) * HOUR);
        }
        let b = CalendarDoc::load(&a.snapshot());

        a.add_event("new", 7 * HOUR, 8 * HOUR);
        let diff = a.diff_since(&b.state_vector());
        assert!(diff.len() < a.snapshot().len());

        b.apply_update(&diff);
        assert_eq!(a.events(), b.events());
    }

    #[test]
    fn garbage_update_is_rejected_without_panicking() {
        let doc = CalendarDoc::new();
        doc.add_event("intact", 0, HOUR);
        assert!(!doc.apply_update(&[0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(doc.events()[0].title, "intact");
    }

    #[test]
    fn titles_handle_unicode_without_panicking() {
        let doc = CalendarDoc::new();
        let (id, _) = doc.add_event("héllo 🎉", 0, HOUR);
        doc.set_title(&id, "héllo 🎉🎉");
        assert_eq!(doc.events()[0].title, "héllo 🎉🎉");
        doc.set_title(&id, "");
        assert_eq!(doc.events()[0].title, "");
    }
}

#[cfg(test)]
mod location_and_notes {
    use super::*;

    const HOUR: i64 = 3_600_000;

    #[test]
    fn a_new_event_has_an_empty_location_and_notes() {
        let doc = CalendarDoc::new();
        doc.add_event("stand-up", HOUR, 2 * HOUR);
        let event = &doc.events()[0];
        assert_eq!(event.location, "");
        assert_eq!(event.notes, "");
    }

    #[test]
    fn location_and_notes_round_trip_through_a_snapshot() {
        let doc = CalendarDoc::new();
        let (id, _) = doc.add_event("review", HOUR, 2 * HOUR);
        doc.set_location(&id, "Demo room");
        doc.set_notes(&id, "Bring the strip spec\nand the artboard");

        let reloaded = CalendarDoc::load(&doc.snapshot());
        let event = &reloaded.events()[0];
        assert_eq!(event.location, "Demo room");
        assert_eq!(event.notes, "Bring the strip spec\nand the artboard");
        assert_eq!(event.title, "review");
    }

    #[test]
    fn setting_the_same_value_twice_produces_no_update() {
        let doc = CalendarDoc::new();
        let (id, _) = doc.add_event("review", HOUR, 2 * HOUR);
        assert!(doc.set_location(&id, "Demo room").is_some());
        assert!(doc.set_location(&id, "Demo room").is_none());
        assert!(doc.set_notes(&id, "").is_none());
        assert!(doc.set_notes(&id, "x").is_some());
    }

    #[test]
    fn setting_a_field_on_a_missing_event_is_none_not_a_panic() {
        let doc = CalendarDoc::new();
        assert!(doc.set_location("nope", "anywhere").is_none());
        assert!(doc.set_notes("nope", "anything").is_none());
    }

    #[test]
    fn an_event_written_before_these_fields_existed_reads_as_empty() {
        let old = CalendarDoc::new();
        {
            let mut txn = old.doc.transact_mut();
            let len = old.events_ref.len(&txn);
            old.events_ref.insert(
                &mut txn,
                len,
                MapPrelim::from([
                    ("id", Any::String("legacy".into())),
                    ("title", Any::String("from an older build".into())),
                    ("start", Any::BigInt(HOUR)),
                    ("end", Any::BigInt(2 * HOUR)),
                ]),
            );
        }

        let reloaded = CalendarDoc::load(&old.snapshot());
        let event = &reloaded.events()[0];
        assert_eq!(event.title, "from an older build");
        assert_eq!(event.location, "");
        assert_eq!(event.notes, "");
        assert!(event.attendees.is_empty());

        assert!(reloaded.set_location("legacy", "added later").is_some());
        assert_eq!(reloaded.events()[0].location, "added later");
    }

    #[test]
    fn the_two_fields_are_independent() {
        let doc = CalendarDoc::new();
        let (id, _) = doc.add_event("review", HOUR, 2 * HOUR);
        doc.set_location(&id, "Room A");
        doc.set_notes(&id, "note");
        doc.set_location(&id, "Room B");
        let event = &doc.events()[0];
        assert_eq!(event.location, "Room B");
        assert_eq!(event.notes, "note");
    }

    #[test]
    fn two_peers_setting_different_fields_converge() {
        let a = CalendarDoc::new();
        let (id, add) = a.add_event("review", HOUR, 2 * HOUR);
        let b = CalendarDoc::new();
        assert!(b.apply_update(&add));

        let from_a = a.set_location(&id, "Room A").unwrap();
        let from_b = b.set_notes(&id, "b's note").unwrap();
        assert!(b.apply_update(&from_a));
        assert!(a.apply_update(&from_b));

        for doc in [&a, &b] {
            let event = &doc.events()[0];
            assert_eq!(event.location, "Room A");
            assert_eq!(event.notes, "b's note");
        }
    }
}
