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

    if let Some(model) = voxel_data.models.get(0) {
        let size = [
            model.size.x as usize,
            model.size.y as usize,
            model.size.z as usize,
        ];
            
        let mut structure = Structure::new(size);

        for voxel in model.voxels.iter() {
            let pos = [
                voxel.x as usize,
                voxel.y as usize,
                voxel.z as usize,
            ];

            let block = Block::index_to_block(voxel.i);
            
            structure.place(pos, block);
        }

        let encoded: Vec<u8> = bincode::serialize(&structure).unwrap();

        let mut output_file_path = match args().nth(2) {
            Some(o) => o,
            None => String::from("output.dat")
        };
        let mut output_file = fs::File::create(output_file_path).unwrap();
        output_file.write_all(&encoded).unwrap();
    }
}

trait IndexToBlock {
    fn index_to_block(index: u8) -> Block;
}

impl IndexToBlock for Block {
    fn index_to_block(index: u8) -> Block {
        match index {
            // Block::Water as last Block in Block enum
            i if i >= Block::None as u8 && i <= Block::Water as u8 =>
                unsafe {std::mem::transmute(i)}
            _ => Block::None,
        }
    }
}