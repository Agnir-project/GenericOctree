use crate::node::OctreeNode;
use crate::{LocCode, Octree};
use petgraph::graphmap::DiGraphMap;
use rayon::prelude::*;

use std::{fmt::Debug, hash::Hash};


pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Ord, Hash, Debug)]
struct VertexPosition {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ColoredTriangle {
    pub color: [u8; 4],
    pub vertices: [VertexPosition; 3],
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

fn get_spins(center: (u32, u32, u32), offset: u32) -> [u32; 6] {
    [
        center.0 - offset,
        center.0 + offset,
        center.1 - offset,
        center.1 + offset,
        center.2 - offset,
        center.2 + offset,
    ]
}

#[allow(clippy::many_single_char_names)]
fn get_angles(center: (u32, u32, u32), offset: u32) -> [VertexPosition; 8] {
    let [l, r, d, u, b, f] = get_spins(center, offset);
    let lbd = VertexPosition { x: l, y: d, z: b };
    let lfd = VertexPosition { x: l, y: d, z: f };
    let lbu = VertexPosition { x: l, y: u, z: b };
    let lfu = VertexPosition { x: l, y: u, z: f };
    let rbd = VertexPosition { x: r, y: d, z: b };
    let rfd = VertexPosition { x: r, y: d, z: f };
    let rbu = VertexPosition { x: r, y: u, z: b };
    let rfu = VertexPosition { x: r, y: u, z: f };
    [lbd, lfd, lbu, lfu, rbd, rfd, rbu, rfu]
}

type MeshGraph = DiGraphMap<VertexPosition, (i32, i32, i32)>;
type ColoredTriangles = Vec<ColoredTriangle>;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct RawVertex {
    pub position: [u32; 3],
    pub color: [u8; 4],
    pub normal: [i32; 3],
}

fn get_vertices<L>(
    loc_code: L,
    node: OctreeNode<color::Rgba<u8>>,
) -> OctreeNode<(MeshGraph, ColoredTriangles)>
where
    L: LocCode,
{
    let data = node.data;
    let mut graph = DiGraphMap::new();
    let center = loc_code.get_center_u32();
    let offset = (2 as u32).pow(32 - loc_code.get_level());
    let color = [data.rgb().r, data.rgb().g, data.rgb().b, data.a];

    let [lbd, lfd, lbu, lfu, rbd, rfd, rbu, rfu] = get_angles(center, offset);

    let triangles: ColoredTriangles = vec![
        ColoredTriangle {
            color,
            vertices: [lbd, rfd, rbd],
        },
        ColoredTriangle {
            color,
            vertices: [lbd, rfd, lfd],
        },
        ColoredTriangle {
            color,
            vertices: [lfd, rfu, rfd],
        },
        ColoredTriangle {
            color,
            vertices: [lfd, rfu, lfu],
        },
        ColoredTriangle {
            color,
            vertices: [lfu, rbu, rfu],
        },
        ColoredTriangle {
            color,
            vertices: [lfu, rbu, lbu],
        },
        ColoredTriangle {
            color,
            vertices: [lbd, lfu, lfd],
        },
        ColoredTriangle {
            color,
            vertices: [lbd, lfu, lbu],
        },
        ColoredTriangle {
            color,
            vertices: [rbd, lbu, rbu],
        },
        ColoredTriangle {
            color,
            vertices: [rbd, lbu, lbd],
        },
        ColoredTriangle {
            color,
            vertices: [rfd, rbu, rbd],
        },
        ColoredTriangle {
            color,
            vertices: [rfd, rbu, rfu],
        },
    ];

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
    OctreeNode::new((graph, triangles))
}

impl<'a, L> From<Octree<L, color::Rgba<u8>>> for Model
where
    L: LocCode,
{
    fn from(tree: Octree<L, color::Rgba<u8>>) -> Self {
        //let centers = gen_centers(&tree);
        //println!("{:?}", centers);

        let tree: Octree<L, (MeshGraph, ColoredTriangles)> = tree.transform_nodes_fn(get_vertices);
        let (vertices_graph, triangles) =
            tree.content
                .into_iter()
                .fold((DiGraphMap::new(), vec![]), |mut acc, val| {
                    acc.1.extend(val.1.data.1.into_iter());
                    val.1.data.0.all_edges().into_iter().for_each(|edge| {
                        acc.0.add_edge(edge.0, edge.1, *edge.2);
                    });
                    acc
                });
        let vertices = triangles
            .into_par_iter()
            .map(|triangle| {
                triangle
                    .vertices
                    .iter()
                    .map(|position| {
                        let anti_normal =
                            vertices_graph
                                .edges(*position)
                                .fold((0, 0, 0), |acc, edge| {
                                    (acc.0 + (edge.2).0, acc.1 + (edge.2).1, acc.2 + (edge.2).2)
                                });
                        let normal = invert(anti_normal);
                        RawVertex {
                            position: [position.x, position.y, position.z],
                            normal: [normal.0, normal.1, normal.2],
                            color: triangle.color,
                        }
                    })
                    .collect::<Vec<RawVertex>>()
            })
            .flatten()
            .collect::<Vec<RawVertex>>();

        let indices = 0u32..vertices.len() as u32;

        let vertices = vertices
            .into_par_iter()
            .map(|vertex: RawVertex| {
                let normal = normalize((vertex.normal[0], vertex.normal[1], vertex.normal[2]));
                Vertex {
                    position: [
                        ((vertex.position[0] as f64 / (2u64.pow(32) as f64)) as f32),
                        ((vertex.position[1] as f64 / (2u64.pow(32) as f64)) as f32),
                        ((vertex.position[2] as f64 / (2u64.pow(32) as f64)) as f32),
                    ],
                    normal: [normal.0, normal.1, normal.2].into(),
                    color: [
                        (vertex.color[0] as f32 / 256f32) as f32,
                        (vertex.color[1] as f32 / 256f32) as f32,
                        (vertex.color[2] as f32 / 256f32) as f32,
                        (vertex.color[3] as f32 / 256f32) as f32,
                    ],
                }
            })
            .collect::<Vec<Vertex>>();

        Model {
            vertices,
            indices: indices.collect::<Vec<u32>>(),
        }
    }
}
