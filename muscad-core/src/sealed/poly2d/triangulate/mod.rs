use super::*;

use point::Point;

mod point;

pub fn triangulate_unchecked<T, V, P>(poly: &V) -> Vec<usize>
where
    V: AsRef<[P]>,
    P: AsV3d<T>,
    T: Float,
{
    let poly = poly.as_ref();
    let mut n = poly.len();
    let mut pts = Vec::with_capacity(n);
    let mut result = vec![];

    for (i, ptr) in poly.iter().enumerate() {
        pts.push(Point::build(i, ptr));
    }

    for i in 0..n {
        pts[i].is_ear = is_ear(&pts, n, i);
    }

    while n > 3 {
        let ear_i = pts.iter().position(|p| p.is_ear).unwrap();

        _output_triangle(&mut result, &pts, n, ear_i);
        pts.remove(ear_i);
        n -= 1;

        let prev_i = _prev(n, ear_i);
        let next_i = ear_i; //since ear_i removed.
        pts[prev_i].is_ear = is_ear(&pts, n, prev_i);
        pts[next_i].is_ear = is_ear(&pts, n, next_i);
    }

    _output_triangle(&mut result, &pts, n, 1);

    result
}

fn _output_triangle<'a, T, P>(out: &mut Vec<usize>, pts: &Vec<Point<'a, T, P>>, n: usize, i: usize)
where
    P: AsV3d<T>,
    T: Float,
{
    out.push(pts[_prev(n, i)].i);
    out.push(pts[i].i);
    out.push(pts[_next(n, i)].i);
}

fn is_ear<'a, T, P>(pts: &Vec<Point<'a, T, P>>, n: usize, i: usize) -> bool
where
    P: AsV3d<T>,
    T: Float,
{
    let prev_i = _prev(n, i);
    let next_i = _next(n, i);

    // const execution time
    if !(_in_cone(pts, n, prev_i, next_i) && _in_cone(pts, n, next_i, prev_i)) {
        return false;
    }

    // O(n^2)
    !_passes_edge(pts, n, prev_i, next_i)
}

fn _passes_edge<'a, T, P>(pts: &Vec<Point<'a, T, P>>, n: usize, i: usize, j: usize) -> bool
where
    P: AsV3d<T>,
    T: Float,
{
    let c = pts[i].ptr;
    let d = pts[j].ptr;

    for idx_a in 0..n {
        let idx_b = _next(n, idx_a);

        let a = pts[idx_a].ptr;
        let b = pts[idx_b].ptr;

        if vector2d::eql(a, c) || vector2d::eql(a, d) || vector2d::eql(b, c) || vector2d::eql(b, d)
        {
            continue;
        }

        if intersect(c, d, a, b) {
            // println!("intersect: {},{} {},{}", i, j, idx_a, idx_b,);

            return true;
        }
    }
    false
}

fn _in_cone<'a, T, P>(pts: &Vec<Point<'a, T, P>>, n: usize, i: usize, j: usize) -> bool
where
    P: AsV3d<T>,
    T: Float,
{
    // check whether point j is in the cone formed by (i, i+1) (i, i-1)
    let o = pts[i].ptr;
    let a = pts[_next(n, i)].ptr;
    let b = pts[_prev(n, i)].ptr;

    let p = pts[j].ptr;

    in_cone(o, a, b, p)
}

const fn _prev(n: usize, i: usize) -> usize {
    (i + n - 1) % n
}

const fn _next(n: usize, i: usize) -> usize {
    (i + 1) % n
}
