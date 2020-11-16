use super::*;

#[allow(unused)]
pub struct Face<T> {
    inner: sealed::FaceRefWeak<T>,
}

impl<T: Float> Face<T> {
    pub(crate) fn from_rc(f: &sealed::FaceRef<T>) -> Self {
        Self {
            inner: Rc::downgrade(&f),
        }
    }
}
