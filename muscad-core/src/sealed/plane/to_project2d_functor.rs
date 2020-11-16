use super::*;

pub fn to_project2d_functor<T, V>(pl: &V) -> fn(&[T; 3]) -> [T; 2]
where
    V: AsPlane<T>,
    T: Float,
{
    project2d::to_functor(pl.a(), pl.b(), pl.c())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plane_to_project2d_functor() {
        let pl = [0.0, -10.0, 0.0, 100.0];

        let f = to_project2d_functor(&pl);

        let p1 = [1.0, 2.0, 3.0];

        assert_eq!(f(&p1), [1.0, 3.0]);
    }
}
