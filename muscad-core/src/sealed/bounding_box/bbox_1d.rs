use super::*;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox1d<T> {
    min: T,
    max: T,
}

impl<T: Float> BoundingBox1d<T> {
    pub fn new() -> Self {
        Self {
            min: T::infinity(),
            max: T::neg_infinity(),
        }
    }

    pub fn add_point(&mut self, v: &T) {
        if self.min.eps_gt(v) {
            self.min = v.clone();
        }
        if self.max.eps_lt(v) {
            self.max = v.clone();
        }
    }

    pub fn is_valid(&self) -> bool {
        self.span().eps_gte(&T::ZERO)
    }

    pub fn contains(&self, v: &T) -> bool {
        self.min.eps_lte(v) && self.max.eps_gte(v)
    }

    pub fn center(&self) -> T {
        (self.min.clone() + &self.max) * &T::HALF
    }

    pub fn span(&self) -> T {
        self.max.clone() - &self.min
    }

    pub fn clear(&mut self) {
        self.min = T::infinity();
        self.max = T::neg_infinity();
    }

    pub fn axis_separation(&self, other: &Self) -> T {
        let c1 = self.center();
        let c2 = other.center();
        let s1 = self.span();
        let s2 = other.span();

        (c1 - &c2).abs() - &((s1 + &s2) * &T::HALF)
    }

    #[allow(unused)]
    pub fn intersects(&self, other: &Self) -> bool {
        self.axis_separation(other).eps_lte(&T::ZERO)
    }

    pub fn min(&self) -> T {
        self.min.clone()
    }

    pub fn max(&self) -> T {
        self.max.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox1d_min_max() {
        let mut bb = BoundingBox1d::new();
        bb.add_point(&0.0);
        assert_eq!(bb.min(), 0.0);
        assert_eq!(bb.max(), 0.0);
    }

    #[test]
    fn test_bbox1d_add() {
        let mut bb = BoundingBox1d::new();
        bb.add_point(&0.0);
        assert_eq!(bb.min, 0.0);
        assert_eq!(bb.max, 0.0);
    }

    #[test]
    fn test_bbox1d_is_valid() {
        let mut bb = BoundingBox1d::<f64>::new();
        bb.add_point(&0.0);
        assert!(bb.is_valid());
        bb.clear();
        assert!(!bb.is_valid());
    }

    #[test]
    fn test_bbox1d_clear() {
        let mut bb = BoundingBox1d::new();
        bb.add_point(&0.0);
        bb.clear();
        assert_eq!(bb.max, -1.0 / 0.0);
        assert_eq!(bb.min, 1.0 / 0.0);
    }

    #[test]
    fn test_bbox1d_center() {
        let mut bb = BoundingBox1d::<f64>::new();
        assert!(bb.center().is_nan());

        bb.add_point(&0.0);
        bb.add_point(&1.0);
        assert_eq!(bb.center(), 0.5);
        bb.add_point(&2.0);
        assert_eq!(bb.center(), 1.0);
    }

    #[test]
    fn test_bbox1d_span() {
        let mut bb = BoundingBox1d::<f64>::new();
        assert_eq!(bb.span(), -1.0 / 0.0);

        bb.add_point(&0.0);
        bb.add_point(&-1.0);
        assert_eq!(bb.span(), 1.0);
        bb.add_point(&-2.0);
        assert_eq!(bb.span(), 2.0);
    }

    #[test]
    fn test_bbox1d_contains() {
        let mut bb = BoundingBox1d::new();
        let [p1, p2, p3, p4, p5, p6] = &[-3.0, -2.0, -1.0, 1.0, 2.0, 3.0];

        bb.add_point(p2);
        assert!(bb.contains(p2));

        bb.add_point(p3);
        bb.add_point(p4);
        bb.add_point(p5);
        assert!(bb.contains(p3));
        assert!(bb.contains(p4));
        assert!(bb.contains(p5));

        assert!(!bb.contains(p1));
        assert!(!bb.contains(p6));
    }

    #[test]
    fn test_bbox1d_axis_separation() {
        let mut b1 = BoundingBox1d::<f64>::new();
        let mut b2 = BoundingBox1d::<f64>::new();
        assert!(b1.axis_separation(&b2).is_nan());

        b1.add_point(&-1.0);
        b1.add_point(&1.0);

        b2.add_point(&9.0);
        b2.add_point(&4.0);
        assert_eq!(b1.axis_separation(&b2), 3.0);

        b2.clear();
        b2.add_point(&-4.0);
        b2.add_point(&4.0);
        assert_eq!(b1.axis_separation(&b2), -5.0);
    }

    #[test]
    fn test_bbox1d_axis_intersects() {
        let mut b1 = BoundingBox1d::<f64>::new();
        let mut b2 = BoundingBox1d::<f64>::new();
        assert!(!b1.intersects(&b2));

        b1.add_point(&-1.0);
        b1.add_point(&1.0);

        b2.add_point(&9.0);
        b2.add_point(&4.0);
        assert!(!b1.intersects(&b2));

        b2.clear();
        b2.add_point(&-4.0);
        b2.add_point(&4.0);
        assert!(b1.intersects(&b2));
    }
}
