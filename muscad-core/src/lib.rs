use std::{
    cell::{RefCell, RefMut},
    cmp::Ordering,
    convert::TryInto,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::{Rc, Weak},
};

pub use api::*;

pub use sealed::BooleanMode;
pub use sealed::BooleanSolver;

mod api;

//TODO: export sealed only in dev-mode
#[allow(unused)]
pub mod sealed;
