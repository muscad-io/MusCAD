use super::*;

#[derive(Debug)]
pub struct BooleanSolver<T: Float> {
    pub mode: BooleanMode,
    pub vertex_pool: VertexPool<T>,
    pub intersections: IntersectionResult<T>,
    pub _hooks: DebugHooks<T>,
}

impl<T: Float> BooleanSolver<T> {
    pub fn new() -> Self {
        Self {
            mode: BooleanMode::All,
            vertex_pool: VertexPool::new(),
            intersections: IntersectionResult::new(),
            _hooks: DebugHooks::new(),
        }
    }

    pub fn generate_intersections(&mut self, model_a: &Model<T>, model_b: &Model<T>) {
        // pair faces with intersection potential
        let face_pairs = self.get_intersecting_face_pairs(model_a, model_b);

        self.traversal_intersect_face_pairs(face_pairs);
    }

    pub fn generate_output_faces(
        &mut self,
        model_a: &Model<T>,
        model_b: &Model<T>,
    ) -> (Vec<FaceData<T>>, Vec<FaceData<T>>) {
        let mut brep_data_a = Vec::with_capacity(model_a.n_faces());
        let mut brep_data_b = Vec::with_capacity(model_b.n_faces());

        model_a.for_each_faceref(|f| {
            self.generate_data_for_face(&mut brep_data_a, Rc::clone(f));
        });

        model_b.for_each_faceref(|f| {
            self.generate_data_for_face(&mut brep_data_b, Rc::clone(f));
        });

        (
            brep_data_a
                .iter_mut()
                .flat_map(|d| d.make_faces(&mut self.vertex_pool))
                .collect::<Vec<_>>(),
            brep_data_b
                .iter_mut()
                .flat_map(|d| d.make_faces(&mut self.vertex_pool))
                .collect::<Vec<_>>(),
        )
    }

    pub fn eval(&mut self, model_a: &Model<T>, model_b: &Model<T>) -> Model<T> {
        // Step 1: Generate all intersections (v-v, v-e, v-f, e-e, e-f, f-f)
        self.generate_intersections(model_a, model_b);

        // Step 2: Generate all face outputs
        let (mut faces_a, mut faces_b) = self.generate_output_faces(model_a, model_b);

        // Step 3: Classification
        let (groups_a, groups_b) = {
            let mut data = generate_data_for_classification(&faces_a, &faces_b);

            let (mut groups_a, mut groups_b) = generate_face_groups(&mut data);

            classify_face_groups(
                &mut self._hooks,
                &mut groups_a,
                &faces_a,
                &data.faces_a_with_shared_edges,
                &data.face_angles,
                true,
            );

            classify_face_groups(
                &mut self._hooks,
                &mut groups_b,
                &faces_b,
                &data.faces_b_with_shared_edges,
                &data.face_angles,
                false,
            );

            (groups_a, groups_b)
        };

        // Step 4: Collect resulting faces
        mark_faces(
            &self.mode,
            &mut faces_a,
            &groups_a.group_ids,
            &groups_a.face_groups,
            true,
        );

        mark_faces(
            &self.mode,
            &mut faces_b,
            &groups_b.group_ids,
            &groups_b.face_groups,
            false,
        );

        let result = generate_brep(&faces_a, &faces_b);

        result
    }
}
