use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Any, Array, ArrayRef, Doc, Map, MapPrelim, MapRef, Out, ReadTxn, StateVector, Transact, Update,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub id: String,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub rotation: f64,
    pub style: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub id: String,
    pub style: String,
    pub points: Vec<Point>,
}

pub struct WhiteboardDoc {
    doc: Doc,
    shapes_ref: MapRef,
    strokes_ref: MapRef,
}

impl Default for WhiteboardDoc {
    fn default() -> Self {
        Self::new()
    }
}

impl WhiteboardDoc {
    pub fn new() -> Self {
        let doc = Doc::new();
        let shapes_ref = doc.get_or_insert_map("shapes");
        let strokes_ref = doc.get_or_insert_map("strokes");
        Self {
            doc,
            shapes_ref,
            strokes_ref,
        }
    }

    pub fn load(snapshot: &[u8]) -> Self {
        let doc = Doc::new();
        let shapes_ref = doc.get_or_insert_map("shapes");
        let strokes_ref = doc.get_or_insert_map("strokes");
        {
            let mut txn = doc.transact_mut();
            if let Ok(update) = Update::decode_v1(snapshot) {
                txn.apply_update(update);
            }
        }
        Self {
            doc,
            shapes_ref,
            strokes_ref,
        }
    }

    pub fn shapes(&self) -> Vec<Shape> {
        let txn = self.doc.transact();
        let mut shapes = Vec::new();
        for (_, value) in self.shapes_ref.iter(&txn) {
            let Out::YMap(map) = value else {
                continue;
            };
            shapes.push(Shape {
                id: string_field(&txn, &map, "id"),
                kind: string_field(&txn, &map, "type"),
                x: float_field(&txn, &map, "x"),
                y: float_field(&txn, &map, "y"),
                w: float_field(&txn, &map, "w"),
                h: float_field(&txn, &map, "h"),
                rotation: float_field(&txn, &map, "rotation"),
                style: string_field(&txn, &map, "style"),
            });
        }
        shapes.sort_by(|a, b| a.id.cmp(&b.id));
        shapes
    }

    pub fn add_shape(
        &self,
        kind: &str,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        style: &str,
    ) -> (String, Vec<u8>) {
        let id = uuid::Uuid::new_v4().to_string();
        let mut txn = self.doc.transact_mut();
        self.shapes_ref.insert(
            &mut txn,
            id.clone(),
            MapPrelim::from([
                ("id", Any::String(id.as_str().into())),
                ("type", Any::String(kind.into())),
                ("x", Any::Number(x)),
                ("y", Any::Number(y)),
                ("w", Any::Number(w)),
                ("h", Any::Number(h)),
                ("rotation", Any::Number(0.0)),
                ("style", Any::String(style.into())),
            ]),
        );
        (id, txn.encode_update_v1())
    }

