use super::*;
use mark_functions::*;

mod mark_functions;

pub fn mark_faces<T: Float>(
    mode: &BooleanMode,
    faces: &mut Vec<FaceData<T>>,
    group_ids: &Vec<usize>,
    face_groups: &Map<usize, (Vec<usize>, Classification)>,
    face_from_a: bool,
) {
    let mark = get_mark_function(mode);

    for i in 0..faces.len() {
        let group_id = group_ids[i];
        let c = face_groups.get(&group_id).unwrap().1;

        mark(&mut faces[i], &c, face_from_a);
    }
}

pub fn generate_brep<T: Float>(faces_a: &Vec<FaceData<T>>, faces_b: &Vec<FaceData<T>>) -> Model<T> {
    let mut m = Model::new();

    for f in faces_a.iter().chain(faces_b.iter()) {
        if f.skip {
            continue;
        }

        let pts: Vec<_> = if f.rev {
            f.l.iter().rev().map(to_v3d).collect()
        } else {
            f.l.iter().map(to_v3d).collect()
        };

        m.add_face_unchecked(&pts);
    }

    m
}

fn to_v3d<T: Float>(v: &VertexRef<T>) -> [T; 3] {
    v.to_position()
}
