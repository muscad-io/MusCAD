use super::*;

use consts::*;

use muscad_macro::ErrStr;

pub use boolean::{BooleanMode, BooleanSolver};
pub use bounding_box::BoundingBox;
pub use edge::{Edge, EdgeRef, EdgeRefWeak};
pub use entity::{AsEntityHashKey, EntityHashKey, HasUniqueId};
pub use error::*;
pub use face::{Face, FaceRef, FaceRefWeak};
pub use float::Float;
pub use model::{Model, VertexPool};
pub use plane::AsPlane;
pub use segment::{AsSegment, Segment, SegmentPointPoint, SegmentPointVector};
pub use util::{
    each_cons_iter::{to_each_cons_3_cycle_iter, to_each_cons_3_iter},
    rc_iter::LinkedWeakIter,
    simple_hash::{NaiveHasher, SimpleHashMap, SimpleHashSet},
};
pub use vector2d::{AsV2d, AsV2dMut};
pub use vector3d::{AsV3d, AsV3dMut};
pub use vertex::{Vertex, VertexRef, VertexRefWeak};

#[macro_use]
mod boolean;
mod bounding_box;
mod consts;
mod edge;
pub mod entity;
mod face;
mod float;
mod model;
pub mod plane;
pub mod poly2d;
pub mod project2d;
pub mod segment;
mod util;
#[macro_use]
pub mod vector2d;
#[macro_use]
pub mod vector3d;
pub mod check;
mod error;
mod vertex;
