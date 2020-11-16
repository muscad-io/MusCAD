use super::*;

#[derive(Debug)]
pub struct IntersectionResult<T> {
    pub edge_pt_map: Map<E<T>, Set<V<T>>>,
    pub face_face_pt_map: Map<F<T>, Map<F<T>, Set<V<T>>>>,
}

impl<T: Float> IntersectionResult<T> {
    pub fn new() -> Self {
        Self {
            edge_pt_map: new_map(),
            face_face_pt_map: new_map(),
        }
    }

    pub fn save(
        &mut self,
        vertex_pool: &mut VertexPool<T>,
        pt: [T; 3],
        e: &EdgeRef<T>,
        target_face: &FaceRef<T>,
        source_face: &FaceRef<T>,
    ) {
        let vert = Rc::clone(vertex_pool.add_or_get_vertex(&pt));

        self.record_ep(e, &vert);
        self.record_ffp(target_face, source_face, &vert);
        self.record_ffp(&source_face, target_face, &vert);
    }

    fn record_ep(&mut self, e: &EdgeRef<T>, v: &VertexRef<T>) {
        self.edge_pt_map
            .entry(e.to_key())
            .or_insert(new_set())
            .insert(v.to_key());
    }

    fn record_ffp(&mut self, f1: &FaceRef<T>, f2: &FaceRef<T>, v: &VertexRef<T>) {
        self.face_face_pt_map
            .entry(f1.to_key())
            .or_insert(new_map())
            .entry(f2.to_key())
            .or_insert(new_set())
            .insert(v.to_key());
    }
}
