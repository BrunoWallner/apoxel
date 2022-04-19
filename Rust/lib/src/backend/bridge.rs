use crossbeam::channel;

use protocol::event::{ServerToClient, ClientToServer};

#[derive(Clone, Debug)]
pub struct Bridge {
    pub event_receiver: channel::Receiver<ServerToClient>,
    pub event_sender: channel::Sender<ClientToServer>,
}
impl Bridge {
    pub fn receive(&self) -> Option<ServerToClient> {
        match self.event_receiver.try_recv() {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }
    
    pub fn send(&self, event: ClientToServer) {
        //godot_print!("event input");
        let _ = self.event_sender.send(event);
    }
}