use super::*;

mod innerloop;
mod split;

impl<T: Float> BooleanSolver<T> {
    pub fn generate_new_face(&mut self, d: &mut BrepData<T>) {
        if d.mode.is_unchanged() {
            return;
        }

        let mut split_edges = vec![];
        let mut loop_edges = vec![];
        let mut dangling_edges = vec![];

        let mut add_dangling = |pts| dangling_edges.push(pts);

        let outer = &d.outer_loop;
        let face_dir = d.f.borrow().to_normal();

        for g in d.segment_groups.drain(..) {
            match g.mode {
                SegmentGroupMode::SplitFace => add_split(&mut split_edges, outer, g.pts),
                SegmentGroupMode::InnerLoop => add_loop(&mut loop_edges, &face_dir, g.pts),
                _ => add_dangling(g.pts),
            }
        }

        emit_hook!(self._hooks, new_face1, &d);

        self.split_face(d, split_edges);

        if d.output_loops.is_empty() {
            d.output_loops.push(d.outer_loop.to_vec());
        }

        let mut new_loops = vec![];
        'a: for inner in loop_edges.into_iter() {
            for outer in d.output_loops.iter_mut() {
                if let Some((l1, l2)) = self._add_implicit_loop(&outer, inner.clone()) {
                    outer.drain(..);
                    new_loops.push(l1);
                    new_loops.push(l2);
                    break 'a;
                }
            }
        }

        for l in d.output_loops.drain(..) {
            if !l.is_empty() {
                new_loops.push(l);
            }
        }

        d.output_loops = new_loops;

        emit_hook!(self._hooks, new_face3, &d);

        if !dangling_edges.is_empty() {
            dbg!("not impl");
        }
    }
}

fn add_split<T: Float>(
    store: &mut Vec<(usize, usize, BrepLoop<T>)>,
    outer: &BrepLoop<T>,
    mut pts: BrepLoop<T>,
) {
    let i1 = outer.iter().position(|x| Rc::ptr_eq(x, &pts[0])).unwrap();
    let i2 = outer
        .iter()
        .position(|x| Rc::ptr_eq(x, &pts[pts.len() - 1]))
        .unwrap();

    if i1 > i2 {
        pts.reverse();
        store.push((i2, i1, pts));
    } else {
        store.push((i1, i2, pts));
    }
}

fn add_loop<T: Float>(store: &mut Vec<BrepLoop<T>>, face_dir: &[T; 3], mut pts: BrepLoop<T>) {
    let inner_loop_dir =
        plane::points_iter_to_plane(pts.iter().map(|v| v.as_position())).to_normal();

    if vector3d::is_same_direction(face_dir, &inner_loop_dir) {
        pts.reverse();
    }

    store.push(pts);
}
