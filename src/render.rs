use crate::node::OctreeNode;
use crate::{aabb::get_level_from_loc_code, Octree};
use genmesh::Position;
use petgraph::{
    graph::EdgeReference,
    graphmap::{DiGraphMap, GraphMap},
    stable_graph::NodeIndex,
};
use rendy::mesh::PosColorNorm;

use std::{
    fmt::Debug,
    hash::{Hasher, Hash},
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr}, collections::HashSet,
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

fn normalize(vector: (i32, i32, i32)) -> (f32, f32, f32) {
    let magnitude = (((vector.0).pow(2) + (vector.1).pow(2) + (vector.2).pow(2)) as f64).sqrt();
    (
        (vector.0 as f64 / magnitude) as f32,
        (vector.1 as f64 / magnitude) as f32,
        (vector.2 as f64 / magnitude) as f32,
    )
}

fn invert(vector: (i32, i32, i32)) -> (i32, i32, i32) {
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
    let offset = (2 as u32).pow(32 - get_level_from_loc_code(*loc_code));
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
    graph.add_edge(lbd, rbd, (1, 0, 0));
    graph.add_edge(lbd, rfd, (1, 0, 1));

    // LFD
    graph.add_edge(lfd, lbd, (0, 0, -1));
    graph.add_edge(lfd, lbu, (0, 1, -1));
    graph.add_edge(lfd, lfu, (0, 1, 0));
    graph.add_edge(lfd, rfd, (1, 0, 0));
    graph.add_edge(lfd, rfu, (1, 1, 0));

    // LBU
    graph.add_edge(lbu, lbd, (0, -1, 0));
    graph.add_edge(lbu, lfd, (0, -1, 1));
    graph.add_edge(lbu, lfu, (0, 0, 1));
    graph.add_edge(lbu, rbd, (1, -1, 0));
    graph.add_edge(lbu, rbu, (1, 0, 0));

    // LFU
    graph.add_edge(lfu, lfd, (0, -1, 0));
    graph.add_edge(lfu, lbu, (0, 0, -1));
    graph.add_edge(lfu, rbu, (1, 0, -1));
    graph.add_edge(lfu, rfu, (1, 0, 0));

    // RBD
    graph.add_edge(rbd, lbd, (-1, 0, 0));
    graph.add_edge(rbd, lbu, (-1, 1, 0));
    graph.add_edge(rbd, rfd, (0, 0, 1));
    graph.add_edge(rbd, rbu, (0, 1, 0));
    graph.add_edge(rbd, rfu, (0, 1, 1));

    // RFD
    graph.add_edge(rfd, lbd, (-1, 0, -1));
    graph.add_edge(rfd, lfd, (-1, 0, 0));
    graph.add_edge(rfd, rbd, (0, 0, -1));
    graph.add_edge(rfd, rfu, (0, 1, 0));

    // RBU
    graph.add_edge(rbu, lbu, (-1, 0, 0));
    graph.add_edge(rbu, lfu, (-1, 0, 1));
    graph.add_edge(rbu, rbd, (0, -1, 0));
    graph.add_edge(rbu, rfu, (0, 0, 1));

    // RFU
    graph.add_edge(rfu, lfd, (-1, -1, 0));
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
    let offset = ((2 as u32).pow(32 - get_level_from_loc_code(loc_code)));
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
            })
            .into_graph();
        let mut verticesGraph = verticesGraph.map(
            |idx: NodeIndex<u32>, node| {
                let anti_normal = verticesGraph.edges(idx).fold((0, 0, 0), |acc, edge| {
                    (
                        acc.0 + edge.weight().0,
                        acc.1 + edge.weight().1,
                        acc.2 + edge.weight().2,
                    )
                });
                let normal = normalize(invert(anti_normal));
                let color = [
                    (node.color[0] as f32 / 256f32) as f32,
                    (node.color[1] as f32 / 256f32) as f32,
                    (node.color[2] as f32 / 256f32) as f32,
                    (node.color[3] as f32 / 256f32) as f32,
                ];
                let position = Position {
                    x: ((node.position.x as f64 / (2u64.pow(32) as f64)) as f32),
                    y: ((node.position.y as f64 / (2u64.pow(32) as f64)) as f32),
                    z: ((node.position.z as f64 / (2u64.pow(32) as f64)) as f32),
                };
                PosColorNorm {
                    normal: [normal.0, normal.1, normal.2].into(),
                    color: color.into(),
                    position: position.into(),
                }
            },
            |_, edge| (),
        );
        let mut indices = vec![];
        let node_indices = verticesGraph
            .node_indices()
            .collect::<Vec<NodeIndex<u32>>>();

        for index in node_indices {
            let edges: Vec<NodeIndex> = verticesGraph.neighbors(index).collect();
            let lines: Vec<Line> = edges
                .into_iter()
                .map(|idx| {
                    verticesGraph
                        .neighbors(idx)
                        .filter(|new_idx| *new_idx != index)
                        .map(|new_idx| Line(idx, new_idx))
                        .collect::<Vec<Line>>()
                })
                .flatten()
                .collect();
            let triangles: HashSet<Triangle> = lines
                .into_iter()
                .filter(|line| {
                    verticesGraph
                        .neighbors(line.1)
                        .find(|new_idx| *new_idx == index)
                        .is_some()
                })
                .map(|line| Triangle(line.0, line.1, index))
                .collect();
            triangles.into_iter().for_each(|triangle| {
                indices.push((triangle.0).index() as u32);
                indices.push((triangle.1).index() as u32);
                indices.push((triangle.2).index() as u32);
                match verticesGraph.find_edge(triangle.0, triangle.1) {
                    Some(edge) => verticesGraph.remove_edge(edge),
                    None => None,
                };
            });
        }

        let vertices = verticesGraph
            .clone()
            .into_nodes_edges()
            .0
            .into_iter()
            .map(|node| node.weight)
            .collect::<Vec<PosColorNorm>>();
        
        Model { vertices, indices }
    }
}

#[derive(Debug)]
struct Line(NodeIndex, NodeIndex);

#[derive(Debug, Eq)]
struct Triangle(NodeIndex, NodeIndex, NodeIndex);

impl Hash for Triangle {

    fn hash<S: Hasher>(&self, state: &mut S) {
        let max = std::cmp::max(std::cmp::max(self.0, self.1), self.2);
        let min = std::cmp::min(std::cmp::min(self.0, self.1), self.2);
        let medium = if self.0 != max && self.0 != min {
            self.0
        } else if self.1 != max && self.1 != min {
            self.1
        } else {
            self.2
        };
        max.hash(state);
        medium.hash(state);
        min.hash(state);
    }

}

impl PartialEq for Triangle {

    fn eq(&self, other: &Self) -> bool {
        let max = std::cmp::max(std::cmp::max(self.0, self.1), self.2);
        let min = std::cmp::min(std::cmp::min(self.0, self.1), self.2);
        let medium = if self.0 != max && self.0 != min {
            self.0
        } else if self.1 != max && self.1 != min {
            self.1
        } else {
            self.2
        };

        let other_max = std::cmp::max(std::cmp::max(other.0, other.1), other.2);
        let other_min = std::cmp::min(std::cmp::min(other.0, other.1), other.2);
        let other_medium = if other.0 != max && other.0 != min {
            other.0
        } else if other.1 != max && other.1 != min {
            other.1
        } else {
            other.2
        };
        
        max == other_max && min == other_min && medium == other_medium
    }

}
