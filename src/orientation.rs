use crate::LocCode;
use std::ops::BitOr;

// TODO: Make u8 not rely on enum position
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    LBU,
    LFU,
    LFD,
    LBD,
    RBD,
    RFD,
    RFU,
    RBU,
    N,
    L,
    R,
    F,
    B,
    U,
    D,
    LF,
    LB,
    RF,
    RB,
    FU,
    FD,
    BU,
    BD,
    LU,
    LD,
    RU,
    RD,
}

type Center = (f64, f64, f64);

impl Orientation {
    pub fn make_new_center<L>(self, loc_code: L, center: Center) -> Center
    where
        L: LocCode,
    {
        let offset: f64 = 1.0 / ((2 as u32).pow(loc_code.get_level()) as f64);
        match self {
            Self::LBU => (center.0 - offset, center.1 + offset, center.2 - offset),
            Self::LFU => (center.0 - offset, center.1 + offset, center.2 + offset),
            Self::LFD => (center.0 - offset, center.1 - offset, center.2 + offset),
            Self::LBD => (center.0 - offset, center.1 - offset, center.2 - offset),
            Self::RBD => (center.0 + offset, center.1 - offset, center.2 - offset),
            Self::RFD => (center.0 + offset, center.1 - offset, center.2 + offset),
            Self::RFU => (center.0 + offset, center.1 + offset, center.2 + offset),
            Self::RBU => (center.0 + offset, center.1 + offset, center.2 - offset),
            _ => center,
        }
    }
}

/// TODO: Make this TryInto
impl Into<u8> for Orientation {
    fn into(self) -> u8 {
        match self {
            Self::LBU => 0,
            Self::LFU => 1,
            Self::LFD => 2,
            Self::LBD => 3,
            Self::RBD => 4,
            Self::RFD => 5,
            Self::RFU => 6,
            Self::RBU => 7,
            _ => 8,
        }
    }
}

impl BitOr for Orientation {
    type Output = Self;

    fn bitor(self, rh: Self) -> Self {
        match (&self, &rh) {
            // X | Z
            (Self::L, Self::F) => Self::LF,
            (Self::L, Self::B) => Self::LB,
            (Self::R, Self::F) => Self::RF,
            (Self::R, Self::B) => Self::RB,
            (Self::F, Self::L) => Self::LF,
            (Self::F, Self::R) => Self::RF,
            (Self::B, Self::L) => Self::LB,
            (Self::B, Self::R) => Self::RB,

            // Z | Y
            (Self::F, Self::U) => Self::FU,
            (Self::F, Self::D) => Self::FD,
            (Self::B, Self::U) => Self::BU,
            (Self::B, Self::D) => Self::BD,
            (Self::U, Self::F) => Self::FU,
            (Self::U, Self::B) => Self::BU,
            (Self::D, Self::F) => Self::FD,
            (Self::D, Self::B) => Self::BD,

            // Y | X
            (Self::L, Self::U) => Self::LU,
            (Self::L, Self::D) => Self::LD,
            (Self::R, Self::U) => Self::RU,
            (Self::R, Self::D) => Self::RD,
            (Self::U, Self::L) => Self::LU,
            (Self::U, Self::R) => Self::RU,
            (Self::D, Self::L) => Self::LD,
            (Self::D, Self::R) => Self::RD,

            // XZ | Y
            (Self::LF, Self::U) => Self::LFU,
            (Self::LF, Self::D) => Self::LFD,
            (Self::LB, Self::U) => Self::LBU,
            (Self::LB, Self::D) => Self::LBD,
            (Self::RF, Self::U) => Self::RFU,
            (Self::RF, Self::D) => Self::RFD,
            (Self::RB, Self::U) => Self::RBU,
            (Self::RB, Self::D) => Self::RBD,
            (Self::U, Self::LF) => Self::LFU,
            (Self::U, Self::LB) => Self::LBU,
            (Self::U, Self::RF) => Self::RFU,
            (Self::U, Self::RB) => Self::RBU,
            (Self::D, Self::LF) => Self::LFD,
            (Self::D, Self::LB) => Self::LBD,
            (Self::D, Self::RF) => Self::RFD,
            (Self::D, Self::RB) => Self::RBD,

            // YZ | X
            (Self::FU, Self::L) => Self::LFU,
            (Self::FU, Self::R) => Self::RFU,
            (Self::FD, Self::L) => Self::LFD,
            (Self::FD, Self::R) => Self::RFD,
            (Self::BU, Self::L) => Self::LBU,
            (Self::BU, Self::R) => Self::RBU,
            (Self::BD, Self::L) => Self::LBD,
            (Self::BD, Self::R) => Self::RBD,
            (Self::L, Self::FU) => Self::LFU,
            (Self::L, Self::FD) => Self::LFD,
            (Self::L, Self::BU) => Self::LBU,
            (Self::L, Self::BD) => Self::LBD,
            (Self::R, Self::FU) => Self::RFU,
            (Self::R, Self::FD) => Self::RFD,
            (Self::R, Self::BU) => Self::RBU,
            (Self::R, Self::BD) => Self::RBD,

            // XY | Z
            (Self::LU, Self::F) => Self::LFU,
            (Self::LU, Self::B) => Self::LBU,
            (Self::LD, Self::F) => Self::LFD,
            (Self::LD, Self::B) => Self::LBD,
            (Self::RU, Self::F) => Self::RFU,
            (Self::RU, Self::B) => Self::RBU,
            (Self::RD, Self::F) => Self::RFD,
            (Self::RD, Self::B) => Self::RBD,
            (Self::F, Self::LU) => Self::LFU,
            (Self::F, Self::LD) => Self::LFD,
            (Self::F, Self::RU) => Self::RFU,
            (Self::F, Self::RD) => Self::RFD,
            (Self::B, Self::LU) => Self::LBU,
            (Self::B, Self::LD) => Self::LBD,
            (Self::B, Self::RU) => Self::RBU,
            (Self::B, Self::RD) => Self::RBD,

            // Otherwise
            (Self::N, _) => rh,
            _ => self,
        }
    }
}
