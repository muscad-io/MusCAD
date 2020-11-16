use super::*;

pub struct EntityHashKey<P> {
    ptr: P,
}

impl<P> EntityHashKey<P> {
    pub fn to_inner(self) -> P {
        self.ptr
    }

    pub fn ptr(&self) -> &P {
        &self.ptr
    }
}

impl<P: Debug + AsEntityHashKey> Debug for EntityHashKey<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{} | {:?}>", self.ptr.to_id(), self.ptr)
    }
}

impl<P: Clone> Clone for EntityHashKey<P> {
    fn clone(&self) -> Self {
        Self {
            ptr: P::clone(&self.ptr),
        }
    }
}

impl<P: Clone> EntityHashKey<P> {
    pub fn from_rc(ptr: &P) -> Self {
        Self { ptr: P::clone(ptr) }
    }

    pub fn to_rc(&self) -> P {
        P::clone(&self.ptr)
    }
}

pub trait AsEntityHashKey: HasUniqueId {
    fn to_key(&self) -> EntityHashKey<Self>
    where
        Self: Clone,
    {
        EntityHashKey::from_rc(self)
    }
}

impl<T: AsEntityHashKey> Hash for EntityHashKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.to_id().hash(state);
    }
}

impl<T: AsEntityHashKey> PartialEq for EntityHashKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr.to_id() == other.ptr.to_id()
    }
}

impl<T: AsEntityHashKey> Eq for EntityHashKey<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut m = Model::new();
        let pts = [[10.0, 0.0, 0.0], [10.0, 1.0, 0.0], [10.0, 0.0, 1.0]];
        let ptr = m.add_face_unchecked(&pts);
        let ptr2 = m.add_face_unchecked(&pts);

        let mut s = SimpleHashSet::default();
        s.insert(ptr.to_key());
        s.insert(ptr.to_key());

        let mut s2 = SimpleHashSet::default();
        s2.insert((ptr.to_key(), ptr2.to_key()));
        s2.insert((ptr2.to_key(), ptr.to_key()));

        assert!(!s.contains(&ptr2.to_key()));
        assert_eq!(s.len(), 1);

        assert_eq!(s2.len(), 2);
        assert!(s2.contains(&(ptr2.to_key(), ptr.to_key())));
        assert!(!s2.contains(&(ptr2.to_key(), ptr2.to_key())));
    }
}
