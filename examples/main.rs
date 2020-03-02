use color::Rgb;
use generic_octree::{Octree, AABB};

fn main() {
    let octree = Octree::<u64, u64>::from_dotvox("./examples/monu10.vox", 21);

    println!(
        "{:?}",
        octree
            .unwrap()
            .into_iter()
            .map(|tree| tree.depth())
            .collect::<Vec<u32>>()
    );
    let mut tree: Octree<u64, Rgb<u8>> = Octree::new(21);
    let data: Vec<AABB> = AABB::new(0.0, 0.0, 0.0, 1.0, 0.5, 0.75).explode((0.5, 0.5, 0.5));
    println!(
        "{:?}",
        data.iter()
            .map(|aabb| aabb.fit_in(1, 21))
            .collect::<Vec<bool>>()
    );
    tree.merge(AABB::new(0.0, 0.0, 0.0, 1.0, 0.5, 0.75), Rgb::new(1, 1, 1));
    println!("{:#?}", tree);
    tree.merge(AABB::new(0.0, 0.0, 0.75, 1.0, 0.5, 1.0), Rgb::new(1, 1, 1));
    println!("{:#?}", tree);
    tree.merge(AABB::new(0.0, 0.5, 0.0, 1.0, 1.0, 1.0), Rgb::new(1, 1, 1));
    println!("{:#?}", tree);
    println!(
        "{:?}",
        AABB::new(1.0, 0.5, 0.75, 1.0, 0.5, 1.0).explode((0.5, 0.5, 0.5))
    );
    println!(
        "{:?}",
        AABB::new(0.0, 0.5, 0.0, 1.0, 1.0, 1.0).explode((0.5, 0.5, 0.5))
    );
}
