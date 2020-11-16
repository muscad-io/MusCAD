use super::*;

impl<T: Float> BooleanSolver<T> {
    pub fn fill_segments(&mut self, d: &mut BrepData<T>) {
        let i = &self.intersections;
        let k = d.f.to_key();
        let face_pts_map = i.face_face_pt_map.get(&k).unwrap();

        for (_, pts) in face_pts_map.iter() {
            for segment in _sort_pts_along_ff_intersection_line(pts).windows(2) {
                d.record_segment(&segment[0], &segment[1]);
            }
        }
    }

    pub fn fill_edge_split_pts(&mut self, d: &mut BrepData<T>) {
        let i = &self.intersections;
        let edge_split_pts = &mut d.edge_split_pts;
        let f = d.f.borrow();

        f.for_each_edgeref(|e| {
            let k = e.to_key();

            if let Some(set) = i.edge_pt_map.get(&k) {
                let res = _sort_pts_along_edge(&e, set);

                edge_split_pts.insert(k, res);
            }
        });
    }
}

pub fn _sort_pts_along_ff_intersection_line<T: Float>(pts: &Set<V<T>>) -> Vec<VertexRef<T>> {
    if pts.len() == 2 {
        // no need to sort
        pts.iter().map(|k| k.to_rc()).collect()
    } else {
        // first get line direction. then sort pts using dot product.
        // Non-planar Face-Face Intersection produces a Line.
        // Either positive or negative direction of the resulting line can be used.
        // Intersection points are ordered using dot product.
        // points:   v1 --- v2 --- v3 --- v4
        //           ^      ^      ^      ^
        //           |      |      |      |
        // segments: x -s1- x -s2- x -s3- x
        let mut v: Vec<_> = pts.iter().map(|k| k.to_rc()).collect();
        let dir = vector3d::sub(v[0].as_ref(), v[1].as_ref());

        v.sort_by(|v1, v2| {
            let d1 = vector3d::dot(&dir, v1.as_ref());
            let d2 = vector3d::dot(&dir, v2.as_ref());

            d1.eps_partial_cmp(&d2).unwrap()
        });

        v
    }
}

pub fn _sort_pts_along_edge<T: Float>(e: &EdgeRef<T>, pts: &Set<V<T>>) -> Vec<VertexRef<T>> {
    // points:   e.v1 --- pt1 --- pt2 --- ... --- e.v2
    let mut rtnval = vec![e.borrow().to_v1()];

    let pt_start = rtnval[0].to_position();

    for p in pts {
        rtnval.push(p.to_rc());
    }

    if pts.len() != 1 {
        rtnval.sort_by(|v1, v2| {
            let d1 = vector3d::distance_sq(&pt_start, v1.as_ref());
            let d2 = vector3d::distance_sq(&pt_start, v2.as_ref());

            d1.eps_partial_cmp(&d2).unwrap()
        });
    }
    rtnval
}
