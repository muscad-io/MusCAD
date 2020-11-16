use super::*;

pub fn fill_face_angles<T: Float>(d: &mut ClassificationData<T>) {
    let mut angle_data = new_map();

    for e in d.shared_edges.iter() {
        let i_a = _get_first_face_index(&mut d.ef_map_a, e);
        let i_b = _get_first_face_index(&mut d.ef_map_b, e);

        let project = get_proj_functor(e);

        let angle_a = calc_angle(project, &d.faces_a[i_a]);
        let angle_b = calc_angle(project, &d.faces_b[i_b]);

        angle_data.insert(e.clone(), (angle_a, angle_b));
    }

    d.face_angles.extend(angle_data.drain());
}

fn _get_first_face_index<T: Float>(map: &mut EdgeFaceMap<T>, e: &VV<T>) -> usize {
    let v: Vec<_> = map.get_mut(e).unwrap().drain().take(1).collect();

    v[0]
}

fn get_proj_functor<T: Float>(e: &VV<T>) -> fn(&[T; 3]) -> [T; 2] {
    let direction = vector3d::sub(&e.1.as_ref(), &e.0.as_ref());

    project2d::to_functor(&direction[X], &direction[Y], &direction[Z])
}

fn calc_angle<T: Float>(proj: fn(&[T; 3]) -> [T; 2], f: &FaceData<T>) -> T {
    let [x, y] =
        proj(&plane::points_iter_to_plane(f.l.iter().map(|v| v.to_position())).to_normal());

    y.atan2(x)
}
