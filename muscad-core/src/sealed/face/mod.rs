use super::*;

//mod inspect;
mod iter;

#[cfg(feature = "serialization")]
mod serialize;

pub type FaceRef<T> = Rc<RefCell<Face<T>>>;
pub type FaceRefWeak<T> = Weak<RefCell<Face<T>>>;

pub struct Face<T> {
    plane: [T; 4],
    pub bbox: BoundingBox<T>,
    pub edge: EdgeRefWeak<T>,
    pub n_edges: usize,
    pub project: fn(&[T; 3]) -> [T; 2],
    id: entity::ID,
}

impl_id!(FACE_ID, Face<T>);

impl<T: Float> Face<T> {
    pub fn new() -> Self {
        Self {
            plane: [T::zero(), T::zero(), T::zero(), T::zero()],
            bbox: BoundingBox::new(),
            edge: Weak::new(),
            n_edges: 0,
            project: project2d::project_z,
            id: Self::_next_id(),
        }
    }

    pub fn to_plane(&self) -> [T; 4] {
        self.plane.clone()
    }

    pub fn to_positions(&self) -> Vec<[T; 3]> {
        self.vertices_iter().map(|v| v.to_position()).collect()
    }

    pub fn to_positions2d(&self) -> Vec<[T; 2]> {
        self.vertices_iter()
            .map(|v| (self.project)(v.as_position()))
            .collect()
    }

    pub fn contains_point<V>(&self, pt: &V) -> bool
    where
        V: AsV3d<T>,
    {
        if !plane::on_plane(&self.plane, pt) {
            return false;
        }

        let pt = pt.as_ref();

        // TODO: avoid creating new Vec
        let poly2d = self
            .vertices_iter()
            .map(|v| (self.project)(v.as_position()))
            .collect::<Vec<_>>();
        poly2d::point_in_poly_unchecked(&(self.project)(pt.try_into().unwrap()), &poly2d)
    }

    pub fn calc(&mut self) {
        self.bbox.clear();

        for v in self.vertices_iter() {
            self.bbox.add_point(&v.as_ref());
        }

        self.plane = plane::points_iter_to_plane(self.vertices_iter().map(|v| v.to_position()));

        self.project = plane::to_project2d_functor(&self.plane);
    }
}

impl<T: Float> AsPlane<T> for Face<T> {
    fn a(&self) -> &T {
        &self.plane[0]
    }

    fn b(&self) -> &T {
        &self.plane[1]
    }

    fn c(&self) -> &T {
        &self.plane[2]
    }

    fn d(&self) -> &T {
        &self.plane[3]
    }

    fn to_normal(&self) -> [T; 3] {
        [self.a().clone(), self.b().clone(), self.c().clone()]
    }
}

impl<T> Debug for Face<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "F{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_plane_and_bbox() {
        let mut m = Model::new();
        let pts = [[10.0, 0.0, 0.0], [10.0, 1.0, 0.0], [10.0, 0.0, 1.0]];
        let ptr = m.add_face_unchecked(&pts);
        let mut f = ptr.borrow_mut();
        f.calc();

        let bb = &f.bbox;

        assert_eq!(f.plane.to_normal(), [1.0, 0.0, 0.0]);
        assert_eq!(*f.plane.d(), 10.0);
        assert_eq!((f.project)(&pts[1]), [1.0, 0.0]);
        assert_eq!(bb.min_x(), 10.0);
        assert_eq!(bb.max_x(), 10.0);
        assert_eq!(bb.min_y(), 0.0);
        assert_eq!(bb.max_y(), 1.0);
        assert_eq!(bb.min_z(), 0.0);
        assert_eq!(bb.max_z(), 1.0);
    }

    #[test]
    fn test_face_contains() {
        let mut m = Model::new();
        let pts = [[10.0, 0.0, 0.0], [10.0, 1.0, 0.0], [10.0, 0.0, 1.0]];
        let ptr = m.add_face_unchecked(&pts);
        let mut f = ptr.borrow_mut();
        f.calc();

        //TODO: on edge
        assert!(f.contains_point(&[10.0, 0.0, 0.01]));
        assert!(!f.contains_point(&[10.0, 0.0, -0.01]));
    }

    #[test]
    fn test_face_as_plane() {
        let mut m = Model::new();
        let pts = [[10.0, 0.0, 0.0], [10.0, 1.0, 0.0], [10.0, 0.0, 1.0]];
        let ptr = m.add_face_unchecked(&pts);
        let mut f = ptr.borrow_mut();
        f.calc();

        let ln = Segment::from_point_vector([1.0, 1.0, 1.0], [1.0, 0.0, 0.0]);
        let pti = segment::intersect_line_plane(&ln, &*f).unwrap();

        assert_eq!([10.0, 1.0, 1.0], pti);
    }
}
