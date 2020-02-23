//
// Author: Alexandre Fourcat
// dot_vox.rs in generic_octree
// Description:
// Create a Generic Octree from a dot_vox file.
//

use crate::aabb::AABB;
use crate::octree::LocCode;
use crate::Octree;
use dot_vox::DotVoxData;
use std::hash::Hash;

pub(crate) fn vox_to_octrees<L: LocCode>(data: DotVoxData, max_depth: u32) -> Vec<Octree<L, u32>> {
    println!("Version of the file: {}.", data.version);
    println!("Palette: {:?}.", data.palette);
    let mut octrees = Vec::with_capacity(1);

    for (i, model) in data.models.iter().enumerate() {
        let mut octree = Octree::new(max_depth);
        println!(
            "Model {} of size: x: {} y: {} z: {}.",
            i, model.size.x, model.size.y, model.size.z
        );
        for voxel in model.voxels.iter() {
            println!(
                "x: {} y: {} z: {} color: {}",
                voxel.x,
                voxel.y,
                voxel.z,
                data.palette[voxel.i as usize] // TODO: maybe do a safe version.
            );
            let (x, y, z) = (
                voxel.x as f64 / model.size.x as f64,
                voxel.y as f64 / model.size.y as f64,
                voxel.z as f64 / model.size.z as f64,
            );
            let (x1, y1, z1) = (
                x + 1.0 / model.size.x as f64,
                y + 1.0 / model.size.y as f64,
                z + 1.0 / model.size.z as f64,
            );
            println!("Data: {:#?}", AABB::<L>::new(x, y, z, x1, y1, z1));
            octree.merge(
                AABB::<L>::new(x, y, z, x1, y1, z1),
                data.palette[voxel.i as usize],
            );
        }
        octrees.push(octree);
    }
    octrees
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn alive() {
        assert!(true);
    }

    #[test]
    fn empty() {
        vox_to_octrees::<u32>(DotVoxData {
            version: 1,
            models: vec![],
            palette: vec![],
            materials: vec![],
        }, 5);
    }

    #[test]
    fn basic() {
        let vox = dot_vox::load("./examples/chr_cat.vox").unwrap();
        let octrees: Vec<Octree<u32, u32>> = vox_to_octrees(vox, 5);
    }
}
