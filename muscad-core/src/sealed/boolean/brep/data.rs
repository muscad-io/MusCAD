use super::*;

pub type BrepLoop<T> = Vec<VertexRef<T>>;

#[derive(Debug, Clone, Copy)]
pub struct BrepMode(pub usize);

impl BrepMode {
    pub fn is_unchanged(&self) -> bool {
        self.0 == 0
    }

    pub fn unchanged() -> Self {
        Self(0)
    }

    pub fn record(&mut self, v: usize) {
        self.0 |= v;
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum SegmentGroupMode {
    SplitFace = 1 << 1,
    InnerLoop = 1 << 2,
    Other = 1 << 8,
}

#[allow(unused)]
#[derive(Debug)]
pub struct SegmentGroup<T: Float> {
    pub mode: SegmentGroupMode,
    pub pts: Vec<VertexRef<T>>,
}

impl<T: Float> SegmentGroup<T> {
    pub fn new(mode: SegmentGroupMode, pts: Vec<VertexRef<T>>) -> Self {
        Self { mode, pts }
    }

    pub fn new_split_face(pts: Vec<VertexRef<T>>) -> Self {
        Self::new(SegmentGroupMode::SplitFace, pts)
    }

    pub fn new_inner_loop(pts: Vec<VertexRef<T>>) -> Self {
        Self::new(SegmentGroupMode::InnerLoop, pts)
    }
}

#[derive(Debug)]
pub struct BrepData<T: Float> {
    pub f: FaceRef<T>,
    pub mode: BrepMode,
    pub outer_loop: Vec<VertexRef<T>>,
    pub boundary_pts: Set<V<T>>,
    pub edge_split_pts: Map<E<T>, Vec<VertexRef<T>>>,
    pub segments: Map<V<T>, Set<V<T>>>,
    pub segment_groups: Vec<SegmentGroup<T>>,
    pub output_loops: Vec<BrepLoop<T>>,
}

impl<T: Float> BrepData<T> {
    pub fn build(f: FaceRef<T>, mode: BrepMode) -> Self {
        Self {
            f,
            mode,
            outer_loop: vec![],
            boundary_pts: new_set(),
            edge_split_pts: new_map(),
            segments: new_map(),
            segment_groups: vec![],
            output_loops: vec![],
        }
    }

    pub fn unchanged(f: FaceRef<T>) -> Self {
        Self::build(f, BrepMode::unchanged())
    }

    pub fn make_faces(&mut self, vertex_pool: &mut VertexPool<T>) -> Vec<FaceData<T>> {
        let f = Rc::clone(&self.f);
        if self.mode.is_unchanged() {
            let l = f
                .borrow()
                .vertices_iter()
                .map(|v| Rc::clone(vertex_pool.add_or_get_vertex(v.as_ref())))
                .collect();

            vec![FaceData::new(f, l)]
        } else {
            self.output_loops
                .drain(..)
                .map(|l| {
                    l.iter()
                        .map(|v| Rc::clone(vertex_pool.add_or_get_vertex(v.as_ref())))
                        .collect()
                })
                .map(|l| FaceData::new(Rc::clone(&f), l))
                .collect()
        }
    }

    pub fn record_segment(&mut self, v1: &VertexRef<T>, v2: &VertexRef<T>) {
        self.segments
            .entry(v1.to_key())
            .or_insert(new_set())
            .insert(v2.to_key());
        self.segments
            .entry(v2.to_key())
            .or_insert(new_set())
            .insert(v1.to_key());
    }
}
