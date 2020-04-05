use generic_octree::Octree;
use std::env;
use std::process;

use generic_octree::dot_vox::ConversionType;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let out = &args[2];

    if let Ok(s) = Octree::<u64, u32>::from_dotvox(filename, 21, ConversionType::Default) {
        s[0].save_to_file(out).unwrap();
    } else {
        eprintln!("Parsing error");
        process::exit(1);
    }

    let _tree = Octree::<u64, u32>::load_from_file(out).unwrap();
}
