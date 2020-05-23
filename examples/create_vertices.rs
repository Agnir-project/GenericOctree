use generic_octree::Octree;

use std::{env, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let i = Instant::now();
    let tree = Octree::<u64, u32>::load_from_file(filename).unwrap();

    let tree = tree.transform_fn(color::Rgba::from_hex);

    let _model: generic_octree::render::Model = tree.into();
    println!("µs: {:?}", i.elapsed().as_micros());

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
