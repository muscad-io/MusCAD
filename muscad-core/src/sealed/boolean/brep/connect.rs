use super::*;

impl<T: Float> BooleanSolver<T> {
    pub fn connect_and_group_segments(&mut self, d: &mut BrepData<T>) {
        let mut segments = new_map();

        segments.extend(d.segments.drain());

        let mut endpoints = new_set();

        for (v, set) in segments.iter() {
            if set.len() == 1 {
                endpoints.insert(v.clone());
            }
        }

        for endpoint in endpoints {
            let start = endpoint.to_rc();

            if let Some(mut cur) = _pop_connected(&mut segments, &endpoint) {
                let mut connected = vec![start, Rc::clone(&cur)];

                while let Some(v_next) = _pop_connected(&mut segments, &cur.to_key()) {
                    connected.push(Rc::clone(&v_next));

                    cur = v_next;
                }

                d.segment_groups
                    .push(SegmentGroup::new_split_face(connected));
            }
        }

        'outer: while let Some(start) = _get_pt(&segments) {
            let mut connected = vec![];

            let mut cur = Rc::clone(&start);

            while let Some(v_next) = _pop_connected(&mut segments, &cur.to_key()) {
                connected.push(Rc::clone(&v_next));

                cur = v_next;

                if Rc::ptr_eq(&start, &cur) {
                    d.segment_groups
                        .push(SegmentGroup::new_inner_loop(connected));

                    continue 'outer;
                }
            }

            dbg!("not impl");
        }
    }
}

fn _remove_segment<T>(segments: &mut Map<V<T>, Set<V<T>>>, v1: &V<T>, v2: &V<T>) {
    if let Some(set) = segments.get_mut(v1) {
        set.remove(v2);

        if set.is_empty() {
            segments.remove(v1);
        }
    }
}

fn _pop_connected<T>(segments: &mut Map<V<T>, Set<V<T>>>, v1: &V<T>) -> Option<VertexRef<T>> {
    if let Some(set) = segments.get_mut(v1) {
        let v2 = set.iter().next().unwrap().clone();

        _remove_segment(segments, v1, &v2);
        _remove_segment(segments, &v2, v1);

        Some(v2.to_rc())
    } else {
        None
    }
}

fn _get_pt<T: Clone>(segments: &Map<V<T>, Set<V<T>>>) -> Option<VertexRef<T>> {
    if segments.is_empty() {
        None
    } else {
        Some(
            segments
                .keys()
                .take(1)
                .cloned()
                .collect::<Vec<_>>()
                .pop()
                .unwrap()
                .to_rc(),
        )
    }
}
