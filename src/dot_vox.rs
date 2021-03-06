//
// Author: Alexandre Fourcat
// dot_vox.rs in generic_octree
// Description:
// Create a Generic Octree from a dot_vox file.
//

use crate::aabb::AABB;
use crate::{LocCode, Octree};
use dot_vox::{DotVoxData, Model, Voxel};
use rayon::prelude::*;

#[derive(PartialEq)]
pub enum ConversionType {
    Default,
    Optimal,
    Worst,
}

impl From<&Voxel> for AABB {
    fn from(voxel: &Voxel) -> AABB {
        AABB::new(
            voxel.x as f64,
            voxel.y as f64,
            voxel.z as f64,
            (voxel.x + 1) as f64,
            (voxel.y + 1) as f64,
            (voxel.z + 1) as f64,
        )
    }
}

fn voxel_to_aabb(
    voxel: Voxel,
    offset: (f64, f64, f64),
    normalization_vector: (f64, f64, f64),
    palette: &[u32],
) -> (AABB, u32) {
    (
        AABB::from(&voxel)
            .offset(offset)
            .normalize_with(normalization_vector),
        palette[voxel.i as usize],
    )
}

pub(crate) fn model_to_octree<L>(
    model: &Model,
    max_depth: u32,
    offset: (f64, f64, f64),
    normalization_vector: (f64, f64, f64),
    palette: &[u32],
) -> Octree<L, u32>
where
    L: LocCode,
{
    let mut octree = Octree::new(max_depth);
    model
        .voxels
        .iter()
        .map(|voxel: &Voxel| voxel_to_aabb(*voxel, offset, normalization_vector, palette))
        .collect::<Vec<(AABB, u32)>>()
        .into_iter()
        .for_each(|(aabb, data)| {
            octree.merge(aabb, data);
        });
    octree
}

pub(crate) fn vox_to_octrees<L>(
    data: DotVoxData,
    max_depth: u32,
    conversion_type: ConversionType,
) -> Vec<Octree<L, u32>>
where
    L: LocCode,
{
    data.models
        .iter()
        .map(|model| {
            let max_size = std::cmp::max(std::cmp::max(model.size.x, model.size.y), model.size.z);
            let frame_size = 2_f64.powf((max_size as f64).log2().ceil()) as u32;

            let offsets = if conversion_type != ConversionType::Default {
                let mut offsets = vec![];
                for range_x in 0_u32..(frame_size - model.size.x) {
                    for range_y in 0_u32..(frame_size - model.size.y) {
                        for range_z in 0_u32..(frame_size - model.size.z) {
                            offsets.push((range_x, range_y, range_z));
                        }
                    }
                }
                offsets
            } else {
                vec![(
                    (frame_size - model.size.x) / 2,
                    (frame_size - model.size.y) / 2,
                    (frame_size - model.size.z) / 2,
                )]
            };
            println!(
                "Model of size: ({}, {}, {}), Framing in: ({}, {}, {})",
                model.size.x, model.size.y, model.size.z, frame_size, frame_size, frame_size
            );
            let mut trees = offsets
                .par_iter()
                .map(|offset| {
                    println!("Computing offset: {:?}", offset);
                    let tree = model_to_octree(
                        model,
                        max_depth,
                        (offset.0 as f64, offset.1 as f64, offset.2 as f64),
                        (frame_size as f64, frame_size as f64, frame_size as f64),
                        &data.palette,
                    );
                    let size = tree.size();
                    (tree, size)
                })
                .collect::<Vec<(Octree<L, u32>, usize)>>();
            match conversion_type {
                ConversionType::Optimal => trees.sort_by(|a, b| a.1.cmp(&b.1).reverse()),
                ConversionType::Worst => trees.sort_by(|a, b| a.1.cmp(&b.1)),
                _ => (),
            };
            trees.pop().unwrap().0
        })
        .collect::<Vec<Octree<L, u32>>>()
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
        vox_to_octrees::<u32>(
            DotVoxData {
                version: 1,
                models: vec![],
                palette: vec![],
                materials: vec![],
            },
            5,
            ConversionType::Default,
        );
    }

    #[test]
    fn basic() {
        let vox = dot_vox::load("./examples/monu10.vox").unwrap();
        let _octrees: Vec<Octree<u32, u32>> = vox_to_octrees(vox, 21, ConversionType::Default);
    }
}
