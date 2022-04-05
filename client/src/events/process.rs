// computation heavy event processor
// for eg. chunk meshing

use crossbeam::channel;
use super::queue::GameEventQueue;
use crate::communication::CommunicationEvent;
use std::thread;

// send CommunicationEvent in, let it process and then let it 
// send to GameEventQueue, to fetch the GameEvent in Gameloop


pub fn init(
    ev_rx: channel::Receiver<CommunicationEvent>,
    queue: GameEventQueue,
) {
    thread::spawn(move || {
        loop {
            match ev_rx.recv().unwrap() {
                CommunicationEvent::ChunkUpdate(chunk) => {
                    // create mesh
                    // send it to queue
                }
                _ => ()
            }
        }
    });
}