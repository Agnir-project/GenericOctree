use crate::node::OctreeNode;
use crate::{aabb::get_level_from_loc_code, Octree};
use genmesh::Position;
use petgraph::graphmap::{DiGraphMap, GraphMap};
use rendy::mesh::PosColorNorm;

use std::{
    fmt::Debug,
    hash::Hash,
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr},
};

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Ord, Hash, Debug)]
struct VertexPosition {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct Model {
    pub vertices: Vec<PosColorNorm>,
    pub indices: Vec<u32>,
}

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Ord, Hash, Debug)]
struct Vertex {
    pub position: VertexPosition,
    pub color: [u8; 4],
}

fn normalize(vector: (u32, u32, u32)) -> (f32, f32, f32) {
    let magnitude = (((vector.0).pow(2) + (vector.1).pow(2) + (vector.2).pow(2)) as f64).sqrt();
    (
        vector.0 / magnitude,
        vector.1 / magnitude,
        vector.2 / magnitude,
    )
}

fn invert(vector: (u32, u32, u32)) -> (u32, u32, u32) {
    (-vector.0, -vector.1, -vector.2)
}

fn get_vertices<L>(loc_code: &L, data: color::Rgba<u8>) -> DiGraphMap<Vertex, (i32, i32, i32)>
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
    let mut graph = DiGraphMap::new();
    let center = get_center(*loc_code);
    let offset = (2 as u32).pow(get_level_from_loc_code(*loc_code));
    let color = [data.a, data.rgb().b, data.rgb().g, 255];
    let lbd = Vertex {
        position: VertexPosition {
            x: (center.0 - offset),
            y: (center.1 - offset),
            z: (center.2 - offset),
        },
        color,
    };

    let lfd = Vertex {
        position: VertexPosition {
            x: (center.0 - offset),
            y: (center.1 - offset),
            z: (center.2 + offset),
        },
        color,
    };

    let lbu = Vertex {
        position: VertexPosition {
            x: (center.0 - offset),
            y: (center.1 + offset),
            z: (center.2 - offset),
        },
        color,
    };

    let lfu = Vertex {
        position: VertexPosition {
            x: (center.0 - offset),
            y: (center.1 + offset),
            z: (center.2 + offset),
        },
        color,
    };

    let rbd = Vertex {
        position: VertexPosition {
            x: (center.0 + offset),
            y: (center.1 - offset),
            z: (center.2 - offset),
        },
        color,
    };

    let rfd = Vertex {
        position: VertexPosition {
            x: (center.0 + offset),
            y: (center.1 - offset),
            z: (center.2 + offset),
        },
        color,
    };

    let rbu = Vertex {
        position: VertexPosition {
            x: (center.0 + offset),
            y: (center.1 + offset),
            z: (center.2 - offset),
        },
        color,
    };

    let rfu = Vertex {
        position: VertexPosition {
            x: (center.0 + offset),
            y: (center.1 + offset),
            z: (center.2 + offset),
        },
        color,
    };
    // LBD
    graph.add_edge(lbd, lfd, (0, 0, 1));
    graph.add_edge(lbd, lbu, (0, 1, 0));
    graph.add_edge(lbd, lfu, (0, 1, 1));
    graph.add_edge(lbd, rbd, (1, 0, 0));
    graph.add_edge(lbd, rfd, (1, 0, 1));
    graph.add_edge(lbd, rbu, (1, 1, 0));
    graph.add_edge(lbd, rfu, (1, 1, 1));

    // LFD
    graph.add_edge(lfd, lbd, (0, 0, -1));
    graph.add_edge(lfd, lbu, (0, 1, -1));
    graph.add_edge(lfd, lfu, (0, 1, 0));
    graph.add_edge(lfd, rbd, (1, 0, -1));
    graph.add_edge(lfd, rfd, (1, 0, 0));
    graph.add_edge(lfd, rbu, (1, 1, -1));
    graph.add_edge(lfd, rfu, (1, 1, 0));

    // LBU
    graph.add_edge(lbu, lbd, (0, -1, 0));
    graph.add_edge(lbu, lfd, (0, -1, 1));
    graph.add_edge(lbu, lfu, (0, 0, 1));
    graph.add_edge(lbu, rbd, (1, -1, 0));
    graph.add_edge(lbu, rfd, (1, -1, 1));
    graph.add_edge(lbu, rbu, (1, 0, 0));
    graph.add_edge(lbu, rfu, (1, 0, 1));

    // LFU
    graph.add_edge(lfu, lbd, (0, -1, -1));
    graph.add_edge(lfu, lfd, (0, -1, 0));
    graph.add_edge(lfu, lbu, (0, 0, -1));
    graph.add_edge(lfu, rbd, (1, -1, -1));
    graph.add_edge(lfu, rfd, (1, -1, 0));
    graph.add_edge(lfu, rbu, (1, 0, -1));
    graph.add_edge(lfu, rfu, (1, 0, 0));

    // RBD
    graph.add_edge(rbd, lbd, (-1, 0, 0));
    graph.add_edge(rbd, lfd, (-1, 0, 1));
    graph.add_edge(rbd, lbu, (-1, 1, 0));
    graph.add_edge(rbd, lfu, (-1, 1, 1));
    graph.add_edge(rbd, rfd, (0, 0, 1));
    graph.add_edge(rbd, rbu, (0, 1, 0));
    graph.add_edge(rbd, rfu, (0, 1, 1));

    // RFD
    graph.add_edge(rfd, lbd, (-1, 0, -1));
    graph.add_edge(rfd, lfd, (-1, 0, 0));
    graph.add_edge(rfd, lbu, (-1, 1, -1));
    graph.add_edge(rfd, lfu, (-1, 1, 0));
    graph.add_edge(rfd, rbd, (0, 0, -1));
    graph.add_edge(rfd, rbu, (0, 1, -1));
    graph.add_edge(rfd, rfu, (0, 1, 0));

    // RBU
    graph.add_edge(rbu, lbd, (-1, -1, 0));
    graph.add_edge(rbu, lfd, (-1, -1, 1));
    graph.add_edge(rbu, lbu, (-1, 0, 0));
    graph.add_edge(rbu, lfu, (-1, 0, 1));
    graph.add_edge(rbu, rbd, (0, -1, 0));
    graph.add_edge(rbu, rfd, (0, -1, 1));
    graph.add_edge(rbu, rfu, (0, 0, 1));

    // RFU
    graph.add_edge(rfu, lbd, (-1, -1, -1));
    graph.add_edge(rfu, lfd, (-1, -1, 0));
    graph.add_edge(rfu, lbu, (-1, 0, -1));
    graph.add_edge(rfu, lfu, (-1, 0, 0));
    graph.add_edge(rfu, rbd, (0, -1, -1));
    graph.add_edge(rfu, rfd, (0, -1, 0));
    graph.add_edge(rfu, rbu, (0, 0, -1));
    graph
}

