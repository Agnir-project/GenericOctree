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
    + Ord
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
    D: Clone + Copy + PartialEq + Debug,
{
    /// Load from voxel octree from files
    pub fn load_from_file<P: AsRef<Path>>(path_ref: P) -> Result<Self, &'static str> {
        let path = path_ref.as_ref();
        match path.extension() {
            Some(x) => match x.to_str() {
                #[cfg(feature = "dot_tree")]
                Some(".tree") => Err("At least we got there"),
                _ => Err("Cannot open format"),
            },
            None => Err("No format to open"),
        }
    }

    /// Create a new Octree
    pub fn new() -> Self {
        let content = HashMap::default();
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

    pub fn remove_node(&mut self, loc_code: &L) {
        self.content.remove(loc_code);
    }

    /// Get a mutable root node.
    pub fn get_mut_root(&mut self, node: &OctreeNode<L, D>) -> Option<&mut OctreeNode<L, D>> {
        let new_loc_code = node.loc_code >> L::from(3);
        self.content.get_mut(&new_loc_code)
    }

    /// Get a immutable root node.
    pub fn get_root(&self, node: &OctreeNode<L, D>) -> Option<&OctreeNode<L, D>> {
        let new_loc_code = node.loc_code >> L::from(3);
        self.content.get(&new_loc_code)
    }

    /// Merge an AABB into the tree
    pub fn merge(&mut self, aabb: AABB<L>, data: D) {
        let mut codes: Vec<L> = self
            .merge_inner(aabb, data, (0.5, 0.5, 0.5), 1, L::from(1))
            .into_iter()
            .collect();
        while !codes.is_empty() {
            codes.sort();
            codes.reverse();
            codes = codes
                .into_iter()
                .filter_map(|code| self.assemble(code))
                .filter(|code| code > &L::from(0))
                .collect::<HashSet<L>>()
                .into_iter()
                .collect();
        }
    }

    fn assemble(&mut self, code: L) -> Option<L> {
        let datas = (0_u8..8_u8)
            .map(|number| (code << L::from(3)) | L::from(number))
            .filter_map(|loc_code| self.lookup(&loc_code))
            .map(|node| node.data)
            .collect::<Vec<D>>();
        if datas.len() != 8 {
            None
        } else {
            let elem = datas[0];
            let is_same = datas
                .into_iter()
                .fold((true, elem), |acc, elem| (acc.0 && acc.1 == elem, acc.1))
                .0;
            if !is_same {
                None
            } else {
                (0_u8..8_u8)
                    .map(|number| (code << L::from(3)) | L::from(number))
                    .for_each(|code| self.remove_node(&code));
                self.insert(OctreeNode::new(elem, code));
                Some(code >> L::from(3))
            }
        }
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
        let mut fitting: HashSet<L> = blocks
            .iter()
            .filter(|aabb| aabb.fit_in(depth))
            .cloned()
            .map(|elem| {
                OctreeNode::new(
                    data,
                    (loc_code << L::from(3)) | (elem.orientation as u8).into(),
                )
            })
            .map(|elem| self.insert(elem))
            .map(|loc_code| loc_code >> L::from(3))
            .collect();
        let subdividables: HashSet<L> = blocks
            .into_iter()
            .filter(|aabb| !aabb.fit_in(depth))
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
        fitting.extend(subdividables);
        fitting
    }

    #[cfg(feature = "vox")]
    fn from_dotvox<U: AsRef<str>>(path: U) -> Result<Vec<Octree<L, u32>>, &'static str> {
        let vox = dot_vox::load(path.as_ref())?;
        let octrees: Vec<Octree<L, u32>> = crate::dot_vox::vox_to_octrees(vox);
        Ok(octrees)
    }
}
