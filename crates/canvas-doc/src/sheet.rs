use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Any, Array, ArrayRef, Doc, Map, MapPrelim, MapRef, Out, ReadTxn, StateVector, Transact, Update,
};

pub const DEFAULT_ROWS: usize = 7;
pub const DEFAULT_COLS: usize = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetMerge {
    pub row: String,
    pub col: String,
    pub rows: usize,
    pub cols: usize,
}

pub struct SheetDoc {
    doc: Doc,
    rows_ref: ArrayRef,
    cols_ref: ArrayRef,
    cells_ref: MapRef,
    merges_ref: MapRef,
    col_w_ref: MapRef,
    row_h_ref: MapRef,
}

impl Default for SheetDoc {
    fn default() -> Self {
        Self::new()
    }
}

impl SheetDoc {
    pub fn new() -> Self {
        Self::wire(Doc::new())
    }

    pub fn load(snapshot: &[u8]) -> Self {
        let sheet = Self::wire(Doc::new());
        sheet.apply_update(snapshot);
        sheet
    }

    fn wire(doc: Doc) -> Self {
        let rows_ref = doc.get_or_insert_array("rows");
        let cols_ref = doc.get_or_insert_array("cols");
        let cells_ref = doc.get_or_insert_map("cells");
        let merges_ref = doc.get_or_insert_map("merges");
        let col_w_ref = doc.get_or_insert_map("col_w");
        let row_h_ref = doc.get_or_insert_map("row_h");
        Self {
            doc,
            rows_ref,
            cols_ref,
            cells_ref,
            merges_ref,
            col_w_ref,
            row_h_ref,
        }
    }

    pub fn seeded() -> Self {
        let sheet = Self::new();
        for _ in 0..DEFAULT_ROWS {
            sheet.insert_row(usize::MAX);
        }
        for _ in 0..DEFAULT_COLS {
            sheet.insert_col(usize::MAX);
        }
        sheet
    }

    pub fn rows(&self) -> Vec<String> {
        let txn = self.doc.transact();
        ids(&txn, &self.rows_ref)
    }

    pub fn cols(&self) -> Vec<String> {
        let txn = self.doc.transact();
        ids(&txn, &self.cols_ref)
    }

    pub fn is_empty(&self) -> bool {
        let txn = self.doc.transact();
        self.cells_ref.len(&txn) == 0
    }

    pub fn insert_row(&self, index: usize) -> (String, Vec<u8>) {
        self.insert_axis(true, index)
    }

    pub fn insert_col(&self, index: usize) -> (String, Vec<u8>) {
        self.insert_axis(false, index)
    }

    fn insert_axis(&self, is_row: bool, index: usize) -> (String, Vec<u8>) {
        let id = uuid::Uuid::new_v4().to_string();
        let array = if is_row {
            &self.rows_ref
        } else {
            &self.cols_ref
        };
        let mut txn = self.doc.transact_mut();
        let at = index.min(array.len(&txn) as usize) as u32;
        array.insert(
            &mut txn,
            at,
            MapPrelim::from([("id", Any::String(id.as_str().into()))]),
        );
        (id, txn.encode_update_v1())
    }

    pub fn delete_row(&self, id: &str) -> Option<Vec<u8>> {
        self.delete_axis(true, id)
    }

    pub fn delete_col(&self, id: &str) -> Option<Vec<u8>> {
        self.delete_axis(false, id)
    }

    fn delete_axis(&self, is_row: bool, id: &str) -> Option<Vec<u8>> {
        let rows = self.rows();
        let cols = self.cols();
        let axis = if is_row { &rows } else { &cols };
        let index = axis.iter().position(|a| a == id)?;
        let doomed: Vec<String> = self
            .merges()
            .into_iter()
            .filter(|m| {
                let (Some(r), Some(c)) = (
                    rows.iter().position(|x| *x == m.row),
                    cols.iter().position(|x| *x == m.col),
                ) else {
                    return true;
                };
                let (start, span) = if is_row { (r, m.rows) } else { (c, m.cols) };
                (start..start + span).contains(&index)
            })
            .map(|m| cell_key(&m.row, &m.col))
            .collect();
        let partners = if is_row { &cols } else { &rows };

        let mut txn = self.doc.transact_mut();
        if is_row {
            self.rows_ref.remove(&mut txn, index as u32);
        } else {
            self.cols_ref.remove(&mut txn, index as u32);
        }
        for key in doomed {
            self.merges_ref.remove(&mut txn, key.as_str());
        }
        for partner in partners {
            let key = if is_row {
                cell_key(id, partner)
            } else {
                cell_key(partner, id)
            };
            self.cells_ref.remove(&mut txn, key.as_str());
        }
        let sizes = if is_row {
            &self.row_h_ref
        } else {
            &self.col_w_ref
        };
        sizes.remove(&mut txn, id);
        Some(txn.encode_update_v1())
    }

