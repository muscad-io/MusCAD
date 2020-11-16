use super::*;

pub type VertexRef<T> = Rc<Vertex<T>>;
pub type VertexRefWeak<T> = Weak<Vertex<T>>;

#[derive(Debug)]
pub struct Vertex<T> {
    position: [T; 3],
    id: entity::ID,
}

impl_id!(VERT_ID, Vertex<T>);

impl<T: Float> Vertex<T> {
    pub fn new() -> Self {
        Self::from_position(vector3d::zero())
    }
}

impl<T> Vertex<T> {
    pub fn from_position(position: [T; 3]) -> Self {
        Self {
            position,
            id: Self::_next_id(),
        }
    }

    pub fn as_position(&self) -> &[T; 3] {
        &self.position
    }
}

impl<T: Clone> Vertex<T> {
    pub fn to_position(&self) -> [T; 3] {
        self.position.to_owned()
    }
}

impl<T> Hash for Vertex<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_id().hash(state);
    }
}

impl<T> AsRef<[T]> for Vertex<T> {
    fn as_ref(&self) -> &[T] {
        &self.position
    }
}

impl<T> AsRef<[T]> for EntityHashKey<Rc<Vertex<T>>> {
    fn as_ref(&self) -> &[T] {
        self.ptr().as_ref().as_ref()
    }
}
