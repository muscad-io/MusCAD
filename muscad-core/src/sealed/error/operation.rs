use super::*;

pub trait ErrStr {
    fn as_str() -> &'static str;
}

#[derive(ErrStr)]
pub struct OperationError<T, DATA = ()> {
    additional_data: DATA,
    _subtype: PhantomData<T>,
}

#[derive(ErrStr)]
pub struct NotEnoughPoints {}

pub trait HasNew {
    fn new() -> Self;
}

impl HasNew for () {
    fn new() -> () {
        ()
    }
}

impl<T, D> OperationError<T, D>
where
    D: HasNew,
{
    pub fn new() -> Self {
        Self {
            additional_data: D::new(),
            _subtype: PhantomData,
        }
    }
}

impl<T: ErrStr, D> Debug for OperationError<T, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", Self::as_str(), T::as_str())
    }
}
