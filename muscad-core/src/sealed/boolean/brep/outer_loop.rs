use super::*;

impl<T: Float> BooleanSolver<T> {
    pub fn fill_outer_loop_and_boundary_pts(&mut self, d: &mut BrepData<T>) {
        let mut outer_loop = vec![];
        let f = d.f.borrow();
        let edge_split_pts = &mut d.edge_split_pts;

        f.for_each_edgeref(|e| {
            let k = e.to_key();
            let e = e.borrow();

            if let Some(pts) = edge_split_pts.remove(&k) {
                outer_loop.extend(pts);
            } else {
                outer_loop.push(Rc::clone(self.vertex_pool.add_or_get_vertex(&e.to_p1())))
            }
        });

        d.outer_loop.extend(outer_loop);
        d.boundary_pts = d.outer_loop.iter().map(|p| p.to_key()).collect();
    }
}
