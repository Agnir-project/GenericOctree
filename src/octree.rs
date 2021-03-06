use hashbrown::HashMap;
use hashbrown::HashSet;

use crate::{LocCode, OctreeNode, Orientation, AABB};
use rayon::prelude::*;

#[cfg(feature = "serialize")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg(feature = "serialize")]
use flate2::Compression;

#[cfg(feature = "serialize")]
use flate2::write::{ZlibDecoder, ZlibEncoder};

#[cfg(feature = "serialize")]
use std::{io::prelude::*, path::Path};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Octree<L: LocCode, D: Send + Sync> {
    pub content: HashMap<L, OctreeNode<D>>,
    max_depth: u32,
}

#[cfg(feature = "dot_tree")]
impl<L, D> Octree<L, D>
where
    L: LocCode + Serialize + DeserializeOwned,
    D: Send + Sync + Serialize + DeserializeOwned,
{
    /// Load from voxel octree from files
    /// TODO: Add better error management
    pub fn load_from_file<P: AsRef<Path>>(path_ref: P) -> Result<Self, std::io::Error> {
        let path = path_ref.as_ref();
        match path.extension() {
            Some(x) => match x.to_str() {
                #[cfg(feature = "dot_tree")]
                Some("tree") => {
                    let file = std::fs::File::open(path)?;
                    let contents: Vec<u8> = file.bytes().filter_map(Result::ok).collect();
                    let mut decoder = ZlibDecoder::new(Vec::new());
                    decoder.write_all(&contents)?;
                    let contents = decoder.finish()?;
                    let tree = bincode::deserialize_from::<&[u8], Self>(&contents).unwrap();
                    Ok(tree)
                }
                _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
            },
            None => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }

    /// Save octree to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path_ref: P) -> Result<(), std::io::Error> {
        let path = path_ref.as_ref();
        let binary = bincode::serialize(self).unwrap();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&binary)?;
        std::fs::write(path, encoder.finish()?)
    }
}

impl<'a, L, D> Octree<L, D>
where
    L: LocCode,
    D: Send + Sync,
{
    /// Get the size of an octree
    pub fn size(&self) -> usize {
        self.content.len()
    }
}

impl<'a, T, D> Octree<T, D>
where
    T: LocCode,
    D: Copy + PartialEq + Send + Sync,
{
    /// Create a new Octree
    pub fn new(max_depth: u32) -> Self {
        let content = HashMap::default();
        Self { content, max_depth }
    }

    /// Create an Octree with given pre-allocated space.
    pub fn with_capacity(max_depth: u32, size: usize) -> Self {
        let content = HashMap::with_capacity(size);
        Self { content, max_depth }
    }

    pub fn depth(&self) -> u32 {
        let keys = self.content.keys();
        keys.max().unwrap_or(&T::root()).get_level()
    }

    /// Return a tree node a node.
    pub fn lookup(&self, loc_code: T) -> Option<&OctreeNode<D>> {
        self.content.get(&loc_code)
    }

    /// Insert a tree node.
    pub fn insert(&mut self, location: T, node: OctreeNode<D>) -> T {
        self.content.insert(location, node);
        location
    }

    pub fn remove_node(&mut self, loc_code: T) {
        self.content.remove(&loc_code);
    }

    /// Merge an AABB into the tree
    pub fn merge(&mut self, aabb: AABB, data: D) {
        let mut codes: Vec<T> = self
            .merge_inner(aabb, data, (0.5, 0.5, 0.5), 1, T::root())
            .into_iter()
            .collect();
        while !codes.is_empty() {
            codes.sort();
            codes.reverse();
            codes = codes
                .into_iter()
                .filter_map(|code| self.assemble(code))
                .filter(|code| *code > T::zero())
                .collect::<HashSet<T>>()
                .into_iter()
                .collect();
        }
    }

    fn assemble(&mut self, code: T) -> Option<T> {
        let datas = (0_u8..8_u8)
            .map(|number| (code << T::three()) | T::from(number))
            .filter_map(|loc_code| self.lookup(loc_code))
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
                    .map(|number| (code << T::three()) | T::from(number))
                    .for_each(|code| self.remove_node(code));
                self.insert(code, OctreeNode::new(elem));
                Some(code >> T::three())
            }
        }
    }

    /// Transform an Octree of data D into an Octree of data U, provided that
    /// U implement From<D>
    pub fn transform<U: From<D> + Send + Sync>(self) -> Octree<T, U> {
        Octree {
            content: self
                .content
                .into_par_iter()
                .map(|(loc_code, data)| (loc_code, data.transform::<U>()))
                .collect::<HashMap<T, OctreeNode<U>>>(),
            max_depth: self.max_depth,
        }
    }

    /// tree.transform_fn(Rgb::from_hex);
    pub fn transform_fn<U: Send + Sync, F: Fn(D) -> U + Sync>(self, function: F) -> Octree<T, U> {
        Octree {
            content: self
                .content
                .into_par_iter()
                .map(|(loc_code, data)| (loc_code, data.transform_fn(&function)))
                .collect::<HashMap<T, OctreeNode<U>>>(),
            max_depth: self.max_depth,
        }
    }

    /// tree.transform_fn(Rgb::from_hex);
    pub fn transform_nodes_fn<U: Send + Sync, F: Fn(T, OctreeNode<D>) -> OctreeNode<U> + Sync>(
        self,
        function: F,
    ) -> Octree<T, U> {
        Octree {
            content: self
                .content
                .into_par_iter()
                .map(|(loc_code, data)| (loc_code, function(loc_code, data)))
                .collect::<HashMap<T, OctreeNode<U>>>(),
            max_depth: self.max_depth,
        }
    }

    /// Internal function for recursively merging AABB.
    /// Returns a HashSet containing all the node that are affected by the merging, not all new nodes
    /// These affected nodes can be scheduled to merge data outside here
    fn merge_inner(
        &mut self,
        aabb: AABB,
        data: D,
        center: (f64, f64, f64),
        depth: u32,
        loc_code: T,
    ) -> HashSet<T> {
        let blocks = aabb.explode(center);
        let max_depth = self.max_depth;

        let (fitting, subdivisables): (Vec<AABB>, Vec<AABB>) = blocks
            .into_iter()
            .partition(|aabb| aabb.fit_in(depth, max_depth));

        let mut codes: Vec<T> = fitting
            .into_iter()
            .map(|elem| {
                self.insert(
                    loc_code << T::three() | elem.orientation,
                    OctreeNode::new(data),
                )
            })
            .map(|loc_code| loc_code >> T::three())
            .collect();

        codes.extend(if depth != self.max_depth {
            subdivisables
                .into_iter()
                .map(|aabb| {
                    let new_loc_code = (loc_code << T::three()) | aabb.orientation;
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
                .collect()
        } else {
            Vec::<T>::default()
        });

        codes.into_iter().collect::<HashSet<T>>()
    }
}

impl<'a, L, D> Octree<L, D>
where
    L: LocCode,
    D: Send + Sync,
{
    #[cfg(feature = "vox")]
    pub fn from_dotvox<U: AsRef<str>>(
        path: U,
        max_depth: u32,
        optimal: crate::dot_vox::ConversionType,
    ) -> Result<Vec<Octree<L, u32>>, &'static str> {
        let vox = dot_vox::load(path.as_ref())?;
        let octrees: Vec<Octree<L, u32>> = crate::dot_vox::vox_to_octrees(vox, max_depth, optimal);
        Ok(octrees)
    }
}
