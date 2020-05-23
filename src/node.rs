#[cfg(feature = "serialize")]
extern crate serde;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct OctreeNode<D> {
    pub data: D,
}

impl<D> OctreeNode<D> {
    /// Create a new Node from it's Data and a LocCode.
    pub(crate) fn new(data: D) -> Self {
        Self { data }
    }

    pub(crate) fn transform<U>(self) -> OctreeNode<U>
    where
        U: From<D>,
    {
        OctreeNode {
            data: U::from(self.data),
        }
    }

    pub(crate) fn transform_fn<U, F>(self, function: F) -> OctreeNode<U>
    where
        F: Fn(D) -> U,
    {
        OctreeNode {
            data: function(self.data),
        }
    }
}
