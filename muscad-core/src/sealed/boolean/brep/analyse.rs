use super::*;

impl<T: Float> BooleanSolver<T> {
    pub fn analyse_brep_configuration(&mut self, f: FaceRef<T>) -> BrepData<T> {
        let mut rtnval = BrepData::unchanged(f);

        self.fill_segments(&mut rtnval);
        self.fill_edge_split_pts(&mut rtnval);

        self.fill_outer_loop_and_boundary_pts(&mut rtnval);

        self.connect_and_group_segments(&mut rtnval);

        for g in rtnval.segment_groups.iter() {
            rtnval.mode.record(g.mode as usize);
        }

        rtnval
    }
}
