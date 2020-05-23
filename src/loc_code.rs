use std::{fmt::Debug, ops::{BitOr, BitXor, BitAnd, Shl, Shr}};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct LocCode<T>(pub T) where T: Copy + Debug;

impl<T> From<u8> for LocCode<T>
where
    T: From<u8> + Copy + Debug,
{
    fn from(val: u8) -> Self {
        Self(val.into())
    }
}

impl<T> From<u16> for LocCode<T>
where
    T: From<u16> + Copy + Debug,
{
    fn from(val: u16) -> Self {
        Self(val.into())
    }
}

impl<T> From<u32> for LocCode<T>
where
    T: From<u32> + Copy + Debug,
{
    fn from(val: u32) -> Self {
        Self(val.into())
    }
}

impl<T> From<u64> for LocCode<T>
where
    T: From<u64> + Copy + Debug,
{
    fn from(val: u64) -> Self {
        Self(val.into())
    }
}

impl<T, D> Shl<D> for LocCode<T>
where
    T: Shl<Output = T> + From<D> + Copy + Debug,
{
    type Output = Self;

    fn shl(self, rhs: D) -> Self::Output {
        let LocCode(lhs) = self;
        LocCode(lhs << rhs.into())
    }
}

impl<T, D> Shr<D> for LocCode<T>
where
    T: Shr<Output = T> + From<D> + Copy + Debug,
{
    type Output = Self;

    fn shr(self, rhs: D) -> Self::Output {
        let LocCode(lhs) = self;
        LocCode(lhs >> rhs.into())
    }
}

impl<T, D> BitXor<D> for LocCode<T>
where
    T: BitXor<Output = T> + From<D> + Copy + Debug,
{
    type Output = Self;

    fn bitxor(self, rhs: D) -> Self::Output {
        let LocCode(lhs) = self;
        LocCode(lhs ^ rhs.into())
    }
}


impl<T, D> BitAnd<D> for LocCode<T>
where
    T: BitAnd<Output = T> + From<D> + Copy + Debug,
{
    type Output = Self;

    fn bitand(self, rhs: D) -> Self::Output {
        let LocCode(lhs) = self;
        LocCode(lhs & rhs.into())
    }
}


impl<T, D> BitOr<D> for LocCode<T>
where
    T: BitOr<Output = T> + From<D> + Copy + Debug,
{
    type Output = Self;

    fn bitor(self, rhs: D) -> Self::Output {
        let LocCode(lhs) = self;
        LocCode(lhs | rhs.into())
    }
}

impl<T> LocCode<T>
where
    T: Eq + From<u8> + Shr<Output = T> + Copy + Debug,
{
    pub fn get_level(&self) -> u32 {
        let mut temp = self.clone();
        let mut level = 1;
        while temp != 1.into() {
            temp = temp >> 3;
            level += 1;
        }
        level
    }
}

impl<T> LocCode<T>
where
    T: Eq + From<u8> + From<u64> + Shr<Output = T> + Shl<Output = T> + BitXor<Output = T> + BitAnd<Output = T> + Copy + Debug,
{
    pub fn get_center_u32(self) -> (u32, u32, u32) {
        if self == LocCode(1u8.into()) {
            return ((2 as u32).pow(31), (2 as u32).pow(31), (2 as u32).pow(31));
        }
        let offset = (2 as u32).pow(32 - self.get_level());
        let mask = u64::max_value() ^ 7u64;
        let center = (self >> 3u8).get_center_u32();
        let value = (self ^ mask) & !mask;
        if value == 0u8.into() {
            (center.0 - offset, center.1 + offset, center.2 - offset)
        } else if value == 1u8.into() {
            (center.0 - offset, center.1 + offset, center.2 + offset)
        } else if value == 2u8.into() {
            (center.0 - offset, center.1 - offset, center.2 + offset)
        } else if value == 3u8.into() {
            (center.0 - offset, center.1 - offset, center.2 - offset)
        } else if value == 4u8.into() {
            (center.0 + offset, center.1 - offset, center.2 - offset)
        } else if value == 5u8.into() {
            (center.0 + offset, center.1 - offset, center.2 + offset)
        } else if value == 6u8.into() {
            (center.0 + offset, center.1 + offset, center.2 + offset)
        } else {
            (center.0 + offset, center.1 + offset, center.2 - offset)
        }
    }
}
