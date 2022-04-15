use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChunkHandle {
    tcp_client: Option<Ref<Node>>,
}

#[methods]
impl ChunkHandle {
    fn new(_owner: &Node) -> Self {
        Self {
            tcp_client: None
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        let tcp_client = owner.get_node("../TcpWrapper/TcpClient").unwrap();
        self.tcp_client = Some(tcp_client);

        godot_print!("chunk handle init");
    }

    #[export]
    fn _process(&self, _owner: &Node, _dt: f64) {
        if let Some(client) = self.tcp_client {
            let client = unsafe {client.assume_safe().as_ref()};
            //if let Some(chunk_update) = client.fetch_chunk_update() {
            //    godot_print!("chunk update");
            //}
        }
    }
}