fn get_center<L>(loc_code: L) -> (u32, u32, u32)
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
        return ((2 as u32).pow(31), (2 as u32).pow(31), (2 as u32).pow(31));
    }
    let offset = ((2 as u32).pow(get_level_from_loc_code(loc_code)));
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
        println!("{}", tree.size());
        let verticesGraph = tree
            .content
            .iter()
            .map(|item: (&L, &OctreeNode<L, u32>)| {
                get_vertices(item.0, color::Rgba::from_hex(item.1.data))
            })
            .fold(DiGraphMap::new(), |mut acc, val| {
                val.all_edges().into_iter().for_each(|edge| {
                    acc.add_edge(edge.0, edge.1, *edge.2);
                });
                acc
            });
        let vertices = verticesGraph
            .nodes()
            .map(|node| {
                let anti_normal = verticesGraph.edges(node).fold((0, 0, 0), |acc, edge| {
                    (acc.0 + (edge.2).0, acc.1 + (edge.2).1, acc.2 + (edge.2).2)
                });
                let normal = normalize(invert(anti_normal));
                PosColorNorm {
                    normal,
                    color: node.color,
                    position: Position {
                        x: node.position.x / 2.pow(32),
                        y: node.position.y / 2.pow(32),
                        z: node.position.z / 2.pow(32)
                    }
                }
            })
            .collect::<Vec<PosColorNorm>>();
        let len = verticesGraph.node_count();
        println!("{}", len);
        Model {
            vertices,
            indices: (0u32..len as u32).collect::<Vec<u32>>(),
        }
    }
}
