use super::*;

pub fn zero<T>() -> RtnType<T>
where
    T: Float,
{
    [T::zero(), T::zero()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2d_zero() {
        assert_eq!([0.0, 0.0], vector2d::zero());
    }
}
