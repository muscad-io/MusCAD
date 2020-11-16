use super::*;

fn _find_ij<'a, T: Float>(
    _hooks: &'a mut DebugHooks<T>,
    l1: &BrepLoop<T>,
    l2: &BrepLoop<T>,
) -> Option<(usize, usize)> {
    let n = l1.len();
    let m = l2.len();
    for i in 0..n {
        for j in 0..m {
            let segment = Segment::from_point_point(l1[i].to_position(), l2[j].to_position());
            if !_intersects(&segment, l1) && !_intersects(&segment, l2) {
                emit_hook!(_hooks, add_inner_loop, &segment, l1, l2);

                return Some((i, j));
            }
        }
    }

    None
}

fn _intersects<T: Float>(seg: &Segment<T>, pts: &BrepLoop<T>) -> bool {
    let n = pts.len();
    for i in 0..n {
        let edge = Segment::from_point_point(pts[i].to_position(), pts[(i + 1) % n].to_position());

        if let Some(i) = segment::intersect_line_line(&edge, &seg) {
            let (on_seg, on_pt1, on_pt2) = segment::on_segment(&seg, &i);
            let (on_e, on_v1, on_v2) = segment::on_segment(&edge, &i);
            if on_pt1 || on_pt2 || on_v1 || on_v2 {
                continue;
            }
            if on_seg && on_e {
                return true;
            }
        } else {
            panic!("no intersection")
        }
    }

    false
}

impl<T: Float> BooleanSolver<T> {
    pub fn _add_implicit_loop(
        &mut self,
        outer_l: &BrepLoop<T>,
        mut inner_l: BrepLoop<T>,
    ) -> Option<(BrepLoop<T>, BrepLoop<T>)> {
        let inner_copy = inner_l.iter().rev().cloned().collect();

        //TODO: refine; very inefficient
        if let Some((i, j)) = _find_ij(&mut self._hooks, outer_l, &inner_l) {
            let n_outer = outer_l.len();
            let n_inner = inner_l.len();
            let mut res = Vec::with_capacity(n_outer + n_inner + 2);

            for k in 0..n_outer {
                if k == i {
                    let mut inner_head = inner_l.split_off(j);
                    let head = Rc::clone(&inner_head[0]);
                    res.push(Rc::clone(&outer_l[k]));
                    res.extend(inner_head.drain(..));
                    res.extend(inner_l.drain(..));
                    res.push(head);
                    res.push(Rc::clone(&outer_l[k]));
                } else {
                    res.push(Rc::clone(&outer_l[k]));
                }
            }

            Some((res, inner_copy))
        } else {
            None
        }
    }
}
