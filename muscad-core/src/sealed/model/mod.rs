use super::*;

use entity_store::*;
use position_hashkey::*;
pub use vertex_pool::VertexPool;

mod entity_store;
mod position_hashkey;
mod vertex_pool;

#[cfg(feature = "serialization")]
mod serialize;

#[derive(Debug)]
pub struct Model<T> {
    vertex_pool: VertexPool<T>,
    face_store: Rc<RefCell<EntityStore<FaceRef<T>>>>,
    edge_store: Rc<RefCell<EntityStore<EdgeRef<T>>>>,
}

impl<T: Float> Model<T> {
    pub fn new() -> Self {
        Self {
            face_store: Rc::new(RefCell::new(EntityStore::new())),
            edge_store: Rc::new(RefCell::new(EntityStore::new())),
            vertex_pool: VertexPool::new(),
        }
    }

    pub fn n_faces(&self) -> usize {
        self.face_store.borrow().len()
    }

    pub fn for_each_faceref<F>(&self, f: F)
    where
        F: FnMut(&FaceRef<T>),
    {
        self.face_store.borrow_mut().for_each(f);
    }

    pub fn for_each_face<F>(&self, mut f: F)
    where
        F: FnMut(RefMut<Face<T>>),
    {
        self.face_store
            .borrow_mut()
            .for_each(|ptr| f(ptr.borrow_mut()));
    }

    pub fn to_points(&self) -> Vec<[T; 3]> {
        self.vertex_pool.to_positions()
    }

    pub fn extend_faces(&mut self, v: Vec<FaceRef<T>>) {
        self.face_store.borrow_mut().extend(v);
    }

    pub fn extend_edges(&mut self, v: Vec<EdgeRef<T>>) {
        self.edge_store.borrow_mut().extend(v);
    }

    pub fn get_point_index<U>(&self, pt: &U) -> Option<usize>
    where
        U: AsV3d<T>,
    {
        self.vertex_pool.get_index(pt)
    }

    pub fn add_or_get_vertex<U>(&mut self, pt: &U) -> &Rc<Vertex<T>>
    where
        U: AsV3d<T>,
    {
        self.vertex_pool.add_or_get_vertex(pt)
    }

    pub fn add_or_get_vertex_weak<U>(&mut self, pt: &U) -> Weak<Vertex<T>>
    where
        U: AsV3d<T>,
    {
        Rc::downgrade(self.vertex_pool.add_or_get_vertex(pt))
    }
}

impl<T: Float> Model<T> {
    pub fn add_face_unchecked<V, P>(&mut self, pts: &V) -> FaceRef<T>
    where
        V: AsRef<[P]>,
        P: AsV3d<T>,
    {
        let pts = pts.as_ref();
        let n = pts.len();

        let edges = pts
            .iter()
            .map(|p| {
                let mut e = Edge::new();
                e.vert = self.add_or_get_vertex_weak(p);
                Rc::new(RefCell::new(e))
            })
            .collect::<Vec<_>>();

        let face = Rc::new(RefCell::new(Face::new()));
        face.borrow_mut().edge = Rc::downgrade(&edges[0]);
        face.borrow_mut().n_edges = n;

        for i in 0..n {
            let prev_i = if i == 0 { n - 1 } else { i - 1 };
            let next_i = if i == n - 1 { 0 } else { i + 1 };
            edges[i].borrow_mut().next = Rc::downgrade(&edges[next_i]);
            edges[i].borrow_mut().prev = Rc::downgrade(&edges[prev_i]);
            edges[i].borrow_mut().face = Rc::downgrade(&face);
        }

        face.borrow_mut().calc();

        self.extend_faces(vec![Rc::clone(&face)]);
        self.extend_edges(edges);

        face
    }
}
