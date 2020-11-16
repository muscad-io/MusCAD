use super::*;

fn _split_face<T: Clone>(outer: &Vec<T>, edges: &Vec<(usize, usize, Vec<T>)>) -> Vec<Vec<T>> {
    let n = edges.len();
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        if _contains_other(n, i, edges) {
            result.push(_make_positive_loop(i, outer, edges));
        } else {
            result.push(_make_negative_loop(outer, &edges[i]));
        }
    }

    result
}

fn _make_positive_loop<T: Clone>(
    i: usize,
    outer: &Vec<T>,
    edges: &Vec<(usize, usize, Vec<T>)>,
) -> Vec<T> {
    let end = edges[i].1;
    let mut cur = edges[i].0;
    let mut j = i + 1;
    let mut positive_loop = vec![];

    while let Some(dj) = _get_next_range(end, cur, &edges[j..]) {
        j += dj;

        let range_start = edges[j].0;
        let edge_pts = &edges[j].2;
        let n_edges = edge_pts.len();

        _push_positive(&mut positive_loop, outer, cur, range_start);

        _push_positive(&mut positive_loop, edge_pts, 1, n_edges - 2);

        cur = edges[j].1;
    }

    let edge_pts = &edges[i].2;
    let n_edges = edge_pts.len();
    _push_positive(&mut positive_loop, outer, cur, end);
    _push_negative(&mut positive_loop, edge_pts, n_edges - 2, 1);

    positive_loop
}

fn _get_next_range<T: Clone>(
    end: usize,
    cur: usize,
    edges: &[(usize, usize, Vec<T>)],
) -> Option<usize> {
    for (i, edge) in edges.iter().enumerate() {
        let (edge_start, edge_end, _) = *edge;
        if edge_start > end || edge_end > end {
            return None;
        }
        if edge_start < cur {
            continue;
        }
        return Some(i);
    }
    None
}

fn _push_positive<T: Clone>(out: &mut Vec<T>, l: &Vec<T>, from: usize, to: usize) {
    let mut i = from;
    while i <= to {
        out.push(l[i].to_owned());
        i += 1;
    }
}

fn _push_negative<T: Clone>(out: &mut Vec<T>, l: &Vec<T>, from: usize, to: usize) {
    let mut i = from;
    while i >= to {
        out.push(l[i].to_owned());
        i -= 1;
    }
}

fn _make_negative_loop<T: Clone>(outer: &Vec<T>, edge: &(usize, usize, Vec<T>)) -> Vec<T> {
    let edge_pts = &edge.2;
    let n_edge = edge_pts.len();
    let (start, end, _) = *edge;

    let mut v = Vec::with_capacity(n_edge * 2 - 2);

    _push_negative(&mut v, &edge_pts, n_edge - 1, 1);

    _push_positive(&mut v, &outer, start, end - 1);

    v
}

fn _contains_other<T>(n: usize, i: usize, edges: &Vec<(usize, usize, Vec<T>)>) -> bool {
    i + 1 < n && edges[i + 1].1 <= edges[i].1
}

/*
    split with non-overlapping edges
    Input:
      base loop: [b0, ..., bn-1]
      sorted segments: (i1_start, i1_end), (i2_start, i2_end), ...
    Output:
      positive loop:  b[...] + i1[i1_start..i1_end] + b[...] + i2...
      negative loops: b[i1_end..i1_start] + i1[...]
                      b[i2_end..i2_start] + i2[...]
    Apply the above procedure recursively to deal with overlapping edges.

    example:
    y: (0, 3, [y0=b0,y1,y2,y3=b3])
    x: (1, 2, [x0=b1,x1,x2,x3=b2])
    z: (4, 9, [z0=b4,z1,z2,z3=b9])
    add auxilary edges: a = (0, 9, [a0=b0,a1=b9])

             b5 b4  b3 b2  b1 b0 b9  b8
             +--+----+-+---+-+---+---+
             |  |    | +---+ |   |   |
             |  |    |   x   |   |   |
             |  |    +-------+   |   |
             |  |        y       |   |
             |  +----------------+   |
             |           z           |
             +-----------------------+
             b6                     b7
    outputs:
    z: [b9, z2, z1] [b4,...,b8]
    x: [b2, x2, x1] [b1]
    y: [b0, b1] [x1, x2, b2] [b3 y2 y1]
    a: [b0, y1, y2, b3] [b4] [z1, z2] [b9]
*/
impl<T: Float> BooleanSolver<T> {
    pub fn split_face(&mut self, d: &mut BrepData<T>, mut edges: Vec<(usize, usize, BrepLoop<T>)>) {
        let n = d.outer_loop.len();
        edges.push((
            0,
            n - 1,
            vec![d.outer_loop[0].to_owned(), d.outer_loop[n - 1].to_owned()],
        ));
        edges.sort_by(|a, b| a.0.cmp(&b.0));

        let faces = _split_face(&d.outer_loop, &edges);

        emit_hook!(&mut self._hooks, new_face2, {
            let edges = edges
                .iter()
                .map(|e| _make_split_edges("x", e.0, e.1, e.2.len()))
                .collect();

            let res = _split_face(&_make_face(d.outer_loop.len()), &edges);

            (&edges, &res)
        });

        d.output_loops.extend(faces);
    }
}

pub fn _make_face(n_pts: usize) -> Vec<String> {
    let mut res = vec![];
    for i in 0..n_pts {
        res.push(format!("b{}", i));
    }
    res
}

pub fn _make_split_edges(
    id: &str,
    start: usize,
    end: usize,
    n_pts: usize,
) -> (usize, usize, Vec<String>) {
    let mut res = vec![format!("b{}", start)];
    for i in 1..n_pts - 1 {
        res.push(format!("{}{}", id, i));
    }
    res.push(format!("b{}", end));

    (start, end, res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_face() {
        let base = _make_face(10);
        let segments = vec![
            _make_split_edges("a", 0, 9, 2),
            _make_split_edges("y", 0, 3, 4),
            _make_split_edges("x", 1, 2, 4),
            _make_split_edges("z", 4, 9, 4),
        ];

        let res = _split_face(&base, &segments);
        assert_eq!(
            &res[0],
            &vec!["b0", "y1", "y2", "b3", "b4", "z1", "z2", "b9"]
        );
        assert_eq!(
            &res[1],
            &vec!["b0", "b1", "x1", "x2", "b2", "b3", "y2", "y1"]
        );
        assert_eq!(&res[2], &vec!["b2", "x2", "x1", "b1"]);
        assert_eq!(
            &res[3],
            &vec!["b9", "z2", "z1", "b4", "b5", "b6", "b7", "b8"]
        );
    }
}
