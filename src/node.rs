#[cfg(feature = "serialize")]
extern crate serde;

use crate::octree::{LocCode, Data};


#[cfg(feature = "serialize")]
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct OctreeNode<L, D> {
    pub loc_code: L,
    pub data: D
}

impl<L, D> OctreeNode<L, D>
where
    L: LocCode,
    D: Data
{
    /// Create a new Node from it's Data and a LocCode.
    pub(crate) fn new(data: D, loc_code: L) -> Self {
        Self {
            data,
            loc_code
        }
    }

    pub(crate) fn transform<U>(self) -> OctreeNode<L, U>
        where U: From<D>
    {
        OctreeNode {
            loc_code: self.loc_code,
            data: U::from(self.data)
        }
    }

    pub(crate) fn transform_fn<U, F>(self, function: F) -> OctreeNode<L, U>
        where F: Fn(D) -> U
    {
        OctreeNode {
            loc_code: self.loc_code,
            data: function(self.data)
        }
    }

}
