#![feature(trait_alias)]

pub mod aabb;
#[cfg(feature = "vox")]
pub mod dot_vox;
pub mod node;
pub mod octree;

pub use aabb::{Plane, AABB};
pub use octree::{LocCode, Octree};
