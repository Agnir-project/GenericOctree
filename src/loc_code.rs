use std::{
    fmt::Debug,
    hash::Hash,
    ops::{BitOr, Shl, Shr},
};

use crate::Orientation;

pub trait LocCode:
    Copy
    + Clone
    + Ord
    + PartialOrd
    + Eq
    + PartialEq
    + Debug
    + Hash
    + Send
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitOr<Output = Self>
    + From<u8>
    + BitOr<Orientation, Output = Self>
{
    /// Useful for many puproses
    fn zero() -> Self;

    /// Useful to get root node
    fn root() -> Self;

    /// Useful for binary trees
    fn one() -> Self;

    /// Useful for quad-trees
    fn two() -> Self;

    // Useful for oct-trees
    fn three() -> Self;

    fn get_level(self) -> u32;

    fn get_center_u32(self) -> (u32, u32, u32);
}

macro_rules! impl_loc_code_num {
    ($ty:ident) => {
        impl BitOr<Orientation> for $ty {
            type Output = Self;

            fn bitor(self, rhs: Orientation) -> Self {
                match rhs {
                    Orientation::LBU => self | 0,
                    Orientation::LFU => self | 1,
                    Orientation::LFD => self | 2,
                    Orientation::LBD => self | 3,
                    Orientation::RBD => self | 4,
                    Orientation::RFD => self | 5,
                    Orientation::RFU => self | 6,
                    Orientation::RBU => self | 7,
                    _ => self | 8,
                }
            }
        }

        impl LocCode for $ty {
            fn root() -> Self {
                Self::one()
            }

            fn one() -> Self {
                1 as Self
            }

            fn zero() -> Self {
                1 as Self
            }

            fn two() -> Self {
                2 as Self
            }

            fn three() -> Self {
                3 as Self
            }

            fn get_level(mut self) -> u32 {
                let mut level = 1;
                while self != 1 {
                    self >>= 3;
                    level += 1;
                }
                level as u32
            }

            fn get_center_u32(self) -> (u32, u32, u32) {
                if self == 1 {
                    return (i32::MAX as u32, i32::MAX as u32, i32::MAX as u32);
                }
                let offset = 2u32.pow(32 - self.get_level());
                let mask = Self::MAX ^ 7;
                let center = (self >> 3).get_center_u32();
                match (self ^ mask) & !mask {
                    0 => (center.0 - offset, center.1 + offset, center.2 - offset),
                    1 => (center.0 - offset, center.1 + offset, center.2 + offset),
                    2 => (center.0 - offset, center.1 - offset, center.2 + offset),
                    3 => (center.0 - offset, center.1 - offset, center.2 - offset),
                    4 => (center.0 + offset, center.1 - offset, center.2 - offset),
                    5 => (center.0 + offset, center.1 - offset, center.2 + offset),
                    6 => (center.0 + offset, center.1 + offset, center.2 + offset),
                    7 => (center.0 + offset, center.1 + offset, center.2 - offset),
                    _ => (i32::MAX as u32, i32::MAX as u32, i32::MAX as u32),
                }
            }
        }
    };
}

impl_loc_code_num!(u8);
impl_loc_code_num!(i16);
impl_loc_code_num!(u16);
impl_loc_code_num!(i32);
impl_loc_code_num!(u32);
impl_loc_code_num!(i64);
impl_loc_code_num!(u64);
