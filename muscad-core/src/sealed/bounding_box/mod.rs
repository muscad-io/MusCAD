use super::*;

use bbox_1d::*;

mod bbox_1d;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox<T> {
    inner: [BoundingBox1d<T>; 3],
}

impl<T: Float> BoundingBox<T> {
    pub fn new() -> Self {
        Self {
            inner: [
                BoundingBox1d::new(),
                BoundingBox1d::new(),
                BoundingBox1d::new(),
            ],
        }
    }

    pub fn add_point<V>(&mut self, pt: &V)
    where
        V: AsV3d<T>,
    {
        let pt = pt.as_ref();

        self.inner[X].add_point(&pt[X]);
        self.inner[Y].add_point(&pt[Y]);
        self.inner[Z].add_point(&pt[Z]);
    }

    pub fn is_valid(&self) -> bool {
        self.inner[X].is_valid() && self.inner[Y].is_valid() && self.inner[Z].is_valid()
    }

    pub fn clear(&mut self) {
        self.inner[X].clear();
        self.inner[Y].clear();
        self.inner[Z].clear();
    }

    pub fn min_x(&self) -> T {
        self.inner[X].min()
    }

    pub fn min_y(&self) -> T {
        self.inner[Y].min()
    }

    pub fn min_z(&self) -> T {
        self.inner[Z].min()
    }

    pub fn max_x(&self) -> T {
        self.inner[X].max()
    }

    pub fn max_y(&self) -> T {
        self.inner[Y].max()
    }

    pub fn max_z(&self) -> T {
        self.inner[Z].max()
    }

    pub fn min(&self) -> [T; 3] {
        [self.min_x(), self.min_y(), self.min_z()]
    }

    pub fn max(&self) -> [T; 3] {
        [self.max_x(), self.max_y(), self.max_z()]
    }

    pub fn center_x(&self) -> T {
        self.inner[X].center()
    }

    pub fn center_y(&self) -> T {
        self.inner[Y].center()
    }

    pub fn center_z(&self) -> T {
        self.inner[Z].center()
    }

    pub fn center(&self) -> [T; 3] {
        [self.center_x(), self.center_y(), self.center_z()]
    }

    pub fn span_x(&self) -> T {
        self.inner[X].span()
    }

    pub fn span_y(&self) -> T {
        self.inner[Y].span()
    }

    pub fn span_z(&self) -> T {
        self.inner[Z].span()
    }

    pub fn span(&self) -> [T; 3] {
        [self.span_x(), self.span_y(), self.span_z()]
    }

    fn _axis_separation_x(&self, other: &BoundingBox1d<T>) -> T {
        self.inner[X].axis_separation(other)
    }

    fn _axis_separation_y(&self, other: &BoundingBox1d<T>) -> T {
        self.inner[Y].axis_separation(other)
    }

    fn _axis_separation_z(&self, other: &BoundingBox1d<T>) -> T {
        self.inner[Z].axis_separation(other)
    }

    pub fn axis_separation_x(&self, other: &Self) -> T {
        other._axis_separation_x(&self.inner[X])
    }

    pub fn axis_separation_y(&self, other: &Self) -> T {
        other._axis_separation_y(&self.inner[Y])
    }

    pub fn axis_separation_z(&self, other: &Self) -> T {
        other._axis_separation_z(&self.inner[Z])
    }

    pub fn axis_separation(&self, other: &Self) -> [T; 3] {
        [
            other.axis_separation_x(self),
            other.axis_separation_y(self),
            other.axis_separation_z(self),
        ]
    }

