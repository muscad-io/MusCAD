use super::*;

#[cfg(feature = "serialization")]
mod serialize;

pub type EdgeRef<T> = Rc<RefCell<Edge<T>>>;
pub type EdgeRefWeak<T> = Weak<RefCell<Edge<T>>>;

#[derive(Debug)]
pub struct Edge<T> {
    pub vert: VertexRefWeak<T>,
    pub face: FaceRefWeak<T>,
    pub next: EdgeRefWeak<T>,
    pub prev: EdgeRefWeak<T>,
    pub rev: Option<EdgeRefWeak<T>>,
    id: entity::ID,
}

impl_id!(EDGE_ID, Edge<T>);

impl<T> Edge<T> {
    pub fn new() -> Self {
        Self {
            vert: Weak::new(),
            face: Weak::new(),
            next: Weak::new(),
            prev: Weak::new(),
            rev: None,
            id: Self::_next_id(),
        }
    }

    pub fn to_v1(&self) -> VertexRef<T> {
        self.vert.upgrade().unwrap()
    }

    pub fn to_v2(&self) -> VertexRef<T> {
        self.next.upgrade().unwrap().borrow().to_v1()
    }
}

impl<T: Float> AsSegment<T> for Edge<T> {
    fn to_p1(&self) -> [T; 3] {
        self.to_v1().to_position()
    }

    fn to_p2(&self) -> [T; 3] {
        self.to_v2().to_position()
    }

    fn to_direction(&self) -> [T; 3] {
        let p1 = &self.to_p1();
        let p2 = &self.to_p2();

        v3d!(p2 - p1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_as_segment() {
        let mut m = Model::new();
        let pts = [[10.0, 0.0, 0.0], [10.0, 1.0, 0.0], [10.0, 0.0, 1.0]];
        let ptr = m.add_face_unchecked(&pts);
        let mut f = ptr.borrow_mut();
        f.calc();
        let mut edges = vec![];
        f.for_each_edgeref(|e| edges.push(e));

        let pti = segment::intersect_line_line(&*edges[0].borrow(), &*edges[1].borrow()).unwrap();

        assert_eq!(pti, [10.0, 1.0, 0.0]);
    }
}
