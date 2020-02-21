#[cfg(feature = "serialize")]
extern crate serde;

use crate::octree::{LocCode};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct OctreeNode<L, D> {
    pub loc_code: L,
    pub data: D,
    pub childs: u8,
}

impl<L: LocCode, D> OctreeNode<L, D> {
    /// Create a new Node from it's Data and a LocCode.
    pub fn new(data: D, loc_code: L) -> Self {
        Self {
            data,
            loc_code,
            childs: 0,
        }
    }

    /// Add a child to the actual Node.
    pub fn add_child(&mut self, child: L) {
        let value: u8;
        unsafe {
            let ptr = std::mem::transmute::<&L, *const u8>(&child);
            ptr.offset(std::mem::size_of::<L>() as isize);
            value = *ptr;
        }
        match value {
            0 => self.childs |= 0b00000001,
            1 => self.childs |= 0b00000010,
            2 => self.childs |= 0b00000100,
            3 => self.childs |= 0b00001000,
            4 => self.childs |= 0b00010000,
            5 => self.childs |= 0b00100000,
            6 => self.childs |= 0b01000000,
            7 => self.childs |= 0b10000000,
            _ => (),
        }
    }
}
