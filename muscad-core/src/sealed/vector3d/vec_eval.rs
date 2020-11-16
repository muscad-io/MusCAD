#[allow(unused_imports)]
use super::*;

#[macro_export]
macro_rules! v3d {
    ([(@value $t:ident)]) => {
        $t;
    };

    ([(@value ($($t:tt)*))]) => {
        v3d!([] $($t)*);
    };

    ([(@rev $t:tt)]) => {
        vector3d::reverse(&v3d!([] $t));
    };

    ([(@add $l:tt $r:tt)]) => {
        vector3d::add(&v3d!([] $l), &v3d!([] $r));
    };

    ([(@sub $l:tt $r:tt)]) => {
        vector3d::sub(&v3d!([] $l), &v3d!([] $r));
    };

    ([(@cross $l:tt $r:tt)]) => {
        vector3d::cross(&v3d!([] $l), &v3d!([] $r));
    };

    ([(@mul $l:tt $r:tt)]) => {
        vector3d::mul(&v3d!([] $l), &v3d!([] $r));
    };

    ([(@value $l:tt) (@op $op:tt) (@op -) (@value $r:tt)] $($rest:tt)*) => {
        v3d!([(@value $l)] $op (- $r) $($rest)*);
    };

    ([(@value $v:tt) (@op +)] $($rest:tt)*) => {
        v3d!([(@add $v ($($rest)*))]);
    };

    ([(@value $l:tt) (@op -)] $r:tt) => {
        v3d!([(@sub $l $r)]);
    };

    ([(@value $l:tt) (@op -) (@value $r:tt)] $($rest:tt)*) => {
        v3d!([(@value $l) (@op +)] (- $r) $($rest)*);
    };

    ([(@value $v:tt) (@op .*)] $r:tt) => {
        v3d!([(@mul $v $r)]);
    };

    ([(@value $v:tt) (@op *)] $r:tt) => {
        v3d!([(@cross $v $r)]);
    };

    ([(@op -)] $t:tt) => {
        v3d!([(@rev $t)]);
    };

    ([(@value $l:tt) (@op *) (@value $r:tt)] $($rest:tt)*) => {
        v3d!([(@value ($l * $r))] $($rest)*);
    };

    ([(@value $l:tt) (@op .*) (@value $r:tt)] $($rest:tt)*) => {
        v3d!([(@value ($l .* $r))] $($rest)*);
    };

    ([(@op -) (@value $v:tt)] $($rest:tt)*) => {
        v3d!([(@value (- $v))] $($rest)*);
    };

    ([$($store:tt)*] + $($rest:tt)*) => {
        v3d!([$($store)* (@op +)] $($rest)*);
    };

    ([$($store:tt)*] - $($rest:tt)*) => {
        v3d!([$($store)* (@op -)] $($rest)*);
    };

    ([$($store:tt)*] .* $($rest:tt)*) => {
        v3d!([$($store)* (@op .*)] $($rest)*);
    };

    ([$($store:tt)*] * $($rest:tt)*) => {
        v3d!([$($store)* (@op *)] $($rest)*);
    };

    ([$($store:tt)*] $head:tt $($rest:tt)*) => {
        v3d!([$($store)* (@value $head)] $($rest)*);
    };

    ([$($store:tt)*]) => {
        [$($store,)*];
    };

    ($out:ident += $($t:tt)*) => {
        vector3d::add_mut(&mut $out, &v3d!($($t)*));
    };

    ($out:ident -= $($t:tt)*) => {
        vector3d::sub_mut(&mut $out, &v3d!($($t)*));
    };

    ($($t:tt)*) => {
        v3d!([] $($t)*);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_v3d() {
        let v1 = [-1.0, 9.0, 2.0];
        let v2 = [2.0, 8.0, 3.0];
        let v3 = [-4.0, 2.0, 1.0];

        let r1 = v3d!(v3 * -(v2 - v1) - v2 + v3);
        let r2 = v3d!(v3 - v2 - (v2 - v1) * -v3);
        let r3 = vector3d::add(
            &vector3d::sub(&vector3d::cross(&vector3d::sub(&v2, &v1), &v3), &v2),
            &v3,
        );

        assert_eq!(r1, r2);
        assert_eq!(r1, r3);

        let t = 0.5;
        assert_eq!(
            v3d!((v1 + v2) .* t),
            vector3d::mul(&vector3d::add(&v1, &v2), &t)
        );
    }
}
