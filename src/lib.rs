#![feature(trait_alias)]
extern crate serde;

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::ops::{Shr, Shl, BitOr};

use serde::{Serialize, Deserialize};

pub trait LocCode = Eq + Hash + Copy + Shr + Shl + BitOr + Debug + From<u8> + From<<Self as Shr>::Output> + From<<Self as Shl>::Output> + From<<Self as BitOr>::Output>;

pub enum ErrorKind {
    CannotPlace(u8),
    OutsideTree,
    BelowThresold(usize, usize)
}


pub trait Subdivisable: Copy {

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
    pub childs: u8
}

impl<L: LocCode, D: Subdivisable> OctreeNode<L, D> {
    
    pub fn new(data: D, loc_code: L) -> Self {
        Self {
            data,
            loc_code,
            childs: 0
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Octree<L: Eq + Hash, D: Subdivisable> {
    content: HashMap<L, OctreeNode<L, D>>
}

impl<L, D> Octree<L, D>
    where L: LocCode,
          D: Subdivisable
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

    pub fn place_data(&mut self, data: D) -> Result<(), ErrorKind> {
        let mut datas = vec![data];
        let mut loc_codes = vec![L::from(1)];
        {
            let root = self.content.get(&L::from(1)).unwrap();
            if !data.fit_in(&root.data) {
                return Err(ErrorKind::OutsideTree)
            }
        };
        'data_loop: loop {
            if datas.len() == 0 {
                break Ok(())
            }
            let actual_data = datas.pop().unwrap();
            'node_loop: loop {
                let loc_code = loc_codes.pop().unwrap();
                let node = self.content.get(&loc_code);
                if let Some(octree_node) = node {
                    let mut divided_datas = actual_data.divide(&octree_node.data);
                    let vec_len = divided_datas.len();
                    if vec_len > 1 {
                        datas.extend(divided_datas);
                        loc_codes.extend(vec![loc_code; vec_len]);
                        continue 'data_loop;
                    }
                    let actual_data = divided_datas.pop().unwrap();
                    let place = actual_data.where_to_place(&octree_node.data);
                    if place > 8 {
                        break 'data_loop Err(ErrorKind::CannotPlace(place))
                    }
                    loc_codes.push(L::from(L::from(loc_code << L::from(3)) | L::from(place)));
                    continue 'node_loop;
                } else {
                    self.content.insert(loc_code, OctreeNode::new(actual_data, loc_code));
                    continue 'data_loop;
                }
            }
        }
    }
}
