use yrs::{Doc, Text, Transact, ReadTxn, StateVector, Update, TextRef, GetString};
use yrs::updates::encoder::Encode;
use yrs::updates::decoder::Decode;

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
        let mut txn = doc.transact_mut();
        if let Ok(update) = Update::decode_v1(snapshot) {
            txn.apply_update(update);
        }
        drop(txn);
        let text_ref = doc.get_or_insert_text("content");
        Self { doc, text_ref }
    }

    pub fn get_text(&self) -> String {
        let txn = self.doc.transact();
        self.text_ref.get_string(&txn)
    }

    pub fn update_text(&self, new_text: &str) {
        let mut txn = self.doc.transact_mut();
        let current_len = self.text_ref.len(&txn);
        if current_len > 0 {
            self.text_ref.remove_range(&mut txn, 0, current_len);
        }
        self.text_ref.insert(&mut txn, 0, new_text);
    }

    pub fn snapshot(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&StateVector::default())
    }

    pub fn state_vector(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.state_vector().encode_v1()
    }
}
