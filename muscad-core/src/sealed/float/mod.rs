use super::*;

/// Float Trait
/// this is used as trait bounds for values of f64, f32 and other types (e.g. ExactFloat)

pub trait FloatOps<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> {}

macro_rules! def_const {
    ($const_name:ident, $method_name:ident) => {
        const $const_name: Self;

        fn $method_name() -> Self;
    };
}

mod seal {
    pub trait Sealed {}
    impl Sealed for f64 {}
}

pub trait Float
where
    Self: seal::Sealed,
    for<'a> Self: Add<&'a Self, Output = Self>,
    for<'a> Self: Sub<&'a Self, Output = Self>,
    for<'a> Self: Mul<&'a Self, Output = Self>,
    for<'a> Self: Div<&'a Self, Output = Self>,
    for<'a> Self: AddAssign<&'a Self>,
    for<'a> Self: SubAssign<&'a Self>,
    for<'a> Self: MulAssign<&'a Self>,
    for<'a> Self: DivAssign<&'a Self>,
    Self: Debug,
    Self: PartialOrd,
    Self: Neg<Output = Self>,
    Self: Clone,
{
    def_const!(ZERO, zero);
    def_const!(PI, pi);
    def_const!(TAU, tau);
    def_const!(ONE, one);
    def_const!(HALF, half);
    def_const!(INFINITY, infinity);
    def_const!(NEG_INFINITY, neg_infinity);

    fn abs(self) -> Self;
    fn round(self) -> Self;
    fn sqrt(self) -> Self;
    fn is_nan(self) -> bool;
    fn atan2(self, other: Self) -> Self;

    fn eps() -> Self;
    fn lt_eps_sq(&self) -> bool;
    fn eps_eql(&self, other: &Self) -> bool;
    fn eps_lt(&self, other: &Self) -> bool;
    fn eps_lte(&self, other: &Self) -> bool;
    fn eps_gt(&self, other: &Self) -> bool;
    fn eps_gte(&self, other: &Self) -> bool;
    fn eps_partial_cmp(&self, other: &Self) -> Option<Ordering>;

    fn hash_val(v: &Self) -> isize;
}

macro_rules! fwd_method {
    ($typename:ident, $fname:ident, $rtntype:ident) => {
        fn $fname(self) -> $rtntype {
            $typename::$fname(self)
        }
    };

    (
        $typename:ident,
        $fname:ident($($arg:ident : $argtype: ident),+),
        $rtntype:ident
    ) => {
        fn $fname(self, $($arg: $argtype),+) -> $rtntype {
            $typename::$fname(self, $($arg),+)
        }
    };
}

macro_rules! impl_const {
    ($const_name:ident, $method_name:ident, $val:expr) => {
        const $const_name: Self = $val;

        fn $method_name() -> Self {
            $val
        }
    };
}

const F64_EPS: f64 = 1.4901161193847656e-08;
const F64_EPS_SQ: f64 = F64_EPS * F64_EPS;

impl Float for f64 {
    impl_const!(ZERO, zero, 0.0);
    impl_const!(PI, pi, core::f64::consts::PI);
    impl_const!(TAU, tau, core::f64::consts::TAU);
    impl_const!(ONE, one, 1.0);
    impl_const!(HALF, half, 0.5);
    impl_const!(INFINITY, infinity, f64::INFINITY);
    impl_const!(NEG_INFINITY, neg_infinity, f64::NEG_INFINITY);

    fwd_method!(f64, abs, Self);
    fwd_method!(f64, round, Self);
    fwd_method!(f64, sqrt, Self);
    fwd_method!(f64, is_nan, bool);
    fwd_method!(f64, atan2(other: Self), Self);

    fn eps() -> Self {
        F64_EPS
    }
    fn lt_eps_sq(&self) -> bool {
        self.abs() < F64_EPS_SQ
    }
    fn eps_eql(&self, other: &Self) -> bool {
        (*other - *self).abs() < Self::eps()
    }
    fn eps_lt(&self, other: &Self) -> bool {
        *other - *self >= Self::eps()
    }
    fn eps_lte(&self, other: &Self) -> bool {
        self.eps_lt(other) || self.eps_eql(other)
    }
    fn eps_gt(&self, other: &Self) -> bool {
        other.eps_lt(self)
    }
    fn eps_gte(&self, other: &Self) -> bool {
        self.eps_gt(other) || self.eps_eql(other)
    }
    fn eps_partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            None
        } else {
            Some(if self.eps_eql(other) {
                Ordering::Equal
            } else if self.eps_lt(other) {
                Ordering::Less
            } else {
                Ordering::Greater
            })
        }
    }

    fn hash_val(v: &Self) -> isize {
        (v * 10e6).round() as isize
    }
}
