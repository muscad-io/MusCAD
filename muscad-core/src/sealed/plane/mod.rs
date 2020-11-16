use super::*;

pub use distance_to_point::*;
pub use negate::*;
pub use on_plane::*;
pub use to_project2d_functor::*;

mod distance_to_point;
mod negate;
mod on_plane;
mod to_project2d_functor;

pub trait AsPlane<T> {
    fn a(&self) -> &T;
    fn b(&self) -> &T;
    fn c(&self) -> &T;
    fn d(&self) -> &T;
    fn to_normal(&self) -> [T; 3];
}

impl<T: Float> AsPlane<T> for [T; 4] {
    fn a(&self) -> &T {
        &self[0]
    }

    fn b(&self) -> &T {
        &self[1]
    }

    fn c(&self) -> &T {
        &self[2]
    }

    fn d(&self) -> &T {
        &self[3]
    }

    fn to_normal(&self) -> [T; 3] {
        [self.a().clone(), self.b().clone(), self.c().clone()]
    }
}

pub fn points_iter_to_plane<T, I, P>(pts_iter: I) -> [T; 4]
where
    P: AsV3d<T>,
    I: Iterator<Item = P> + Clone,
    T: Float,
{
    // TODO: TEMP. Find a better algorithm

    let mut sum = vector3d::zero();
    let first_p = pts_iter.clone().next().unwrap();

    let it = to_each_cons_3_cycle_iter(pts_iter);

    for (p1, p2, p3) in it {
        let (p1, p2, p3) = (p1.as_ref(), p2.as_ref(), p3.as_ref());

        vector3d::add_mut(&mut sum, &v3d!((p2 - p1) * (p3 - p2)));
    }

    vector3d::normalize_mut(&mut sum);

    let d = vector3d::dot(&sum, &first_p);
    let [a, b, c] = sum;

    [a, b, c, d]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_iter_to_plane() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let p = points_iter_to_plane(pts.iter());
        let p2 = points_iter_to_plane(pts.iter().rev());

        assert_eq!(p.to_normal(), [0.0, 0.0, 1.0]);
        assert_eq!(*p.d(), 0.0);
        assert_eq!(p2.to_normal(), [0.0, 0.0, -1.0]);
        assert_eq!(*p2.d(), 0.0);
    }
}
