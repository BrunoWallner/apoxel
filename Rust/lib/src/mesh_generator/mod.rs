mod gen;
mod sides;

use gdnative::prelude::*;

use crossbeam::channel;
use std::thread;
use crate::Coord;

pub type InternalChunk = (Coord, ByteArray);

#[derive(NativeClass)]
#[inherit(Node)]
pub struct MeshGenerator {
    mesh_sender: channel::Sender<Option<Ref<Spatial>>>,
    mesh_receiver: channel::Receiver<Option<Ref<Spatial>>>
}

#[methods]
impl MeshGenerator {
    fn new(_owner: &Node) -> Self {
        let (mesh_sender, mesh_receiver) = channel::unbounded();
        Self {
            mesh_sender,
            mesh_receiver,
        }
    }

    #[export]
    fn queue_chunk(&self, _owner: &Node, chunk: InternalChunk) {
        let sender = self.mesh_sender.clone();
        thread::spawn(move || {
            let mesh = gen::mesh(chunk);
            sender.send(mesh).unwrap();
        });
    }

    #[export]
    fn _process(&self, owner: &Node, _dt: f64) {
        if let Ok(mesh) = self.mesh_receiver.try_recv() {
            if let Some(mesh) = mesh {
                owner.add_child(mesh, false);
            }
        }
    }
}