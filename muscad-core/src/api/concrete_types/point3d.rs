use super::*;

#[repr(C)]
pub struct Point3d<T> {
    inner: [T; 3],
}

impl<T> AsRef<Point3d<T>> for Point3d<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsRef<[T]> for Point3d<T> {
    fn as_ref(&self) -> &[T] {
        &self.inner
    }
}

impl<T: Clone> From<&[T; 3]> for Point3d<T> {
    fn from(v: &[T; 3]) -> Self {
        Point3d {
            inner: (*v).clone(),
        }
    }
}

impl<T> From<[T; 3]> for Point3d<T> {
    fn from(v: [T; 3]) -> Self {
        Point3d { inner: v }
    }
}

impl<T: Debug> Debug for Point3d<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Point3d")
            .field(&self.inner[0])
            .field(&self.inner[1])
            .field(&self.inner[2])
            .finish()
    }
}
