use super::*;
use analyse::*;
pub use data::*;
use face_angle::*;
use group_face::*;
use init_edge_face_map::*;

mod analyse;
mod data;
mod face_angle;
mod group_face;
mod init_edge_face_map;

pub fn classify_face_groups<T: Float>(
    _hooks: &mut DebugHooks<T>,
    result: &mut ClassificationResult,
    faces: &Vec<FaceData<T>>,
    faces_with_shared_edges: &Set<usize>,
    angles: &Map<VV<T>, (T, T)>,
    face_from_a: bool,
) {
    for (_, group) in result.face_groups.iter_mut() {
        group.1 = analyse_face_group(
            _hooks,
            faces,
            &group.0,
            &faces_with_shared_edges,
            &angles,
            face_from_a,
        );
    }
}

pub fn generate_face_groups<T: Float>(
    data: &mut ClassificationData<T>,
) -> (ClassificationResult, ClassificationResult) {
    let ids_a = group_connected(data.faces_a, &data.ef_map_a, &data.shared_edges);
    let ids_b = group_connected(data.faces_b, &data.ef_map_b, &data.shared_edges);

    let mut map_a = new_map();
    let mut map_b = new_map();

    fill_group_data(&mut map_a, &ids_a);
    fill_group_data(&mut map_b, &ids_b);

    (
        ClassificationResult::new(ids_a, map_a),
        ClassificationResult::new(ids_b, map_b),
    )
}

pub fn generate_data_for_classification<'a, T: Float>(
    faces_a: &'a Vec<FaceData<T>>,
    faces_b: &'a Vec<FaceData<T>>,
) -> ClassificationData<'a, T> {
    let mut data = ClassificationData::new(faces_a, faces_b);

    for t in faces_a.iter().enumerate() {
        fill_edge_face_map(&mut data.ef_map_a, t);
    }

    for t in faces_b.iter().enumerate() {
        fill_edge_face_map(&mut data.ef_map_b, t);
    }

    fill_shared_edges(&mut data.shared_edges, &data.ef_map_a, &data.ef_map_b);

    fill_faces_with_shared_edges(
        &mut data.faces_a_with_shared_edges,
        &data.shared_edges,
        &data.ef_map_a,
    );

    fill_faces_with_shared_edges(
        &mut data.faces_b_with_shared_edges,
        &data.shared_edges,
        &data.ef_map_b,
    );

    fill_face_angles(&mut data);

    data
}
