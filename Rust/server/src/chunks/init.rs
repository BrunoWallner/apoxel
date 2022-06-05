use crate::channel::*;
use super::Instruction;
use Instruction::*;
use std::thread;
use std::collections::BTreeMap;
use protocol::{Coord, chunk::Chunk};
use super::generation::generate;
use crate::CONFIG;

pub(super) fn init(
    rx: Receiver<Instruction>,
) {
     
    thread::spawn(move || {
        let mut chunks: BTreeMap<Coord, Chunk> = BTreeMap::default();
        let mut leftover: BTreeMap<Coord, Chunk> = BTreeMap::default();

        while let Some(instruction) = rx.recv() {
            match instruction {
                RequestChunk { coord, sender } => {
                    if let Some(chunk) = chunks.get(&coord) {
                        let _ = sender.send(chunk.clone());
                    }
                    // generate
                    let mut super_chunk = generate(Chunk::new(coord), CONFIG.chunks.seed);
                    // extract main chunk
                    let mc_coord = super_chunk.main_chunk;
                    let mut main_chunk  = super_chunk.chunks.remove(&mc_coord).unwrap();
                    // merge main chunk with leftover if found
                    if let Some(left) = leftover.get(&mc_coord) {
                        main_chunk.merge(left);
                        leftover.remove(&mc_coord);
                    }
                    // push chunk to chunks and send it
                    chunks.insert(mc_coord, main_chunk.clone());
                    let _ = sender.send(main_chunk);

                    // handle leftovers of generated chunk


                }
                UnloadChunk { coord } => {
                    chunks.remove(&coord);
                }
            }
        }
    });
}