    pub fn value(&self, row: &str, col: &str) -> String {
        let txn = self.doc.transact();
        match self.cells_ref.get(&txn, &cell_key(row, col)) {
            Some(Out::YMap(cell)) => match cell.get(&txn, "value") {
                Some(Out::Any(Any::String(s))) => s.to_string(),
                _ => String::new(),
            },
            _ => String::new(),
        }
    }

    pub fn set_value(&self, row: &str, col: &str, value: &str) -> Option<Vec<u8>> {
        if self.value(row, col) == value {
            return None;
        }
        let key = cell_key(row, col);
        let mut txn = self.doc.transact_mut();
        if value.is_empty() {
            self.cells_ref.remove(&mut txn, key.as_str());
        } else if let Some(Out::YMap(cell)) = self.cells_ref.get(&txn, key.as_str()) {
            cell.insert(&mut txn, "value", Any::String(value.into()));
        } else {
            self.cells_ref.insert(
                &mut txn,
                key,
                MapPrelim::from([("value", Any::String(value.into()))]),
            );
        }
        Some(txn.encode_update_v1())
    }

    pub fn merges(&self) -> Vec<SheetMerge> {
        let txn = self.doc.transact();
        let mut merges = Vec::new();
        for (key, value) in self.merges_ref.iter(&txn) {
            let Some((row, col)) = key.split_once(':') else {
                continue;
            };
            let Out::YMap(span) = value else {
                continue;
            };
            merges.push(SheetMerge {
                row: row.to_string(),
                col: col.to_string(),
                rows: span_field(&txn, &span, "rows"),
                cols: span_field(&txn, &span, "cols"),
            });
        }
        merges.sort_by(|a, b| a.row.cmp(&b.row).then_with(|| a.col.cmp(&b.col)));
        merges
    }

    pub fn merge_at(&self, row: &str, col: &str) -> Option<SheetMerge> {
        self.merges()
            .into_iter()
            .find(|m| m.row == row && m.col == col)
    }

    pub fn is_covered(&self, row: &str, col: &str) -> bool {
        let rows = self.rows();
        let cols = self.cols();
        let (Some(r), Some(c)) = (
            rows.iter().position(|x| x == row),
            cols.iter().position(|x| x == col),
        ) else {
            return false;
        };
        self.merges().into_iter().any(|m| {
            let (Some(mr), Some(mc)) = (
                rows.iter().position(|x| *x == m.row),
                cols.iter().position(|x| *x == m.col),
            ) else {
                return false;
            };
            (mr, mc) != (r, c) && (mr..mr + m.rows).contains(&r) && (mc..mc + m.cols).contains(&c)
        })
    }

    pub fn merge(&self, row: &str, col: &str, rows: usize, cols: usize) -> Option<Vec<u8>> {
        if rows == 0 || cols == 0 || rows * cols < 2 {
            return None;
        }
        let all_rows = self.rows();
        let all_cols = self.cols();
        let r = all_rows.iter().position(|x| x == row)?;
        let c = all_cols.iter().position(|x| x == col)?;
        if r + rows > all_rows.len() || c + cols > all_cols.len() {
            return None;
        }
        for candidate_row in &all_rows[r..r + rows] {
            for candidate_col in &all_cols[c..c + cols] {
                if self.is_covered(candidate_row, candidate_col)
                    || self.merge_at(candidate_row, candidate_col).is_some()
                {
                    return None;
                }
            }
        }
        let mut txn = self.doc.transact_mut();
        self.merges_ref.insert(
            &mut txn,
            cell_key(row, col),
            MapPrelim::from([
                ("rows", Any::BigInt(rows as i64)),
                ("cols", Any::BigInt(cols as i64)),
            ]),
        );
        Some(txn.encode_update_v1())
    }

    pub fn unmerge(&self, row: &str, col: &str) -> Option<Vec<u8>> {
        let key = cell_key(row, col);
        let mut txn = self.doc.transact_mut();
        self.merges_ref.get(&txn, key.as_str())?;
        self.merges_ref.remove(&mut txn, key.as_str());
        Some(txn.encode_update_v1())
    }

