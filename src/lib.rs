#![feature(trait_alias)]
extern crate serde;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitOr, Shl, Shr, BitAnd};
use std::convert::TryInto;

use serde::{Deserialize, Serialize};

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

pub enum ErrorKind {
    CannotPlace(u8),
    OutsideTree,
    BelowThresold(usize, usize),
}

pub trait Subdivisable: Copy + Debug {
    type CenterType;

    fn get_center(&self) -> Self::CenterType;

    fn where_to_place(&self, rhs: &Self) -> u8;

    fn fit_in(&self, rhs: &Self) -> bool;

    fn divide(self, rhs: &Self) -> Vec<Self>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OctreeNode<L, D: Subdivisable> {
    pub loc_code: L,
    pub data: D,
    pub childs: u8,
}

impl<L: LocCode, D: Subdivisable> OctreeNode<L, D> {
    pub fn new(data: D, loc_code: L) -> Self {
        Self {
            data,
            loc_code,
            childs: 0,
        }
    }

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
            _ => ()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Octree<L: Eq + Hash, D: Subdivisable> {
    content: HashMap<L, OctreeNode<L, D>>,
}

impl<L, D> Octree<L, D>
where
    L: LocCode,
    D: Subdivisable,
{
    /// Create a new Octree from an entry. It's necessary to initialize
    /// it with a entry because the tree lay on the first entry.
    pub fn new(data: D) -> Self {
        let mut content = HashMap::default();
        content.insert(L::from(1), OctreeNode::new(data, L::from(1)));
        Self { content }
    }

    pub fn with_capacity(size: usize, data: D) -> Self {
        let mut content = HashMap::with_capacity(size);
        content.insert(L::from(1), OctreeNode::new(data, L::from(1)));
        Self { content }
    }

    pub fn lookup(&self, loc_code: &L) -> Option<&OctreeNode<L, D>> {
        self.content.get(loc_code)
    }

    pub fn insert(&mut self, node: OctreeNode<L, D>) {
        self.content.insert(node.loc_code, node);
    }

    pub fn get_mut_root(&mut self, node: &OctreeNode<L, D>) -> Option<&mut OctreeNode<L, D>> {
        let new_loc_code = L::from(node.loc_code >> L::from(3));
        self.content.get_mut(&new_loc_code)
    }

    fn check_subdivise(
        loc_codes: &mut Vec<L>,
        datas: &mut Vec<D>,
        divided_datas: &Vec<D>,
        loc_code: L,
    ) -> bool {
        let vec_len = divided_datas.len();

        if vec_len > 1 {
            datas.extend(divided_datas);
            loc_codes.extend(vec![loc_code; vec_len]);
            true
        } else {
            false
        }
    }

    fn compute_loc(
        divided_datas: &mut Vec<D>,
        loc_code: L,
        octree_node: &OctreeNode<L, D>,
    ) -> Result<L, ErrorKind> {
        let entry = divided_datas.pop().unwrap();
        let place = entry.where_to_place(&octree_node.data);

        if place > 8 {
            Err(ErrorKind::CannotPlace(place))
        } else {
            Ok(L::from(L::from(loc_code << L::from(3)) | L::from(place)))
        }
    }

    /// Iterate through datas, add to input vector subdivisable data
    fn insert_subdivise(
        &mut self,
        mut loc_codes: &mut Vec<L>,
        mut datas: &mut Vec<D>,
        entry: D,
    ) -> Result<(), ErrorKind> {
        loop {
            let loc_code = loc_codes.pop().unwrap();

            if let Some(octree_node) = self.content.get(&loc_code) {
                let mut divided_datas = entry.divide(&octree_node.data);

                if !Self::check_subdivise(&mut loc_codes, &mut datas, &divided_datas, loc_code) {
                    loc_codes.push(Self::compute_loc(
                        &mut divided_datas,
                        loc_code,
                        octree_node,
                    )?);
                }
            } else {
                let node = OctreeNode::new(entry, loc_code);
                let root_node = self.get_mut_root(&node).unwrap();
                root_node.add_child(loc_code & L::from(0x7));
                self.content.insert(loc_code, node);
                break Ok(());
            }
        }
    }

    fn is_insertable(&self, data: &D) -> Result<(), ErrorKind> {
        let root = self.content.get(&L::from(1)).unwrap();

        if !data.fit_in(&root.data) {
            Err(ErrorKind::OutsideTree)
        } else {
            Ok(())
        }
    }

    pub fn place_data(&mut self, data: D) -> Result<(), ErrorKind> {
        self.is_insertable(&data)?;
        let mut datas = vec![data];
        let mut loc_codes = vec![L::from(1)];

        while let Some(entry) = datas.pop() {
            self.insert_subdivise(&mut loc_codes, &mut datas, entry)?
        }
        Ok(())
    }
}
