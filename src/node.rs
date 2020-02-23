#[cfg(feature = "serialize")]
extern crate serde;

use crate::octree::LocCode;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct OctreeNode<L, D> {
    pub loc_code: L,
    pub data: D
}

impl<L: LocCode, D> OctreeNode<L, D> {
    /// Create a new Node from it's Data and a LocCode.
    pub fn new(data: D, loc_code: L) -> Self {
        Self {
            data,
            loc_code
        }
    }

}
