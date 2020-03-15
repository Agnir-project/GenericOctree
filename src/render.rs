use crate::node::OctreeNode;
use crate::{aabb::get_level_from_loc_code, Octree};
use genmesh::Position;

use std::{
    fmt::Debug,
    hash::Hash,
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr},
};

pub struct Model {
    pub vertices: Vec<PosColorNorm>,
    pub indices: Vec<u32>,
}
use rendy::mesh::PosColorNorm;

lazy_static::lazy_static! {
    static ref LBD: Position = Position {
        x: -1.0 / 3.0_f32.sqrt(),
        y: -1.0 / 3.0_f32.sqrt(),
        z: -1.0 / 3.0_f32.sqrt()
    };

    static ref LBU: Position = Position {
        x: -1.0 / 3.0_f32.sqrt(),
        y: 1.0 / 3.0_f32.sqrt(),
        z: -1.0 / 3.0_f32.sqrt()
    };

    static ref RBD: Position = Position {
        x: 1.0 / 3.0_f32.sqrt(),
        y: -1.0 / 3.0_f32.sqrt(),
        z: -1.0 / 3.0_f32.sqrt()
    };

    static ref RBU: Position = Position {
        x: 1.0 / 3.0_f32.sqrt(),
        y: 1.0 / 3.0_f32.sqrt(),
        z: -1.0 / 3.0_f32.sqrt()
    };

    static ref LFD: Position = Position {
        x: -1.0 / 3.0_f32.sqrt(),
        y: -1.0 / 3.0_f32.sqrt(),
        z: 1.0 / 3.0_f32.sqrt()
    };

    static ref LFU: Position = Position {
        x: -1.0 / 3.0_f32.sqrt(),
        y: 1.0 / 3.0_f32.sqrt(),
        z: 1.0 / 3.0_f32.sqrt()
    };

    static ref RFD: Position = Position {
        x: 1.0 / 3.0_f32.sqrt(),
        y: -1.0 / 3.0_f32.sqrt(),
        z: 1.0 / 3.0_f32.sqrt()
    };

    static ref RFU: Position = Position {
        x: 1.0 / 3.0_f32.sqrt(),
        y: 1.0 / 3.0_f32.sqrt(),
        z: 1.0 / 3.0_f32.sqrt()
    };
}

fn get_vertices<L>(loc_code: &L, data: &OctreeNode<L, u32>) -> Vec<PosColorNorm>
where
    L: Eq
        + Debug
        + Hash
        + From<u8>
        + From<u64>
        + Copy
        + Shr<Output = L>
        + Shl<Output = L>
        + BitOr<Output = L>
        + BitAnd<Output = L>
        + BitXor<Output = L>
        + Not<Output = L>,
{
    let center = get_center(*loc_code);
    let offset: f64 = 1.0 / ((2 as u32).pow(get_level_from_loc_code(*loc_code)) as f64);
    let color = data.data.to_be_bytes();
    let color = [
        color[0] as f32 / 256_f32,
        color[1] as f32 / 256_f32,
        color[2] as f32 / 256_f32,
        color[3] as f32 / 256_f32,
    ];
    vec![
        // LBD, LFD, LBU
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBU).into(),
            color: color.into(),
        },
        // LFD, LFU, LBU
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBU).into(),
            color: color.into(),
        },
        // LBD, LFD, RFD
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFD).into(),
            color: color.into(),
        },
        //LBD, RBD, RFD
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFD).into(),
            color: color.into(),
        },
        //RBD, RBU, LBD
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBD).into(),
            color: color.into(),
        },
        //RBU, LBU, LBD
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBD).into(),
            color: color.into(),
        },
        //RBU, RFU, LBU
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBU).into(),
            color: color.into(),
        },
        //RFU, LFU, LBU
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*LBU).into(),
            color: color.into(),
        },
        //RFU, LFD, LFU
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFU).into(),
            color: color.into(),
        },
        //RFU, LFD, RFD
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 - offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*LFD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFD).into(),
            color: color.into(),
        },
        //RFU, RBU, RFD
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBU).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFD).into(),
            color: color.into(),
        },
        //RFD, RBD, RBU
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 + offset) as f32,
            }
            .into(),
            normal: (*RFD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 - offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBD).into(),
            color: color.into(),
        },
        PosColorNorm {
            position: Position {
                x: (center.0 + offset) as f32,
                y: (center.1 + offset) as f32,
                z: (center.2 - offset) as f32,
            }
            .into(),
            normal: (*RBU).into(),
            color: color.into(),
        },
    ]
}

fn get_center<L>(loc_code: L) -> (f64, f64, f64)
where
    L: Eq
        + Debug
        + Hash
        + From<u8>
        + From<u64>
        + Copy
        + Shr<Output = L>
        + Shl<Output = L>
        + BitOr<Output = L>
        + BitAnd<Output = L>
        + BitXor<Output = L>
        + Not<Output = L>,
{
    if loc_code == L::from(1u8) {
        return (0.5, 0.5, 0.5);
    }
    let offset: f64 = 1.0 / ((2 as u32).pow(get_level_from_loc_code(loc_code)) as f64);
    let mask = L::from(u64::max_value() ^ 7u64);
    let center = get_center(loc_code >> L::from(3u8));
    let value = (loc_code ^ mask) & !mask;
    if value == L::from(0u8) {
        (center.0 - offset, center.1 + offset, center.2 - offset)
    } else if value == L::from(1u8) {
        (center.0 - offset, center.1 + offset, center.2 + offset)
    } else if value == L::from(2u8) {
        (center.0 - offset, center.1 - offset, center.2 + offset)
    } else if value == L::from(3u8) {
        (center.0 - offset, center.1 - offset, center.2 - offset)
    } else if value == L::from(4u8) {
        (center.0 + offset, center.1 - offset, center.2 - offset)
    } else if value == L::from(5u8) {
        (center.0 + offset, center.1 - offset, center.2 + offset)
    } else if value == L::from(6u8) {
        (center.0 + offset, center.1 + offset, center.2 + offset)
    } else {
        (center.0 + offset, center.1 + offset, center.2 - offset)
    }
}

impl<L> From<&Octree<L, u32>> for Model
where
    L: Eq
        + Debug
        + Hash
        + Ord
        + From<u8>
        + From<u64>
        + Copy
        + Shr<Output = L>
        + Shl<Output = L>
        + BitOr<Output = L>
        + BitAnd<Output = L>
        + BitXor<Output = L>
        + Not<Output = L>
        + PartialEq,
{
    fn from(tree: &Octree<L, u32>) -> Self {
        let vertices = tree
            .content
            .iter()
            .map(|item: (&L, &OctreeNode<L, u32>)| get_vertices(item.0, item.1))
            .flatten()
            .collect::<Vec<PosColorNorm>>();
        let len = vertices.len();
        Model {
            vertices,
            indices: (0u32..len as u32).collect::<Vec<u32>>(),
        }
    }
}
