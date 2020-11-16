use super::*;

pub struct Point<'a, T, P> {
    pub is_ear: bool,
    pub i: usize,
    pub ptr: &'a P,
    _t: PhantomData<T>,
}

impl<'a, T, P> Debug for Point<'a, T, P>
where
    T: Float,
    P: AsV3d<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let p = self.ptr.as_ref();
        let x = &p[X];
        let y = &p[Y];
        let ear = if self.is_ear { "Ear" } else { "" };

        write!(f, "{}({:+.2?} {:+.2?})", ear, x, y)
    }
}

impl<'a, T, P> Point<'a, T, P> {
    pub fn build(i: usize, ptr: &'a P) -> Self {
        Self {
            is_ear: false,
            i,
            ptr,
            _t: PhantomData,
        }
    }
}
