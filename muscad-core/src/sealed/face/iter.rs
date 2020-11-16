use super::*;

fn _get_edge<T>(e: &EdgeRefWeak<T>) -> EdgeRef<T> {
    e.upgrade().unwrap()
}

/// Simple Edge Iterator. Modifying linked-list while looping may panic
fn _next_edge<T>(e: &EdgeRefWeak<T>) -> EdgeRefWeak<T> {
    Weak::clone(&e.upgrade().unwrap().borrow().next)
}

/// Simple Vertex Iterator. Modifying linked-list while looping may panic
fn _get_vert<T>(e: &EdgeRefWeak<T>) -> Rc<Vertex<T>> {
    Rc::clone(&e.upgrade().unwrap().borrow().vert.upgrade().unwrap())
}

impl<T> Face<T> {
    pub fn vertices_iter(&self) -> LinkedWeakIter<RefCell<Edge<T>>, Rc<Vertex<T>>> {
        LinkedWeakIter::build(Weak::clone(&self.edge), _next_edge, _get_vert)
    }

    pub fn edges_iter(&self) -> LinkedWeakIter<RefCell<Edge<T>>, EdgeRef<T>> {
        LinkedWeakIter::build(Weak::clone(&self.edge), _next_edge, _get_edge)
    }

    pub fn for_each_edgeref<F>(&self, mut f: F)
    where
        F: FnMut(EdgeRef<T>),
    {
        let mut cur = Weak::clone(&self.edge);

        loop {
            f(cur.upgrade().unwrap());

            cur = Weak::clone(&cur.upgrade().unwrap().borrow().next);

            if Weak::ptr_eq(&cur, &self.edge) {
                break;
            }
        }
    }

    pub fn for_each_edge<F>(&self, mut f: F)
    where
        F: FnMut(RefMut<Edge<T>>),
    {
        self.for_each_edgeref(|e| f(e.borrow_mut()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_vertices_iter() {
        let mut d = Model::new();
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [2.0, 0.0, 0.0];
        let v3 = [3.0, 0.0, 0.0];

        let f = d.add_face_unchecked(&[v1, v2, v3]);

        let pts: Vec<_> = f
            .borrow()
            .vertices_iter()
            .map(|v| v.to_position())
            .collect();

        assert_eq!(pts, [v1, v2, v3]);
    }

    #[test]
    fn test_face_edges_iter() {
        let mut d = Model::new();
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [2.0, 0.0, 0.0];
        let v3 = [3.0, 0.0, 0.0];

        let f = d.add_face_unchecked(&[v1, v2, v3]);

        let edges: Vec<_> = f.borrow().edges_iter().collect();

        let assert_same = |w: &Weak<_>, p| assert!(Rc::ptr_eq(&w.upgrade().unwrap(), p));

        assert_same(&edges[0].borrow().next, &edges[1]);
        assert_same(&edges[1].borrow().next, &edges[2]);
        assert_same(&edges[2].borrow().next, &edges[0]);
    }
}