    pub fn move_shape(&self, id: &str, x: f64, y: f64) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let Some(Out::YMap(shape)) = self.shapes_ref.get(&txn, id) else {
            return None;
        };
        if float_field(&txn, &shape, "x") == x && float_field(&txn, &shape, "y") == y {
            return None;
        }
        shape.insert(&mut txn, "x", Any::Number(x));
        shape.insert(&mut txn, "y", Any::Number(y));
        Some(txn.encode_update_v1())
    }

    pub fn resize_shape(&self, id: &str, w: f64, h: f64) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let Some(Out::YMap(shape)) = self.shapes_ref.get(&txn, id) else {
            return None;
        };
        if float_field(&txn, &shape, "w") == w && float_field(&txn, &shape, "h") == h {
            return None;
        }
        shape.insert(&mut txn, "w", Any::Number(w));
        shape.insert(&mut txn, "h", Any::Number(h));
        Some(txn.encode_update_v1())
    }

    pub fn rotate_shape(&self, id: &str, rotation: f64) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        let Some(Out::YMap(shape)) = self.shapes_ref.get(&txn, id) else {
            return None;
        };
        if float_field(&txn, &shape, "rotation") == rotation {
            return None;
        }
        shape.insert(&mut txn, "rotation", Any::Number(rotation));
        Some(txn.encode_update_v1())
    }

    pub fn delete_shape(&self, id: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        self.shapes_ref.get(&txn, id)?;
        self.shapes_ref.remove(&mut txn, id);
        Some(txn.encode_update_v1())
    }

    pub fn strokes(&self) -> Vec<Stroke> {
        let txn = self.doc.transact();
        let mut strokes = Vec::new();
        for (_, value) in self.strokes_ref.iter(&txn) {
            let Out::YMap(map) = value else {
                continue;
            };
            let points = match map.get(&txn, "points") {
                Some(Out::YArray(array)) => read_points(&txn, &array),
                _ => Vec::new(),
            };
            strokes.push(Stroke {
                id: string_field(&txn, &map, "id"),
                style: string_field(&txn, &map, "style"),
                points,
            });
        }
        strokes.sort_by(|a, b| a.id.cmp(&b.id));
        strokes
    }

    pub fn begin_stroke(&self, style: &str) -> (String, Vec<u8>) {
        let id = uuid::Uuid::new_v4().to_string();
        let mut txn = self.doc.transact_mut();
        self.strokes_ref.insert(
            &mut txn,
            id.clone(),
            MapPrelim::from([
                ("id", Any::String(id.as_str().into())),
                ("style", Any::String(style.into())),
            ]),
        );
        let Some(Out::YMap(stroke)) = self.strokes_ref.get(&txn, id.as_str()) else {
            return (id, txn.encode_update_v1());
        };
        stroke.insert(&mut txn, "points", yrs::ArrayPrelim::default());
        (id, txn.encode_update_v1())
    }

    pub fn extend_stroke(&self, id: &str, points: &[Point]) -> Option<Vec<u8>> {
        if points.is_empty() {
            return None;
        }
        let mut txn = self.doc.transact_mut();
        let Some(Out::YMap(stroke)) = self.strokes_ref.get(&txn, id) else {
            return None;
        };
        let Some(Out::YArray(array)) = stroke.get(&txn, "points") else {
            return None;
        };
        let mut index = array.len(&txn);
        for point in points {
            array.insert(&mut txn, index, Any::Number(point.x));
            array.insert(&mut txn, index + 1, Any::Number(point.y));
            index += 2;
        }
        Some(txn.encode_update_v1())
    }

    pub fn point_count(&self, id: &str) -> usize {
        let txn = self.doc.transact();
        let Some(Out::YMap(stroke)) = self.strokes_ref.get(&txn, id) else {
            return 0;
        };
        match stroke.get(&txn, "points") {
            Some(Out::YArray(array)) => array.len(&txn) as usize / 2,
            _ => 0,
        }
    }

    pub fn delete_stroke(&self, id: &str) -> Option<Vec<u8>> {
        let mut txn = self.doc.transact_mut();
        self.strokes_ref.get(&txn, id)?;
        self.strokes_ref.remove(&mut txn, id);
        Some(txn.encode_update_v1())
    }

    pub fn is_empty(&self) -> bool {
        let txn = self.doc.transact();
        self.shapes_ref.len(&txn) == 0 && self.strokes_ref.len(&txn) == 0
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

fn read_points<T: ReadTxn>(txn: &T, array: &ArrayRef) -> Vec<Point> {
    let mut flat = Vec::new();
    for value in array.iter(txn) {
        if let Out::Any(Any::Number(n)) = value {
            flat.push(n);
        }
    }
    flat.chunks_exact(2)
        .map(|pair| Point {
            x: pair[0],
            y: pair[1],
        })
        .collect()
}

fn string_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> String {
    match map.get(txn, key) {
        Some(Out::Any(Any::String(s))) => s.to_string(),
        _ => String::new(),
    }
}

fn float_field<T: ReadTxn>(txn: &T, map: &MapRef, key: &str) -> f64 {
    match map.get(txn, key) {
        Some(Out::Any(Any::Number(n))) => n,
        Some(Out::Any(Any::BigInt(n))) => n as f64,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pts(from: usize, count: usize) -> Vec<Point> {
        (from..from + count)
            .map(|n| Point {
                x: n as f64,
                y: (n * 2) as f64,
            })
            .collect()
    }

    #[test]
    fn a_new_board_is_empty() {
        let doc = WhiteboardDoc::new();
        assert!(doc.is_empty());
        assert!(doc.shapes().is_empty());
        assert!(doc.strokes().is_empty());
    }

    #[test]
    fn shapes_round_trip_through_snapshot() {
        let doc = WhiteboardDoc::new();
        let (id, _) = doc.add_shape("rect", 10.0, 20.0, 100.0, 50.0, "solid");
        doc.rotate_shape(&id, 1.5);

        let restored = WhiteboardDoc::load(&doc.snapshot());
        assert_eq!(restored.shapes(), doc.shapes());
        let shape = &restored.shapes()[0];
        assert_eq!(shape.kind, "rect");
        assert_eq!(
            (shape.x, shape.y, shape.w, shape.h),
            (10.0, 20.0, 100.0, 50.0)
        );
        assert_eq!(shape.rotation, 1.5);
        assert_eq!(shape.style, "solid");
    }

    #[test]
    fn strokes_round_trip_with_their_points_in_order() {
        let doc = WhiteboardDoc::new();
        let (id, _) = doc.begin_stroke("ink");
        doc.extend_stroke(&id, &pts(0, 3));
        doc.extend_stroke(&id, &pts(3, 3));

        let restored = WhiteboardDoc::load(&doc.snapshot());
        let stroke = &restored.strokes()[0];
        assert_eq!(stroke.style, "ink");
        assert_eq!(stroke.points, pts(0, 6));
    }

    #[test]
    fn empty_board_round_trips() {
        let doc = WhiteboardDoc::new();
        assert!(WhiteboardDoc::load(&doc.snapshot()).is_empty());
    }

    #[test]
    fn appending_1000_points_costs_traffic_proportional_to_the_points_added() {
        let doc = WhiteboardDoc::new();
        let (id, _) = doc.begin_stroke("ink");
        doc.extend_stroke(&id, &pts(0, 1000));
        assert_eq!(doc.point_count(&id), 1000);

        let peer_state = doc.state_vector();
        let one_more = doc.extend_stroke(&id, &pts(1000, 1)).unwrap();
        let ten_more = {
            let before = doc.state_vector();
            doc.extend_stroke(&id, &pts(1001, 10)).unwrap();
            doc.diff_since(&before)
        };

        let whole_canvas = doc.snapshot().len();
        assert!(
            one_more.len() * 20 < whole_canvas,
            "one appended point cost {} bytes against a {whole_canvas}-byte canvas",
            one_more.len()
        );
        assert!(
            ten_more.len() < one_more.len() * 12,
            "ten points cost {} bytes, one cost {}",
            ten_more.len(),
            one_more.len()
        );
        let _ = peer_state;
    }

    #[test]
    fn stroke_traffic_does_not_grow_with_the_canvas() {
        let small = WhiteboardDoc::new();
        let (small_id, _) = small.begin_stroke("ink");
        let small_cost = small.extend_stroke(&small_id, &pts(0, 10)).unwrap().len();

        let big = WhiteboardDoc::new();
        for n in 0..200 {
            big.add_shape("rect", n as f64, n as f64, 10.0, 10.0, "solid");
        }
        for _ in 0..20 {
            let (other, _) = big.begin_stroke("ink");
            big.extend_stroke(&other, &pts(0, 200));
        }
        let (big_id, _) = big.begin_stroke("ink");
        let big_cost = big.extend_stroke(&big_id, &pts(0, 10)).unwrap().len();

        assert!(
            big_cost < small_cost * 3,
            "the same 10 points cost {big_cost} bytes on a busy canvas and {small_cost} on an empty one"
        );
        assert!(big.snapshot().len() > 20 * big_cost);
    }

    #[test]
    fn extending_an_unknown_stroke_is_a_no_op() {
        let doc = WhiteboardDoc::new();
        assert!(doc.extend_stroke("no-such-id", &pts(0, 3)).is_none());
        assert!(doc.delete_stroke("no-such-id").is_none());
        assert_eq!(doc.point_count("no-such-id"), 0);
    }

    #[test]
    fn extending_with_nothing_produces_no_update() {
        let doc = WhiteboardDoc::new();
        let (id, _) = doc.begin_stroke("ink");
        assert!(doc.extend_stroke(&id, &[]).is_none());
    }

    #[test]
    fn touching_an_unknown_shape_is_a_no_op() {
        let doc = WhiteboardDoc::new();
        doc.add_shape("rect", 0.0, 0.0, 1.0, 1.0, "solid");
        assert!(doc.move_shape("no-such-id", 1.0, 1.0).is_none());
        assert!(doc.resize_shape("no-such-id", 1.0, 1.0).is_none());
        assert!(doc.rotate_shape("no-such-id", 1.0).is_none());
        assert!(doc.delete_shape("no-such-id").is_none());
        assert_eq!(doc.shapes().len(), 1);
    }

    #[test]
    fn setting_the_same_geometry_produces_no_update() {
        let doc = WhiteboardDoc::new();
        let (id, _) = doc.add_shape("rect", 3.0, 4.0, 5.0, 6.0, "solid");
        assert!(doc.move_shape(&id, 3.0, 4.0).is_none());
        assert!(doc.resize_shape(&id, 5.0, 6.0).is_none());
        assert!(doc.rotate_shape(&id, 0.0).is_none());
        assert!(doc.move_shape(&id, 3.0, 5.0).is_some());
    }

    #[test]
    fn delete_removes_only_the_named_item() {
        let doc = WhiteboardDoc::new();
        let (keep, _) = doc.add_shape("rect", 0.0, 0.0, 1.0, 1.0, "solid");
        let (drop, _) = doc.add_shape("ellipse", 1.0, 1.0, 2.0, 2.0, "solid");
        doc.delete_shape(&drop);
        assert_eq!(doc.shapes().len(), 1);
        assert_eq!(doc.shapes()[0].id, keep);
    }

    #[test]
    fn two_peers_drawing_at_once_keep_both_strokes_whole() {
        let a = WhiteboardDoc::new();
        let (shared, _) = a.begin_stroke("ink");
        a.extend_stroke(&shared, &pts(0, 5));
        let b = WhiteboardDoc::load(&a.snapshot());

        let (a_stroke, a_begin) = a.begin_stroke("ink");
        let a_points = a.extend_stroke(&a_stroke, &pts(100, 20)).unwrap();
        let (b_stroke, b_begin) = b.begin_stroke("ink");
        let b_points = b.extend_stroke(&b_stroke, &pts(200, 20)).unwrap();

        a.apply_update(&b_begin);
        a.apply_update(&b_points);
        b.apply_update(&a_begin);
        b.apply_update(&a_points);

        assert_eq!(a.strokes(), b.strokes());
        assert_eq!(a.strokes().len(), 3);
        assert_eq!(a.point_count(&a_stroke), 20);
        assert_eq!(a.point_count(&b_stroke), 20);
        assert_eq!(a.point_count(&shared), 5);
    }

    #[test]
    fn two_peers_extending_the_same_stroke_lose_no_points() {
        let a = WhiteboardDoc::new();
        let (id, begin) = a.begin_stroke("ink");
        let b = WhiteboardDoc::load(&a.snapshot());
        let _ = begin;

        let from_a = a.extend_stroke(&id, &pts(0, 10)).unwrap();
        let from_b = b.extend_stroke(&id, &pts(100, 10)).unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.strokes(), b.strokes());
        assert_eq!(a.point_count(&id), 20, "points were lost or duplicated");
    }

    #[test]
    fn concurrent_moves_of_the_same_shape_converge() {
        let a = WhiteboardDoc::new();
        let (id, _) = a.add_shape("rect", 0.0, 0.0, 10.0, 10.0, "solid");
        let b = WhiteboardDoc::load(&a.snapshot());

        let from_a = a.move_shape(&id, 50.0, 50.0).unwrap();
        let from_b = b.resize_shape(&id, 99.0, 99.0).unwrap();

        a.apply_update(&from_b);
        b.apply_update(&from_a);

        assert_eq!(a.shapes(), b.shapes());
        assert_eq!(a.shapes()[0].x, 50.0);
        assert_eq!(a.shapes()[0].w, 99.0);
    }

    #[test]
    fn three_way_concurrent_shapes_converge() {
        let a = WhiteboardDoc::new();
        a.add_shape("rect", 0.0, 0.0, 1.0, 1.0, "solid");
        let b = WhiteboardDoc::load(&a.snapshot());
        let c = WhiteboardDoc::load(&a.snapshot());

        let ua = a.add_shape("rect", 1.0, 1.0, 1.0, 1.0, "solid").1;
        let ub = b.add_shape("ellipse", 2.0, 2.0, 1.0, 1.0, "solid").1;
        let uc = c.add_shape("line", 3.0, 3.0, 1.0, 1.0, "solid").1;

        for (doc, updates) in [
            (&a, vec![&ub, &uc]),
            (&b, vec![&ua, &uc]),
            (&c, vec![&ua, &ub]),
        ] {
            for u in updates {
                doc.apply_update(u);
            }
        }

        assert_eq!(a.shapes(), b.shapes());
        assert_eq!(b.shapes(), c.shapes());
        assert_eq!(a.shapes().len(), 4);
    }

    #[test]
    fn updates_are_idempotent() {
        let a = WhiteboardDoc::new();
        let (id, _) = a.begin_stroke("ink");
        let b = WhiteboardDoc::load(&a.snapshot());
        let update = a.extend_stroke(&id, &pts(0, 5)).unwrap();

        b.apply_update(&update);
        b.apply_update(&update);
        b.apply_update(&update);

        assert_eq!(b.point_count(&id), 5);
    }

    #[test]
    fn garbage_update_is_rejected_without_panicking() {
        let doc = WhiteboardDoc::new();
        doc.add_shape("rect", 0.0, 0.0, 1.0, 1.0, "intact");
        assert!(!doc.apply_update(&[0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(doc.shapes()[0].style, "intact");
    }
}
