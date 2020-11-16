use super::*;

pub fn analyse_face_group<T: Float>(
    _hooks: &mut DebugHooks<T>,
    faces: &Vec<FaceData<T>>,
    group: &Vec<usize>,
    faces_with_shared_edges: &Set<usize>,
    angles: &Map<VV<T>, (T, T)>,
    face_from_a: bool,
) -> Classification {
    for i in group.iter() {
        if faces_with_shared_edges.contains(i) {
            let res = analyse_neighbourhood(_hooks, &faces[*i], angles, face_from_a);

            if res != Classification::Unknown {
                emit_hook!(_hooks, classify_group3, {
                    let group_faces = group.iter().map(|&i| faces[i].l.to_vec()).collect();

                    (group_faces, res)
                });

                return res;
            }
        }
    }

    Classification::Unknown
}

pub fn analyse_neighbourhood<T: Float>(
    _hooks: &mut DebugHooks<T>,
    f: &FaceData<T>,
    angles: &Map<VV<T>, (T, T)>,
    face_from_a: bool,
) -> Classification {
    for e in f.edges_iter() {
        let edge = (e.0.to_key(), e.1.to_key());
        let rev_edge = reverse_pair(&edge);

        match (angles.get(&edge), angles.get(&rev_edge)) {
            (Some(angle), Some(angle_rev)) => {
                let res = _classify(_hooks, angle, angle_rev, face_from_a);

                if res != Classification::Unknown {
                    emit_hook!(_hooks, classify_group2, (edge.0.to_rc(), edge.1.to_rc()));

                    return res;
                }
            }
            _ => continue,
        }
    }

    Classification::Unknown
}

fn _classify<T: Float>(
    _hooks: &mut DebugHooks<T>,
    angle: &(T, T),
    angle_rev: &(T, T),
    face_from_a: bool,
) -> Classification {
    let angle = to_principal_tuple(angle);
    let angle_rev = to_principal_tuple(&flip_angle_tuple(angle_rev));

    let [a1, a2, b1, b2] = if face_from_a {
        [angle.0, angle_rev.0, angle.1, angle_rev.1].to_owned()
    } else {
        [angle.1, angle_rev.1, angle.0, angle_rev.0].to_owned()
    };

    emit_hook!(
        _hooks,
        classify_group1,
        a1.to_owned(),
        a2.to_owned(),
        b1.to_owned(),
        b2.to_owned()
    );

    if a1.eps_eql(&b2) || a2.eps_eql(&b2) {
        return Classification::In;
    }

    if _is_between(b1, a1, a2) {
        Classification::In
    } else {
        Classification::Out
    }
}

fn _is_between<T: Float>(x: T, a1: T, a2: T) -> bool {
    if a1 < a2 {
        a1 < x && x < a2
    } else {
        !(a2 < x && x < a1)
    }
}

fn flip_angle_tuple<T: Float>(t: &(T, T)) -> (T, T) {
    (_flip_angle(&t.0), _flip_angle(&t.1))
}

fn _flip_angle<T: Float>(angle: &T) -> T {
    // mirror along y=x, then flip.
    -T::PI.clone() * &T::half() - angle
}

fn to_principal_tuple<T: Float>(t: &(T, T)) -> (T, T) {
    (_to_principal(&t.0), _to_principal(&t.1))
}

fn _to_principal<T: Float>(angle: &T) -> T {
    if angle.eps_gt(&T::TAU) {
        return angle.clone() - &T::TAU;
    }
    if angle.eps_lt(&T::ZERO) {
        return angle.clone() + &T::TAU;
    }

    angle.clone()
}
