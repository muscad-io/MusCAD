use super::*;
pub use data::*;

mod data;

pub type FacePairs<T> = Vec<FacePair<T>>;
pub type FacePair<T> = (FaceRef<T>, FaceRef<T>);

impl<T: Float> BooleanSolver<T> {
    pub fn get_intersecting_face_pairs(
        &self,
        model_a: &Model<T>,
        model_b: &Model<T>,
    ) -> FacePairs<T> {
        let mut res = vec![];

        model_a.for_each_faceref(|ptr_a| {
            model_b.for_each_faceref(|ptr_b| {
                let fa = ptr_a.borrow();
                let fb = ptr_b.borrow();

                if fa.bbox.intersects(&fb.bbox) {
                    res.push((Rc::clone(ptr_a), Rc::clone(ptr_b)))
                }
            })
        });

        res
    }

    pub fn traversal_intersect_face_pairs(&mut self, pairs: FacePairs<T>) {
        for (ptr_a, ptr_b) in pairs {
            self.intersect_face_face(&ptr_a, &ptr_b);
            self.intersect_face_face(&ptr_b, &ptr_a);
        }
    }

    fn intersect_face_face(&mut self, a: &FaceRef<T>, b: &FaceRef<T>) {
        // TODO: check coplanar
        b.borrow()
            .for_each_edgeref(|e| self.intersect_face_edge(a, &e, b));
    }

    fn intersect_face_edge(
        &mut self,
        ptr_f: &FaceRef<T>,
        ptr_e: &EdgeRef<T>,
        ptr_src_f: &FaceRef<T>,
    ) {
        let e = ptr_e.borrow();
        let f = ptr_f.borrow();

        let pt = segment::intersect_line_plane(&*e, &*f);

        if pt.is_none() {
            return;
        }

        let pt = pt.unwrap();
        let within_segment = segment::on_segment(&*e, &pt).0;
        let within_face = f.contains_point(&pt);

        if !within_segment || !within_face {
            return;
        }

        emit_hook!(
            &mut self._hooks,
            intersect_face_edge,
            pt.clone(),
            ptr_e,
            ptr_f,
            ptr_src_f
        );

        self.intersections
            .save(&mut self.vertex_pool, pt, ptr_e, ptr_f, ptr_src_f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let mut m = Model::new();
        let f1 = m.add_face_unchecked(&[
            [1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [1.0, 1.0, 1.0],
        ]);
        let f2 = m.add_face_unchecked(&[
            [1.52, -0.54, 0.69],
            [1.52, 0.46, 0.69],
            [0.52, 0.46, 0.69],
            [0.52, -0.54, 0.69],
        ]);
        let k1 = f1.to_key();
        let k2 = f2.to_key();

        let mut solver = BooleanSolver::new();

        solver.traversal_intersect_face_pairs(vec![(f1, f2)]);
        let ff_map = solver.intersections.face_face_pt_map;
        let set1 = ff_map.get(&k1).unwrap().get(&k2).unwrap();
        let set2 = ff_map.get(&k2).unwrap().get(&k1).unwrap();

        let p1 = &[1.00, 0.46, 0.69];
        let p2 = &[1.00, -0.54, 0.69];

        for v in set1.iter() {
            let v = &v.as_ref();
            assert!(vector3d::eql(v, p1) || vector3d::eql(v, p2));
        }

        assert_eq!(set1, set2);
        assert_eq!(set1.len(), 2);
    }
}
