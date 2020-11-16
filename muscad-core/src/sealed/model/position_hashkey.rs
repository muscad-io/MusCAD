use super::*;

#[derive(Debug)]
pub struct PositionHashKey<T>(pub [T; 3]);

impl<T: Float> Hash for PositionHashKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash_val(&self.0[X]).hash(state);
        T::hash_val(&self.0[Y]).hash(state);
        T::hash_val(&self.0[Z]).hash(state);
    }
}

impl<T: Float> PartialEq for PositionHashKey<T> {
    fn eq(&self, other: &Self) -> bool {
        vector3d::eql(&self.0, &other.0)
    }
}

impl<T: Float> Eq for PositionHashKey<T> {}