    pub fn col_width(&self, col: &str) -> Option<u32> {
        let txn = self.doc.transact();
        size_field(&txn, &self.col_w_ref, col)
    }

    pub fn row_height(&self, row: &str) -> Option<u32> {
        let txn = self.doc.transact();
        size_field(&txn, &self.row_h_ref, row)
    }

    pub fn set_col_width(&self, col: &str, width: u32) -> Option<Vec<u8>> {
        self.set_size(false, col, width)
    }

    pub fn set_row_height(&self, row: &str, height: u32) -> Option<Vec<u8>> {
        self.set_size(true, row, height)
    }

    fn set_size(&self, is_row: bool, id: &str, size: u32) -> Option<Vec<u8>> {
        let current = if is_row {
            self.row_height(id)
        } else {
            self.col_width(id)
        };
        if current == Some(size) {
            return None;
        }
        let map = if is_row {
            &self.row_h_ref
        } else {
            &self.col_w_ref
        };
        let mut txn = self.doc.transact_mut();
        map.insert(&mut txn, id.to_string(), Any::BigInt(size as i64));
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

pub fn cell_key(row: &str, col: &str) -> String {
    format!("{row}:{col}")
}

pub fn column_label(index: usize) -> String {
    let mut label = String::new();
    let mut n = index;
    loop {
        label.insert(0, (b'A' + (n % 26) as u8) as char);
        if n < 26 {
            return label;
        }
        n = n / 26 - 1;
    }
}

fn ids<T: ReadTxn>(txn: &T, array: &ArrayRef) -> Vec<String> {
    array
        .iter(txn)
        .filter_map(|value| {
            let Out::YMap(map) = value else {
                return None;
            };
            match map.get(txn, "id") {
                Some(Out::Any(Any::String(s))) => Some(s.to_string()),
                _ => None,
            }
        })
        .collect()
}

fn span_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> usize {
    match map.get(txn, key) {
        Some(Out::Any(Any::BigInt(n))) if n > 0 => n as usize,
        Some(Out::Any(Any::Number(n))) if n > 0.0 => n as usize,
        _ => 1,
    }
}

fn size_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> Option<u32> {
    match map.get(txn, key) {
        Some(Out::Any(Any::BigInt(n))) if n > 0 => Some(n as u32),
        Some(Out::Any(Any::Number(n))) if n > 0.0 => Some(n as u32),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filled() -> SheetDoc {
        let sheet = SheetDoc::seeded();
        let rows = sheet.rows();
        let cols = sheet.cols();
        for (r, row) in rows.iter().enumerate() {
            for (c, col) in cols.iter().enumerate() {
                sheet.set_value(row, col, &format!("r{r}c{c}"));
            }
        }
        sheet
    }

    #[test]
    fn a_seeded_sheet_is_seven_by_seven_and_empty() {
        let sheet = SheetDoc::seeded();
        assert_eq!(sheet.rows().len(), DEFAULT_ROWS);
        assert_eq!(sheet.cols().len(), DEFAULT_COLS);
        assert!(sheet.is_empty());
    }

    #[test]
    fn a_cell_keeps_its_value_when_a_row_is_inserted_above_it() {
        let sheet = filled();
        let rows = sheet.rows();
        let cols = sheet.cols();
        let (fresh, _) = sheet.insert_row(3);

        let after = sheet.rows();
        assert_eq!(after.len(), DEFAULT_ROWS + 1);
        assert_eq!(after[3], fresh);
        for (r, row) in rows.iter().enumerate() {
            for (c, col) in cols.iter().enumerate() {
                assert_eq!(sheet.value(row, col), format!("r{r}c{c}"));
            }
        }
        assert_eq!(sheet.value(&fresh, &cols[0]), "");
    }

    #[test]
    fn a_cell_keeps_its_value_when_a_column_is_inserted_before_it() {
        let sheet = filled();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.insert_col(0);

        assert_eq!(sheet.cols().len(), DEFAULT_COLS + 1);
        for (r, row) in rows.iter().enumerate() {
            for (c, col) in cols.iter().enumerate() {
                assert_eq!(sheet.value(row, col), format!("r{r}c{c}"));
            }
        }
    }

    #[test]
    fn two_docs_inserting_a_row_at_the_same_index_converge_with_no_loss() {
        let a = SheetDoc::seeded();
        let b = SheetDoc::load(&a.snapshot());

        let (row_a, from_a) = a.insert_row(3);
        let (row_b, from_b) = b.insert_row(3);
        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.rows(), b.rows());
        assert_eq!(a.rows().len(), DEFAULT_ROWS + 2);
        assert!(a.rows().contains(&row_a));
        assert!(a.rows().contains(&row_b));
    }

    #[test]
    fn two_docs_typing_into_different_cells_converge() {
        let a = SheetDoc::seeded();
        let b = SheetDoc::load(&a.snapshot());
        let rows = a.rows();
        let cols = a.cols();

        let from_a = a.set_value(&rows[0], &cols[0], "mine").unwrap();
        let from_b = b.set_value(&rows[1], &cols[1], "theirs").unwrap();
        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.value(&rows[0], &cols[0]), "mine");
        assert_eq!(a.value(&rows[1], &cols[1]), "theirs");
        assert_eq!(b.value(&rows[0], &cols[0]), "mine");
        assert_eq!(b.value(&rows[1], &cols[1]), "theirs");
    }

    #[test]
    fn a_merge_survives_a_reload() {
        let sheet = filled();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.merge(&rows[1], &cols[1], 2, 3).unwrap();
        sheet.set_col_width(&cols[0], 140);
        sheet.set_row_height(&rows[0], 44);

        let restored = SheetDoc::load(&sheet.snapshot());
        assert_eq!(restored.merges(), sheet.merges());
        assert_eq!(
            restored.merge_at(&rows[1], &cols[1]),
            Some(SheetMerge {
                row: rows[1].clone(),
                col: cols[1].clone(),
                rows: 2,
                cols: 3,
            })
        );
        assert_eq!(restored.col_width(&cols[0]), Some(140));
        assert_eq!(restored.row_height(&rows[0]), Some(44));
        assert_eq!(restored.value(&rows[3], &cols[3]), "r3c3");
    }

    #[test]
    fn a_covered_cell_keeps_its_value_and_reports_as_covered() {
        let sheet = filled();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.merge(&rows[1], &cols[1], 2, 2).unwrap();

        assert!(!sheet.is_covered(&rows[1], &cols[1]));
        assert!(sheet.is_covered(&rows[1], &cols[2]));
        assert!(sheet.is_covered(&rows[2], &cols[1]));
        assert!(sheet.is_covered(&rows[2], &cols[2]));
        assert!(!sheet.is_covered(&rows[3], &cols[1]));
        assert_eq!(sheet.value(&rows[2], &cols[2]), "r2c2");
    }

    #[test]
    fn merges_refuse_to_overlap_or_run_off_the_sheet() {
        let sheet = SheetDoc::seeded();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.merge(&rows[1], &cols[1], 2, 2).unwrap();

        assert!(sheet.merge(&rows[2], &cols[2], 2, 2).is_none());
        assert!(sheet.merge(&rows[1], &cols[1], 1, 2).is_none());
        assert!(sheet.merge(&rows[5], &cols[5], 4, 1).is_none());
        assert!(sheet.merge(&rows[0], &cols[0], 1, 1).is_none());
        assert_eq!(sheet.merges().len(), 1);
    }

    #[test]
    fn unmerge_frees_the_range_again() {
        let sheet = SheetDoc::seeded();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.merge(&rows[1], &cols[1], 2, 2).unwrap();
        assert!(sheet.unmerge(&rows[1], &cols[1]).is_some());
        assert!(sheet.unmerge(&rows[1], &cols[1]).is_none());
        assert!(!sheet.is_covered(&rows[2], &cols[2]));
        assert!(sheet.merge(&rows[2], &cols[2], 2, 2).is_some());
    }

    #[test]
    fn deleting_a_row_does_not_orphan_a_merge_anchored_on_it() {
        let sheet = filled();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.merge(&rows[1], &cols[1], 2, 2).unwrap();
        sheet.delete_row(&rows[1]).unwrap();

        assert!(sheet.merges().is_empty());
        assert!(!sheet.is_covered(&sheet.rows()[1], &cols[2]));
        assert_eq!(sheet.rows().len(), DEFAULT_ROWS - 1);
    }

    #[test]
    fn deleting_a_row_a_merge_only_spans_also_drops_that_merge() {
        let sheet = filled();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.merge(&rows[1], &cols[1], 3, 1).unwrap();
        sheet.delete_row(&rows[3]).unwrap();
        assert!(sheet.merges().is_empty());

        sheet.merge(&sheet.rows()[0], &cols[0], 2, 1).unwrap();
        sheet.delete_row(&sheet.rows()[5]).unwrap();
        assert_eq!(sheet.merges().len(), 1);
    }

    #[test]
    fn deleting_a_row_clears_its_cells_and_leaves_the_rest() {
        let sheet = filled();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.set_col_width(&cols[2], 120);
        sheet.delete_col(&cols[2]).unwrap();

        assert_eq!(sheet.cols().len(), DEFAULT_COLS - 1);
        assert_eq!(sheet.col_width(&cols[2]), None);
        assert_eq!(sheet.value(&rows[0], &cols[2]), "");
        assert_eq!(sheet.value(&rows[0], &cols[3]), "r0c3");
    }

    #[test]
    fn deleting_an_unknown_row_changes_nothing() {
        let sheet = SheetDoc::seeded();
        assert!(sheet.delete_row("nope").is_none());
        assert!(sheet.delete_col("nope").is_none());
        assert_eq!(sheet.rows().len(), DEFAULT_ROWS);
    }

    #[test]
    fn clearing_a_cell_removes_it_rather_than_storing_an_empty_string() {
        let sheet = SheetDoc::seeded();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.set_value(&rows[0], &cols[0], "x");
        assert!(!sheet.is_empty());
        sheet.set_value(&rows[0], &cols[0], "");
        assert!(sheet.is_empty());
        assert!(sheet.set_value(&rows[0], &cols[0], "").is_none());
    }

    #[test]
    fn setting_the_same_value_twice_produces_no_update() {
        let sheet = SheetDoc::seeded();
        let rows = sheet.rows();
        let cols = sheet.cols();
        assert!(sheet.set_value(&rows[0], &cols[0], "same").is_some());
        assert!(sheet.set_value(&rows[0], &cols[0], "same").is_none());
        assert!(sheet.set_col_width(&cols[0], 90).is_some());
        assert!(sheet.set_col_width(&cols[0], 90).is_none());
    }

    #[test]
    fn column_labels_run_past_z() {
        assert_eq!(column_label(0), "A");
        assert_eq!(column_label(6), "G");
        assert_eq!(column_label(25), "Z");
        assert_eq!(column_label(26), "AA");
        assert_eq!(column_label(27), "AB");
        assert_eq!(column_label(51), "AZ");
        assert_eq!(column_label(52), "BA");
    }

    #[test]
    fn concurrent_edits_to_the_same_cell_converge() {
        let a = SheetDoc::seeded();
        let b = SheetDoc::load(&a.snapshot());
        let rows = a.rows();
        let cols = a.cols();

        let from_a = a.set_value(&rows[0], &cols[0], "from A").unwrap();
        let from_b = b.set_value(&rows[0], &cols[0], "from B").unwrap();
        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.value(&rows[0], &cols[0]), b.value(&rows[0], &cols[0]));
    }

