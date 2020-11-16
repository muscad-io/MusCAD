use super::*;

pub use intersect_line_line::*;
pub use intersect_line_plane::*;
pub use length::*;
pub use on_segment::*;

mod intersect_line_line;
mod intersect_line_plane;
mod length;
mod on_segment;

pub struct SegmentPointPoint;
pub struct SegmentPointVector;

type PointPoint = SegmentPointPoint;
type PointVector = SegmentPointVector;

#[derive(Debug)]
pub struct Segment<T, L = PointPoint> {
    pub v1: [T; 3],
    pub v2: [T; 3],
    _l: PhantomData<L>,
}

pub trait AsSegment<T> {
    fn to_p1(&self) -> [T; 3];
    fn to_p2(&self) -> [T; 3];
    fn to_direction(&self) -> [T; 3];
}

impl<T: Float, L> Segment<T, L> {
    pub fn new() -> Self {
        Self {
            v1: vector3d::zero(),
            v2: vector3d::zero(),
            _l: PhantomData,
        }
    }
}

impl<T: Float> AsSegment<T> for &Segment<T> {
    fn to_p1(&self) -> [T; 3] {
        self.v1.to_owned()
    }

    fn to_p2(&self) -> [T; 3] {
        self.v2.to_owned()
    }
    fn to_direction(&self) -> [T; 3] {
        let p2 = self.v2.to_owned();
        let p1 = &self.v1;

        v3d!(p2 - p1)
    }
}

impl<T: Float> AsSegment<T> for Segment<T> {
    fn to_p1(&self) -> [T; 3] {
        self.v1.to_owned()
    }

    fn to_p2(&self) -> [T; 3] {
        self.v2.to_owned()
    }
    fn to_direction(&self) -> [T; 3] {
        let p2 = self.v2.to_owned();
        let p1 = &self.v1;

        v3d!(p2 - p1)
    }
}

impl<T: Float> AsSegment<T> for Segment<T, PointVector> {
    fn to_p1(&self) -> [T; 3] {
        self.v1.to_owned()
    }

    fn to_p2(&self) -> [T; 3] {
        let v = self.v2.to_owned();
        let p1 = &self.v1;

        v3d!(v + p1)
    }
    fn to_direction(&self) -> [T; 3] {
        self.v2.to_owned()
    }
}

impl<T: Float, L> Segment<T, L> where Self: AsSegment<T> {}

impl<T: Float> Segment<T, PointPoint> {
    pub fn from_point_point(v1: [T; 3], v2: [T; 3]) -> Self {
        Self {
            v1,
            v2,
            _l: PhantomData,
        }
    }
}

impl<T: Float> Segment<T, PointVector> {
    pub fn from_point_vector(v1: [T; 3], v2: [T; 3]) -> Self {
        Self {
            v1,
            v2,
            _l: PhantomData,
        }
    }
}

/*
impl<T, V: AsSegment<T>> AsSegment<T> for Rc<RefCell<V>> {
    fn to_p1(&self) -> [T; 3] {
        self.borrow().to_p1()
    }

    fn to_p2(&self) -> [T; 3] {
        self.borrow().to_p2()
    }
    fn to_direction(&self) -> [T; 3] {
        self.borrow().to_direction()
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_pt_pt() {
        let p1 = [1.0, 2.0, 3.0];
        let p2 = [10.0, 10.0, 10.0];
        let v = [9.0, 8.0, 7.0];

        let ln = Segment::from_point_point(p1.clone(), p2.clone());

        assert_eq!(p1, ln.to_p1());
        assert_eq!(p2, ln.to_p2());
        assert_eq!(v, ln.to_direction());
    }

    #[test]
    fn test_segment_pt_vec() {
        let p1 = [1.0, 2.0, 3.0];
        let p2 = [10.0, 10.0, 10.0];
        let v = [9.0, 8.0, 7.0];

        let ln = Segment::from_point_vector(p1.clone(), v.clone());

        assert_eq!(p1, ln.to_p1());
        assert_eq!(p2, ln.to_p2());
        assert_eq!(v, ln.to_direction());
    }
}
