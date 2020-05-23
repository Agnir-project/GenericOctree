#![cfg_attr(nightly, feature(trait_alias))]

pub mod aabb;

mod loc_code;
mod node;
mod octree;
mod orientation;

#[cfg(feature = "render")]
pub mod render;

#[cfg(feature = "vox")]
pub mod dot_vox;

pub use aabb::{Plane, AABB};
pub use loc_code::LocCode;
pub use node::OctreeNode;
pub use octree::Octree;
pub use orientation::Orientation;

#[cfg(feature = "render")]
pub use render::{Model, Vertex};
