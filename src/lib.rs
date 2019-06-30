use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::ops::{Shr, Shl, BitOr};

pub trait Subdivisable {

    fn where_to_place(&self, rhs: &Self) -> u8;
}

#[derive(Debug)]
pub struct OctreeNode<L, D: Subdivisable> {
    pub loc_code: L,
    pub data: D,
    pub childs: u8
}

impl<L, D: Subdivisable> OctreeNode<L, D> {
    
    #[warn(dead_code)]
    pub fn new(data: D, loc_code: L) -> Self {
        Self {
            data,
            loc_code,
            childs: 0
        }
    }
    
}

#[derive(Debug)]
pub struct Octree<L: Eq + Hash, D: Subdivisable> {
    content: HashMap<L, OctreeNode<L, D>>
}

impl<L, D> Octree<L, D>
    where L: Hash + Eq + Copy + Shr + Shl + From<u8> + From<<L as Shr>::Output> + From<<L as Shl>::Output> + BitOr + From<<L as BitOr>::Output> + Debug,
          D: Subdivisable + Copy
{
    /// Create a new Octree from an entry. It's necessary to initialize
    /// it with a entry because the tree lay on the first entry.
    pub fn new(data: D) -> Self {
        let mut content = HashMap::default();
        content.insert(L::from(1), OctreeNode::new(data, L::from(1)));
        Self {
            content
        }
    }

    pub fn with_capacity(size: usize, data: D) -> Self {
        let mut content = HashMap::with_capacity(size);
        content.insert(L::from(1), OctreeNode::new(data, L::from(1)));
        Self {
            content
        }
    }

    pub fn lookup(&self, loc_code: &L) -> Option<&OctreeNode<L, D>> {
        self.content.get(loc_code)
    }

    pub fn insert(&mut self, node: OctreeNode<L, D>) {
        self.content.insert(node.loc_code, node);
    }

    pub fn get_root(&self, node: OctreeNode<L, D>) -> Option<&OctreeNode<L, D>> {
        let new_loc_code = L::from(node.loc_code >> L::from(3));
        self.content.get(&new_loc_code)
    }

    pub fn place_data(&mut self, data: D) {
        let mut loc_code = L::from(1);
        loop {
            let node = self.content.get(&loc_code);
            if let Some(octree_node) = node {
                let place = data.where_to_place(&octree_node.data);
                loc_code = L::from(L::from(loc_code << L::from(3)) | L::from(place));
            } else {
                self.content.insert(loc_code, OctreeNode::new(data, loc_code));
                break
            }
        }
    }
}
