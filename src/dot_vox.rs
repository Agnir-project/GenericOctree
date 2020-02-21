//
// Author: Alexandre Fourcat
// dot_vox.rs in generic_octree
// Description:
// Create a Generic Octree from a dot_vox file.

use dot_vox::DotVoxData;
use crate::Octree;

impl<T> From<DotVoxData> for Octree<T, (u8, u8, u8)> {
    fn from(data: DotVoxData) -> Octree<T, (u8, u8, u8)> {
        Octree::new()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn alive() {
        assert!(true);
    }
}
