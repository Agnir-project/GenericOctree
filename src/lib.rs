#![cfg_attr(nightly, feature(trait_alias))]

pub mod aabb;

pub mod loc_code;
pub mod node;
pub mod octree;

#[cfg(feature = "render")]
pub mod render;

#[cfg(feature = "vox")]
pub mod dot_vox;

pub use aabb::{Plane, AABB};
pub use loc_code::LocCode;
pub use octree::Octree;
