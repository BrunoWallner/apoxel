use std::io::Read;
use std::io::Write;
use std::fs;
use std::env::args;
use std::process::exit;

use protocol::chunk::Structure;
use protocol::chunk::Block;

fn main() {
    let file_path = match args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("could not find file!");
            exit(1);
        }
    };

    let mut file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("error opening file: {}", err);
            exit(1);
        }
    };

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
            let pos = [
                voxel.x as usize,
                voxel.z as usize,
                voxel.y as usize,
            ];

            let color: [u8; 4] = colors[voxel.i as usize];
            let color = [color[0], color[1], color[2]];
            let block = Block::from_color(color);
            structure.place(pos, block);
        }

        let encoded: Vec<u8> = bincode::serialize(&structure).unwrap();

        let output_file_path = match args().nth(2) {
            Some(o) => o,
            None => String::from("output.dat")
        };
        let mut output_file = fs::File::create(output_file_path).unwrap();
        output_file.write_all(&encoded).unwrap();
    }
}