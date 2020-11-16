#[allow(unused_imports)]
use super::*;

#[macro_export]
macro_rules! v2d {
    ([(@value $t:ident)]) => {
        $t;
    };

    ([(@value ($($t:tt)*))]) => {
        v2d!([] $($t)*);
    };

    ([(@rev $t:tt)]) => {
        vector2d::reverse(&v2d!([] $t));
    };

    ([(@add $l:tt $r:tt)]) => {
        vector2d::add(&v2d!([] $l), &v2d!([] $r));
    };

    ([(@sub $l:tt $r:tt)]) => {
        vector2d::sub(&v2d!([] $l), &v2d!([] $r));
    };

    ([(@cross $l:tt $r:tt)]) => {
        vector2d::cross(&v2d!([] $l), &v2d!([] $r));
    };

    ([(@mul $l:tt $r:tt)]) => {
        vector2d::mul(&v2d!([] $l), &v2d!([] $r));
    };

    ([(@value $l:tt) (@op $op:tt) (@op -) (@value $r:tt)] $($rest:tt)*) => {
        v2d!([(@value $l)] $op (- $r) $($rest)*);
    };

    ([(@value $v:tt) (@op +)] $($rest:tt)*) => {
        v2d!([(@add $v ($($rest)*))]);
    };

    ([(@value $l:tt) (@op -)] $r:tt) => {
        v2d!([(@sub $l $r)]);
    };

    ([(@value $l:tt) (@op -) (@value $r:tt)] $($rest:tt)*) => {
        v2d!([(@value $l) (@op +)] (- $r) $($rest)*);
    };

    ([(@value $v:tt) (@op .*)] $r:tt) => {
        v2d!([(@mul $v $r)]);
    };

    ([(@value $v:tt) (@op *)] $r:tt) => {
        v2d!([(@cross $v $r)]);
    };

    ([(@op -)] $t:tt) => {
        v2d!([(@rev $t)]);
    };

    ([(@value $l:tt) (@op *) (@value $r:tt)] $($rest:tt)*) => {
        v2d!([(@value ($l * $r))] $($rest)*);
    };

    ([(@value $l:tt) (@op .*) (@value $r:tt)] $($rest:tt)*) => {
        v2d!([(@value ($l .* $r))] $($rest)*);
    };

    ([(@op -) (@value $v:tt)] $($rest:tt)*) => {
        v2d!([(@value (- $v))] $($rest)*);
    };

    ([$($store:tt)*] + $($rest:tt)*) => {
        v2d!([$($store)* (@op +)] $($rest)*);
    };

    ([$($store:tt)*] - $($rest:tt)*) => {
        v2d!([$($store)* (@op -)] $($rest)*);
    };

    ([$($store:tt)*] .* $($rest:tt)*) => {
        v2d!([$($store)* (@op .*)] $($rest)*);
    };

    ([$($store:tt)*] * $($rest:tt)*) => {
        v2d!([$($store)* (@op *)] $($rest)*);
    };

    ([$($store:tt)*] $head:tt $($rest:tt)*) => {
        v2d!([$($store)* (@value $head)] $($rest)*);
    };

    ([$($store:tt)*]) => {
        [$($store,)*];
    };

    ($out:ident += $($t:tt)*) => {
        vector2d::add_mut(&mut $out, &v2d!($($t)*));
    };

    ($out:ident -= $($t:tt)*) => {
        vector2d::sub_mut(&mut $out, &v2d!($($t)*));
    };

    ($($t:tt)*) => {
        v2d!([] $($t)*);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_v2d() {
        let v1 = [-1.0, 9.0];
        let v2 = [2.0, 8.0];
        let v3 = [-4.0, 2.0];

        let r1 = v2d!(-(v2 - v1) - v2 + v3);
        let r2 = v2d!(v3 - v2 - (v2 - v1));
        let r3 = vector2d::sub(&vector2d::sub(&v3, &v2), &vector2d::sub(&v2, &v1));

        assert_eq!(r1, r2);
        assert_eq!(r1, r3);

        let t = 0.5;
        assert_eq!(
            v2d!((v1 + v2) .* t),
            vector2d::mul(&vector2d::add(&v1, &v2), &t)
        );
    }
}
