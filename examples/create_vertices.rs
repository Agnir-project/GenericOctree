use generic_octree::Octree;
use std::env;
use std::process;
use genmesh::{
    generators::{IndexedPolygon, SharedVertex},
    Vertices,
};
use rendy::mesh::{PosColorNorm};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let tree = Octree::<u64, u32>::load_from_file(filename).unwrap();

    let model: generic_octree::render::Model = generic_octree::render::Model::from(&tree);

    /*let model_indices: Vec<u32> = genmesh::Vertices::vertices(model.indexed_polygon_iter())
        .map(|i| i as u32)
        .collect();

    let model_vertices: Vec<PosColorNorm> = model.shared_vertex_iter()
                .map(|v| PosColorNorm {
                    position: v.pos.into(),
                    color: [
                        (v.pos.x + 1.0) / 2.0,
                        (v.pos.y + 1.0) / 2.0,
                        (v.pos.z + 1.0) / 2.0,
                        1.0,
                    ]
                    .into(),
                    normal: v.normal.into(),
                })
                .collect();*/
}
