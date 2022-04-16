mod gen;
mod sides;

use gdnative::prelude::*;

use crossbeam::channel;
use std::thread;
use crate::Coord;

pub type InternalChunk = (Coord, ByteArray);
// mesh coord, verts, uvs, normals, indices
pub type MeshData = (Coord, Vector3Array, Vector2Array, Vector3Array, PoolArray<i32>);

#[derive(NativeClass)]
#[inherit(Node)]
pub struct MeshGenerator {
    mesh_sender: channel::Sender<MeshData>,
    mesh_receiver: channel::Receiver<MeshData>
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
    fn fetch_mesh(&self, _owner: &Node) -> Option<MeshData> {
        match self.mesh_receiver.try_recv() {
            Ok(mesh) => Some(mesh),
            Err(_) => None,
        }
    }
}