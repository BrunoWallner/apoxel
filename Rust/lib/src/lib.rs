mod tcp_client;

use tcp_client::TcpClient;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<TcpClient>();
}

godot_init!(init);