use super::*;

pub fn negate<T, V>(pl: &V) -> [T; 4]
where
    V: AsPlane<T>,
    T: Float,
{
    [
        -pl.a().clone(),
        -pl.b().clone(),
        -pl.c().clone(),
        -pl.d().clone(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_negate() {
        let pl = [1.0, 2.0, 3.0, 4.0];

        let p = negate(&pl);

        assert_eq!(p.to_normal(), [-1.0, -2.0, -3.0]);
        assert_eq!(*p.d(), -4.0);
    }
}
