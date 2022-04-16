#[macro_use]
extern crate lazy_static;

mod tcp_client;
mod mesh_generator;

use tcp_client::TcpClient;
use mesh_generator::MeshGenerator;

use gdnative::prelude::*;

pub type Coord = PoolArray<i32>;

fn init(handle: InitHandle) {
    handle.add_class::<TcpClient>();
    handle.add_class::<MeshGenerator>();
}

godot_init!(init);