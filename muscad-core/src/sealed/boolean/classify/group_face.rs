use super::*;

pub fn fill_group_data(map: &mut Map<usize, (Vec<usize>, Classification)>, ids: &Vec<usize>) {
    for (face_i, group_id) in ids.iter().enumerate() {
        map.entry(*group_id)
            .or_insert((vec![], Classification::Unknown))
            .0
            .push(face_i);
    }
}

pub fn group_connected<T: Float>(
    faces: &Vec<FaceData<T>>,
    map: &EdgeFaceMap<T>,
    shared_edges: &Set<VV<T>>,
) -> Vec<usize> {
    let n = faces.len();
    let mut group_ids = Vec::with_capacity(n);
    group_ids.resize_with(n, || 0);

    connected_group_in_undirected_graph(&mut group_ids, faces, map, shared_edges);

    group_ids
}

fn connected_group_in_undirected_graph<T: Float>(
    group_ids: &mut Vec<usize>,
    faces: &Vec<FaceData<T>>,
    map: &EdgeFaceMap<T>,
    shared_edges: &Set<VV<T>>,
) {
    // Simple Depth First Search. O(V+E)

    let mut id = 0;
    let mut stack = vec![];

    for i in 0..faces.len() {
        if group_ids[i] != 0 {
            continue;
        }

        id += 1;
        stack.push(i);

        while let Some(idx) = stack.pop() {
            if group_ids[idx] != 0 {
                continue;
            }
            group_ids[idx] = id;

            for e in faces[idx].edges_iter() {
                let e = (e.0.to_key(), e.1.to_key());
                let e_rev = reverse_pair(&e);

                if shared_edges.contains(&e) {
                    continue;
                }

                for j in map.get(&e).unwrap() {
                    stack.push(*j);
                }

                if let Some(indices) = map.get(&e_rev) {
                    for j in indices {
                        stack.push(*j);
                    }
                }
            }
        }
    }
}
