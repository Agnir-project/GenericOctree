use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, Shl, Shr};
use std::path::Path;

use crate::node::OctreeNode;

use crate::aabb::{Orientation, AABB};

pub trait LocCode = Eq
    + Hash
    + Copy
    + Debug
    + Shr<Output = Self>
    + Shl<Output = Self>
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + From<u8>
    + From<Self>
    + TryInto<u8>;

/// Octree's error kinds.
pub enum ErrorKind {
    CannotPlace(u8),
    OutsideTree,
    BelowThresold(usize, usize),
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Octree<L: Eq + Hash, D> {
    content: HashMap<L, OctreeNode<L, D>>,
}

impl<L, D> Octree<L, D>
where
    L: LocCode,
    D: Clone + Copy,
{
    /// Load from voxel octree from files
    pub fn load_from_file<P: AsRef<Path>>(path_ref: P) -> Result<Self, &'static str> {
        let path = path_ref.as_ref();
        match path.extension() {
            Some(x) => match x.to_str() {
                #[cfg(feature = "dot_tree")]
                Some(".tree") => {
                    println!("test");
                    Err("At least we got there")
                }
                _ => Err("Cannot open format"),
            },
            None => Err("No format to open"),
        }
    }

    /// Create a new Octree from an entry. It's necessary to initialize
    /// it with a entry because the tree lay on the first entry.
    pub fn new(data: D) -> Self {
        let mut content = HashMap::default();
        content.insert(L::from(1), OctreeNode::new(data, L::from(1)));
        Self { content }
    }

    /// Create an Octree with given pre-allocated space.
    pub fn with_capacity(size: usize, data: D) -> Self {
        let mut content = HashMap::with_capacity(size);
        content.insert(L::from(1), OctreeNode::new(data, L::from(1)));
        Self { content }
    }

    /// Return a tree node a node.
    pub fn lookup(&self, loc_code: &L) -> Option<&OctreeNode<L, D>> {
        self.content.get(loc_code)
    }

    /// Insert a tree node.
    pub fn insert(&mut self, node: OctreeNode<L, D>) -> L {
        let location = node.loc_code;
        self.content.insert(location, node);
        location
    }

    /// Get a mutable root node.
    pub fn get_mut_root(&mut self, node: &OctreeNode<L, D>) -> Option<&mut OctreeNode<L, D>> {
        let new_loc_code = L::from(node.loc_code >> L::from(3));
        self.content.get_mut(&new_loc_code)
    }

    /// Get a immutable root node.
    pub fn get_root(&self, node: &OctreeNode<L, D>) -> Option<&OctreeNode<L, D>> {
        let new_loc_code = L::from(node.loc_code >> L::from(3));
        self.content.get(&new_loc_code)
    }

    /// Merge an AABB into the tree
    pub fn merge(&mut self, aabb: AABB<L>, data: D) {
        let codes = self.merge_inner(aabb, data, (0.5, 0.5, 0.5), 1, L::from(1));
        println!("{:?}", codes);
    }

    /// Internal function for recursively merging AABB.
    /// Returns a HashSet containing all the node that are affected by the merging, not all new nodes
    /// These affected nodes can be scheduled to merge data outside here
    fn merge_inner(
        &mut self,
        aabb: AABB<L>,
        data: D,
        center: (f64, f64, f64),
        depth: u32,
        loc_code: L,
    ) -> HashSet<L> {
        let blocks = aabb.explode(center);
        let fitting: Vec<AABB<L>> = blocks
            .iter()
            .filter(|aabb| aabb.fit_in(depth))
            .cloned()
            .collect();
        println!("fitting = {:?} for center = {:?}", fitting, center);
        let fit_codes = fitting
            .into_iter()
            .map(|elem| {
                OctreeNode::new(
                    data,
                    (loc_code << L::from(3)) | (elem.orientation as u8).into(),
                )
            })
            .map(|elem| self.insert(elem))
            .map(|loc_code| loc_code >> L::from(3))
            .collect();
        let subdividables: Vec<AABB<L>> = blocks
            .into_iter()
            .filter(|aabb| !aabb.fit_in(depth))
            .collect();
        println!(
            "subdivisables = {:?} for center = {:?}",
            subdividables, center
        );
        let subdivisable_codes: HashSet<L> = subdividables
            .into_iter()
            .map(|aabb| {
                let new_loc_code = (loc_code << L::from(3)) | (aabb.orientation as u8).into();
                let new_center = aabb.orientation.make_new_center(new_loc_code, center);
                self.merge_inner(
                    aabb.with_orientation(Orientation::N),
                    data,
                    new_center,
                    depth + 1,
                    new_loc_code,
                )
            })
            .flatten()
            .collect();
        fit_codes
    }
}
