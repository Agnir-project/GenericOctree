//
// Author: Alexandre Fourcat
// dot_vox.rs in generic_octree
// Description:
// Create a Generic Octree from a dot_vox file.
//

use crate::octree::LocCode;
use crate::Octree;
use dot_vox::DotVoxData;
use std::hash::Hash;

impl<T: LocCode> From<DotVoxData> for Octree<T, (u8, u8, u8)> {
    fn from(data: DotVoxData) -> Octree<T, (u8, u8, u8)> {
        println!("Version of the file: {}.", data.version);

        for (i, model) in data.models.iter().enumerate() {
            println!(
                "Model {} of size: x: {} y: {} z: {}.",
                i, model.size.x, model.size.y, model.size.z
            );
            for voxel in model.voxels.iter() {
                println!(
                    "x: {} y: {} z: {} color: {}",
                    voxel.x, voxel.y, voxel.z, data.palette[voxel.i as usize]
                );
            }
        }

        Octree::new()
    }
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
        Octree::<u32, (u8, u8, u8)>::from(DotVoxData {
            version: 1,
            models: vec![],
            palette: vec![],
            materials: vec![],
        });
    }

    #[test]
    fn basic() {
        Octree::<u32, (u8, u8, u8)>::from(dot_vox::load("./example_file.vox").unwrap());
    }
}
