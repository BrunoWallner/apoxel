use crate::channel::*;
use super::Instruction;
use std::thread;

pub(super) fn init(
    rx: Receiver<Instruction>,
) {
    use Instruction::*;
     
    thread::spawn(move || {
        while let Some(instruction) = rx.recv() {
            match instruction {
                RequestChunk { coord, sender } => {
                    todo!();
                }
            }
        }
    });
}