    #[test]
    fn one_side_deleting_a_row_while_the_other_types_in_it_converges() {
        let a = SheetDoc::seeded();
        let b = SheetDoc::load(&a.snapshot());
        let rows = a.rows();
        let cols = a.cols();

        let from_a = a.delete_row(&rows[2]).unwrap();
        let from_b = b.set_value(&rows[2], &cols[0], "typed").unwrap();
        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.rows(), b.rows());
        assert_eq!(a.rows().len(), DEFAULT_ROWS - 1);
        assert_eq!(a.value(&rows[2], &cols[0]), b.value(&rows[2], &cols[0]));
    }

    #[test]
    fn updates_are_idempotent() {
        let a = SheetDoc::seeded();
        let b = SheetDoc::load(&a.snapshot());
        let rows = a.rows();
        let cols = a.cols();
        let update = a.set_value(&rows[0], &cols[0], "once").unwrap();

        b.apply_update(&update);
        b.apply_update(&update);
        assert_eq!(b.value(&rows[0], &cols[0]), "once");
    }

    #[test]
    fn diff_since_sends_only_what_is_missing() {
        let a = filled();
        let b = SheetDoc::load(&a.snapshot());
        let rows = a.rows();
        let cols = a.cols();

        a.set_value(&rows[0], &cols[0], "changed");
        let diff = a.diff_since(&b.state_vector());
        assert!(diff.len() < a.snapshot().len());

        b.apply_update(&diff);
        assert_eq!(b.value(&rows[0], &cols[0]), "changed");
    }

    #[test]
    fn garbage_update_is_rejected_without_panicking() {
        let sheet = SheetDoc::seeded();
        let rows = sheet.rows();
        let cols = sheet.cols();
        sheet.set_value(&rows[0], &cols[0], "intact");
        assert!(!sheet.apply_update(&[0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(sheet.value(&rows[0], &cols[0]), "intact");
        assert_eq!(sheet.rows().len(), DEFAULT_ROWS);
    }
}
