use super::*;

pub struct EntityStore<T> {
    inner: Vec<T>,
}

impl<T> EntityStore<T> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn extend(&mut self, v: Vec<T>) {
        self.inner.extend(v);
    }

    pub fn for_each(&mut self, f: impl FnMut(&T)) {
        self.inner.iter().for_each(f);
    }
}

impl<T> Debug for EntityStore<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<store>")
    }
}
