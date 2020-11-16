use crate::Float;

use super::*;

pub trait AsV3d<T>: AsRef<[T]> {}
impl<V: AsRef<[T]>, T> AsV3d<T> for V {}

pub trait AsV3dMut<T>: AsMut<[T]> {}
impl<V: AsMut<[T]>, T> AsV3dMut<T> for V {}

type RtnType<T> = [T; 3];

pub use cross::*;
pub use direction::*;
pub use distance::*;
pub use dot::*;
pub use eql::*;
pub use length::*;
pub use new::*;
pub use ops::*;

mod cross;
mod direction;
mod distance;
mod dot;
mod eql;
mod length;
mod new;
mod ops;
#[macro_use]
mod vec_eval;
