use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{Doc, GetString, ReadTxn, StateVector, Text, TextRef, Transact, Update};

pub struct StickyNoteDoc {
    doc: Doc,
    text_ref: TextRef,
}

impl Default for StickyNoteDoc {
    fn default() -> Self {
        Self::new()
    }
}

impl StickyNoteDoc {
    pub fn new() -> Self {
        let doc = Doc::new();
        let text_ref = doc.get_or_insert_text("content");
        Self { doc, text_ref }
    }

    pub fn load(snapshot: &[u8]) -> Self {
        let doc = Doc::new();
        let text_ref = doc.get_or_insert_text("content");
        {
            let mut txn = doc.transact_mut();
            if let Ok(update) = Update::decode_v1(snapshot) {
                txn.apply_update(update);
            }
        }
        Self { doc, text_ref }
    }

    pub fn get_text(&self) -> String {
        let txn = self.doc.transact();
        self.text_ref.get_string(&txn)
    }

    pub fn is_empty(&self) -> bool {
        let txn = self.doc.transact();
        self.text_ref.len(&txn) == 0
    }

    pub fn set_text(&self, new_text: &str) -> Option<Vec<u8>> {
        let current = self.get_text();
        if current == new_text {
            return None;
        }
        let (prefix, suffix) = common_affixes(&current, new_text);
        let mut txn = self.doc.transact_mut();
        let remove_len = current.len() - prefix - suffix;
        if remove_len > 0 {
            self.text_ref
                .remove_range(&mut txn, prefix as u32, remove_len as u32);
        }
        let inserted = &new_text[prefix..new_text.len() - suffix];
        if !inserted.is_empty() {
            self.text_ref.insert(&mut txn, prefix as u32, inserted);
        }
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

fn common_affixes(old: &str, new: &str) -> (usize, usize) {
    let ob = old.as_bytes();
    let nb = new.as_bytes();
    let max_prefix = ob.len().min(nb.len());

    let mut prefix = 0;
    while prefix < max_prefix && ob[prefix] == nb[prefix] {
        prefix += 1;
    }
    while prefix > 0 && !(old.is_char_boundary(prefix) && new.is_char_boundary(prefix)) {
        prefix -= 1;
    }

    let max_suffix = max_prefix - prefix;
    let mut suffix = 0;
    while suffix < max_suffix && ob[ob.len() - 1 - suffix] == nb[nb.len() - 1 - suffix] {
        suffix += 1;
    }
    while suffix > 0
        && !(old.is_char_boundary(ob.len() - suffix) && new.is_char_boundary(nb.len() - suffix))
    {
        suffix -= 1;
    }

    (prefix, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_snapshot() {
        let doc = StickyNoteDoc::new();
        doc.set_text("hello world");
        let snapshot = doc.snapshot();

        let restored = StickyNoteDoc::load(&snapshot);
        assert_eq!(restored.get_text(), "hello world");
    }

    #[test]
    fn empty_doc_round_trips() {
        let doc = StickyNoteDoc::new();
        let restored = StickyNoteDoc::load(&doc.snapshot());
        assert!(restored.is_empty());
        assert_eq!(restored.get_text(), "");
    }

    #[test]
    fn set_text_returns_none_when_unchanged() {
        let doc = StickyNoteDoc::new();
        doc.set_text("same");
        assert!(doc.set_text("same").is_none());
    }

    #[test]
    fn diff_is_minimal_for_an_append() {
        let doc = StickyNoteDoc::new();
        doc.set_text("the quick brown fox jumps over the lazy dog");
        let update = doc
            .set_text("the quick brown fox jumps over the lazy dogs")
            .unwrap();
        assert!(
            update.len() < 40,
            "append produced a {}-byte update; it should encode one char, not the whole string",
            update.len()
        );
        assert_eq!(
            doc.get_text(),
            "the quick brown fox jumps over the lazy dogs"
        );
    }

    #[test]
    fn diff_is_minimal_for_a_middle_edit() {
        let doc = StickyNoteDoc::new();
        doc.set_text("aaaaaaaaaaXaaaaaaaaaa");
        let update = doc.set_text("aaaaaaaaaaYaaaaaaaaaa").unwrap();
        assert!(
            update.len() < 45,
            "middle edit produced {} bytes",
            update.len()
        );
        assert_eq!(doc.get_text(), "aaaaaaaaaaYaaaaaaaaaa");
    }

    #[test]
    fn handles_unicode_without_panicking() {
        let doc = StickyNoteDoc::new();
        doc.set_text("héllo wörld 🎉");
        doc.set_text("héllo wörld 🎉🎉");
        assert_eq!(doc.get_text(), "héllo wörld 🎉🎉");
        doc.set_text("");
        assert_eq!(doc.get_text(), "");
    }

    #[test]
    fn deletion_works() {
        let doc = StickyNoteDoc::new();
        doc.set_text("abcdef");
        doc.set_text("abef");
        assert_eq!(doc.get_text(), "abef");
        doc.set_text("");
        assert!(doc.is_empty());
    }

    #[test]
    fn concurrent_edits_converge() {
        let a = StickyNoteDoc::new();
        a.set_text("shared");
        let b = StickyNoteDoc::load(&a.snapshot());

        let from_a = a.set_text("shared by A").unwrap();
        let from_b = b.set_text("shared by B").unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(
            a.get_text(),
            b.get_text(),
            "two docs diverged after exchanging concurrent updates"
        );
    }

    #[test]
    fn three_way_concurrent_edits_converge() {
        let a = StickyNoteDoc::new();
        a.set_text("base");
        let b = StickyNoteDoc::load(&a.snapshot());
        let c = StickyNoteDoc::load(&a.snapshot());

        let ua = a.set_text("base A").unwrap();
        let ub = b.set_text("base B").unwrap();
        let uc = c.set_text("base C").unwrap();

        for (doc, updates) in [
            (&a, vec![&ub, &uc]),
            (&b, vec![&ua, &uc]),
            (&c, vec![&ua, &ub]),
        ] {
            for u in updates {
                doc.apply_update(u);
            }
        }

        assert_eq!(a.get_text(), b.get_text());
        assert_eq!(b.get_text(), c.get_text());
    }

    #[test]
    fn updates_are_idempotent() {
        let a = StickyNoteDoc::new();
        a.set_text("hello");
        let b = StickyNoteDoc::load(&a.snapshot());
        let update = a.set_text("hello there").unwrap();

        b.apply_update(&update);
        b.apply_update(&update);
        b.apply_update(&update);

        assert_eq!(b.get_text(), "hello there");
    }

    #[test]
    fn diff_since_sends_only_what_is_missing() {
        let a = StickyNoteDoc::new();
        a.set_text("a long shared starting document that both sides already have");
        let b = StickyNoteDoc::load(&a.snapshot());

        a.set_text("a long shared starting document that both sides already have!");
        let diff = a.diff_since(&b.state_vector());
        assert!(diff.len() < a.snapshot().len());

        b.apply_update(&diff);
        assert_eq!(a.get_text(), b.get_text());
    }

    #[test]
    fn garbage_update_is_rejected_without_panicking() {
        let doc = StickyNoteDoc::new();
        doc.set_text("intact");
        assert!(!doc.apply_update(&[0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(doc.get_text(), "intact");
    }
}
