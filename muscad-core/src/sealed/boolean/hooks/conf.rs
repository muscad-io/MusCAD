use super::*;

def_hooks!(
    intersect_face_edge([T; 3], &EdgeRef<T>, &FaceRef<T>, &FaceRef<T>),
    add_inner_loop(&Segment<T>, &BrepLoop<T>, &BrepLoop<T>),
    new_face1(&BrepData<T>),
    new_face2(&Vec<(usize, usize, Vec<String>)>, &Vec<Vec<String>>),
    new_face3(&BrepData<T>),
    classify_group1(T, T, T, T),
    classify_group2((VertexRef<T>, VertexRef<T>)),
    classify_group3(Vec<BrepLoop<T>>, Classification),
);
