use protocol::blocks::Block;
use protocol::chunk::Structure;

use std::fs;
use std::io::Read;

lazy_static! {
    pub static ref TREE: Structure = load_struct("tree.vox");
    pub static ref STONE: Structure = load_struct("stone.vox");
    pub static ref HOUSE: Structure = load_struct("house.vox");
}

fn load_struct(path: &str) -> Structure {
    let path = String::from("./models/") + path;

    log::info!("path: {}", path);

    let mut file = fs::File::open(path.clone()).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let voxel_data = dot_vox::load_bytes(&buffer).unwrap();

    let mut colors: Vec<[u8; 4]> = Vec::new();
    for color in voxel_data.palette.iter() {
        colors.push(color.to_le_bytes()); // little_endian apparently
    }

    if let Some(model) = voxel_data.models.get(0) {
        // rotation of z and y bc of my stupid standard :(
        let size = [
            model.size.x as usize,
            model.size.z as usize,
            model.size.y as usize,
        ];

        let mut structure = Structure::new(size);

        for voxel in model.voxels.iter() {
            // rotation of z and y bc of my stupid standard :(
            let pos = [voxel.x as usize, voxel.z as usize, voxel.y as usize];

            let color: [u8; 4] = colors[voxel.i as usize];
            let color = [color[0], color[1], color[2]];
            let block = Block::from_color(color);
            structure.place(pos, block);
        }

        structure
    } else {
        panic!("invalid structure file: {}", path);
    }
}
