mod tcp_client;
mod chunk_handle;

use tcp_client::TcpClient;
use chunk_handle::ChunkHandle;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<TcpClient>();
    handle.add_class::<ChunkHandle>();
}

godot_init!(init);