use super::*;

pub type EdgeFaceMap<T> = Map<VV<T>, Set<usize>>;
pub type EdgeData<T> = (VertexRef<T>, VertexRef<T>);

#[derive(Debug)]
pub struct FaceData<T: Float> {
    pub f: FaceRef<T>,
    pub l: BrepLoop<T>,
    pub skip: bool,
    pub rev: bool,
}

impl<T: Float> FaceData<T> {
    pub fn new(f: FaceRef<T>, l: BrepLoop<T>) -> Self {
        Self {
            f,
            l,
            skip: true,
            rev: false,
        }
    }

    pub fn edges_iter(&self) -> impl Iterator<Item = EdgeData<T>> + '_ {
        self.l
            .iter()
            .zip(self.l.iter().cycle().skip(1))
            .map(|(v1, v2)| (Rc::clone(v1), Rc::clone(v2)))
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Classification {
    Unknown,
    In,
    Out,
    Bad,
}

impl std::ops::BitOrAssign for Classification {
    fn bitor_assign(&mut self, rhs: Self) {
        if Self::Unknown == rhs {
            return;
        }

        if Self::Unknown == *self {
            *self = rhs
        } else if *self != rhs {
            *self = Self::Bad
        }
    }
}

#[derive(Debug)]
pub struct ClassificationResult {
    pub group_ids: Vec<usize>,
    pub face_groups: Map<usize, (Vec<usize>, Classification)>,
}

impl ClassificationResult {
    pub fn new(
        group_ids: Vec<usize>,
        face_groups: Map<usize, (Vec<usize>, Classification)>,
    ) -> Self {
        Self {
            group_ids,
            face_groups,
        }
    }
}

#[derive(Debug)]
pub struct ClassificationData<'a, T: Float> {
    pub faces_a: &'a Vec<FaceData<T>>,
    pub faces_b: &'a Vec<FaceData<T>>,
    pub ef_map_a: EdgeFaceMap<T>,
    pub ef_map_b: EdgeFaceMap<T>,
    pub shared_edges: Set<VV<T>>,
    pub faces_a_with_shared_edges: Set<usize>,
    pub faces_b_with_shared_edges: Set<usize>,
    pub face_angles: Map<VV<T>, (T, T)>,
}

impl<'a, T: Float> ClassificationData<'a, T> {
    pub fn new(faces_a: &'a Vec<FaceData<T>>, faces_b: &'a Vec<FaceData<T>>) -> Self {
        Self {
            faces_a,
            faces_b,
            ef_map_a: new_map(),
            ef_map_b: new_map(),
            shared_edges: new_set(),
            faces_a_with_shared_edges: new_set(),
            faces_b_with_shared_edges: new_set(),
            face_angles: new_map(),
        }
    }
}
