use super::*;

pub fn fill_edge_face_map<T: Float>(
    map: &mut EdgeFaceMap<T>,
    (face_index, f): (usize, &FaceData<T>),
) {
    for e in f.edges_iter() {
        map.entry((e.0.to_key(), e.1.to_key()))
            .or_insert(new_set())
            .insert(face_index);
    }
}

pub fn fill_shared_edges<T: Float>(
    set: &mut Set<VV<T>>,
    map1: &EdgeFaceMap<T>,
    map2: &EdgeFaceMap<T>,
) {
    let mut set_a = new_set();
    let mut set_b = new_set();
    set_a.extend(map1.keys().cloned());
    set_b.extend(map2.keys().cloned());

    set.extend(set_a.intersection(&set_b).cloned());
}

pub fn fill_faces_with_shared_edges<T: Float>(
    v: &mut Set<usize>,
    edges: &Set<VV<T>>,
    map: &EdgeFaceMap<T>,
) {
    let mut set: Set<usize> = new_set();

    for e in edges.iter() {
        set.extend(map.get(e).unwrap());
    }

    v.extend(set.iter().map(|i| *i));
}
