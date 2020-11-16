use super::*;

/// Vertex Pool as a BiMap
#[derive(Debug)]
pub struct VertexPool<T> {
    pub store: Vec<VertexRef<T>>,
    pub index_map: SimpleHashMap<PositionHashKey<T>, usize>,
}

impl<T> VertexPool<T> {
    pub fn new() -> Self {
        Self {
            store: vec![],
            index_map: SimpleHashMap::default(),
        }
    }
}

impl<T: Float> VertexPool<T> {
    pub fn to_positions(&self) -> Vec<[T; 3]> {
        self.store.iter().map(|v| v.to_position()).collect()
    }

    pub fn get_index<U>(&self, pt: &U) -> Option<usize>
    where
        U: AsV3d<T>,
    {
        let pt = pt.as_ref();
        let k = PositionHashKey([pt[X].clone(), pt[Y].clone(), pt[Z].clone()]);

        if let Some(i) = self.index_map.get(&k) {
            Some(*i)
        } else {
            None
        }
    }

    pub fn add_or_get_vertex<U>(&mut self, pt: &U) -> &Rc<Vertex<T>>
    where
        U: AsV3d<T>,
    {
        let pt = pt.as_ref();
        let owned_pt = [pt[X].clone(), pt[Y].clone(), pt[Z].clone()];
        let k = PositionHashKey(owned_pt);

        if let Some(i) = self.index_map.get(&k) {
            &self.store[*i]
        } else {
            let v = Rc::new(Vertex::from_position(k.0.to_owned()));
            self.index_map.insert(k, self.store.len());
            self.store.push(v);
            self.store.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_pool_key_by_eql() {
        let mut p = VertexPool::new();
        let r1 = p.add_or_get_vertex(&[1.0, 0.46, 0.69]).clone();
        let r2 = p
            .add_or_get_vertex(&[1.0, 0.460000001, 0.689999995])
            .clone();
        assert!(Rc::ptr_eq(&r1, &r2));
    }
}