    pub fn contains<V>(&self, pt: &V) -> bool
    where
        V: AsV3d<T>,
    {
        let pt = pt.as_ref();
        self.inner[X].contains(&pt[X])
            && self.inner[Y].contains(&pt[Y])
            && self.inner[Z].contains(&pt[Z])
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let separation = self.axis_separation(other);

        T::ZERO.eps_gte(&separation[X])
            && T::ZERO.eps_gte(&separation[Y])
            && T::ZERO.eps_gte(&separation[Z])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_min_max() {
        let mut bb = BoundingBox::new();

        bb.add_point(&[1.0, 2.0, 3.0]);

        assert_eq!(bb.min_x(), 1.0);
        assert_eq!(bb.min_y(), 2.0);
        assert_eq!(bb.min_z(), 3.0);
        assert_eq!(bb.min(), [1.0, 2.0, 3.0]);

        assert_eq!(bb.max_x(), 1.0);
        assert_eq!(bb.max_y(), 2.0);
        assert_eq!(bb.max_z(), 3.0);
        assert_eq!(bb.max(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_bbox_add() {
        let mut bb = BoundingBox::new();

        bb.add_point(&[1.0, 2.0, 3.0]);

        assert_eq!(bb.inner[0].min(), 1.0);
        assert_eq!(bb.inner[0].max(), 1.0);
        assert_eq!(bb.inner[1].min(), 2.0);
        assert_eq!(bb.inner[1].max(), 2.0);
        assert_eq!(bb.inner[2].min(), 3.0);
        assert_eq!(bb.inner[2].max(), 3.0);
    }

    #[test]
    fn test_bbox_is_valid() {
        let mut bb = BoundingBox::<f64>::new();

        bb.add_point(&[1.0, 2.0, 3.0]);
        assert!(bb.is_valid());

        bb.clear();
        assert!(!bb.is_valid());
    }

    #[test]
    fn test_bbox_clear() {
        let mut bb = BoundingBox::new();

        bb.add_point(&[1.0, 2.0, 3.0]);
        bb.clear();

        assert_eq!(bb.inner[0].min(), 1.0 / 0.0);
        assert_eq!(bb.inner[0].max(), -1.0 / 0.0);
        assert_eq!(bb.inner[1].min(), 1.0 / 0.0);
        assert_eq!(bb.inner[1].max(), -1.0 / 0.0);
        assert_eq!(bb.inner[2].min(), 1.0 / 0.0);
        assert_eq!(bb.inner[2].max(), -1.0 / 0.0);
    }

    #[test]
    fn test_bbox_center() {
        let mut bb = BoundingBox::<f64>::new();

        assert!(bb.center_x().is_nan());
        assert!(bb.center_y().is_nan());
        assert!(bb.center_z().is_nan());

        bb.add_point(&[1.0, 2.0, 3.0]);
        assert_eq!(bb.center_x(), 1.0);
        assert_eq!(bb.center_y(), 2.0);
        assert_eq!(bb.center_z(), 3.0);
        assert_eq!(bb.center(), [1.0, 2.0, 3.0]);

        bb.add_point(&[0.0, 0.0, 0.0]);
        assert_eq!(bb.center_x(), 0.5);
        assert_eq!(bb.center_y(), 1.0);
        assert_eq!(bb.center_z(), 1.5);
        assert_eq!(bb.center(), [0.5, 1.0, 1.5]);
    }

    #[test]
    fn test_bbox_span() {
        let mut bb = BoundingBox::<f64>::new();

        assert_eq!(bb.span_x(), -1.0 / 0.0);
        assert_eq!(bb.span_y(), -1.0 / 0.0);
        assert_eq!(bb.span_z(), -1.0 / 0.0);

        bb.add_point(&[-1.0, -2.0, -3.0]);
        assert_eq!(bb.span_x(), 0.0);
        assert_eq!(bb.span_y(), 0.0);
        assert_eq!(bb.span_z(), 0.0);
        assert_eq!(bb.span(), [0.0, 0.0, 0.0]);

        bb.add_point(&[0.0, 0.0, 0.0]);
        assert_eq!(bb.span_x(), 1.0);
        assert_eq!(bb.span_y(), 2.0);
        assert_eq!(bb.span_z(), 3.0);
        assert_eq!(bb.span(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_bbox_contains() {
        let mut bb = BoundingBox::new();
        bb.add_point(&[-1.0, -1.0, -1.0]);
        bb.add_point(&[1.0, 1.0, 1.0]);
        assert!(bb.contains(&[0.0, 0.0, 0.0]));
        assert!(bb.contains(&[1.0, 0.0, 0.0]));
        assert!(bb.contains(&[0.0, 1.0, 0.0]));
        assert!(bb.contains(&[0.0, 0.0, 1.0]));
        assert!(!bb.contains(&[0.0, 0.0, 1.1]));
    }

    #[test]
    fn test_bbox_axis_saparation() {
        let mut b1 = BoundingBox::<f64>::new();
        let mut b2 = BoundingBox::<f64>::new();

        assert!(b1.axis_separation_x(&b2).is_nan());
        assert!(b1.axis_separation_y(&b2).is_nan());
        assert!(b1.axis_separation_z(&b2).is_nan());

        b1.add_point(&[-1.0, -1.0, -1.0]);
        b1.add_point(&[1.0, 1.0, 1.0]);

        b2.add_point(&[-1.0, 5.0, -4.0]);
        b2.add_point(&[1.0, 9.0, 4.0]);

        assert_eq!(b1.axis_separation_x(&b2), -2.0);
        assert_eq!(b1.axis_separation_y(&b2), 4.0);
        assert_eq!(b1.axis_separation_z(&b2), -5.0);
        assert_eq!(b1.axis_separation(&b2), [-2.0, 4.0, -5.0]);
    }

    #[test]

    fn test_bbox_axis_intersects() {
        let mut b1 = BoundingBox::<f64>::new();
        let mut b2 = BoundingBox::<f64>::new();
        let mut b3 = BoundingBox::<f64>::new();
        let mut b4 = BoundingBox::<f64>::new();
        assert!(!b2.intersects(&b1));

        b1.add_point(&[-1.0, -1.0, -1.0]);
        b1.add_point(&[1.0, 1.0, 1.0]);

        b2.add_point(&[-1.0, -1.0, -1.0]);
        assert!(b2.intersects(&b1));

        b3.add_point(&[0.0, 0.0, 0.0]);
        b3.add_point(&[0.1, 0.1, 0.1]);
        assert!(b3.intersects(&b1));

        b4.add_point(&[0.0, 0.0, 100.0]);
        assert!(!b4.intersects(&b1));
    }
}
