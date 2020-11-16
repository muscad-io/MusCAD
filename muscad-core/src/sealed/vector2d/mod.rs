use crate::Float;

use super::*;

pub trait AsV2d<T>: AsRef<[T]> {}
impl<V: AsRef<[T]>, T> AsV2d<T> for V {}

pub trait AsV2dMut<T>: AsMut<[T]> {}
impl<V: AsMut<[T]>, T> AsV2dMut<T> for V {}

type RtnType<T> = [T; 2];

pub use angle::*;
pub use cross::*;
pub use direction::*;
pub use distance::*;
pub use dot::*;
pub use eql::*;
pub use length::*;
pub use new::*;
pub use ops::*;

mod ops;
#[macro_use]
mod vec_eval;
mod angle;
mod cross;
mod direction;
mod distance;
mod dot;
mod eql;
mod length;
mod new;
