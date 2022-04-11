pub mod client;

use client::Bridge;
use protocol::event::ClientToServer;

use gdnative::prelude::*;

// use std::thread;

/// The TcpClient "class"
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct TcpClient {
    bridge: Option<Bridge>,
}

#[methods]
impl TcpClient {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder
            .signal("Event")
            // can be omitted when only used by code and not GUI
            .with_param_default("Event", Variant::new(String::from("event")))
            .done();
    }

    fn new(_owner: &Node) -> Self {
        TcpClient { 
            bridge: None,
        }
    }

    #[export]
    fn establish_connection(&mut self, _owner: &Node, host: String) {
        let (rt, bridge) = client::init(host);
        core::mem::forget(rt);

        self.bridge = Some(bridge);
    }

    #[export]
    fn connection_established(&self, _owner: &Node) -> bool {
        self.bridge.is_some()
    }

    #[export]
    fn send(&self, _owner: &Node, event: String) -> bool {
        if let Some(bridge) = &self.bridge {
            let event: Result<ClientToServer, serde_json::Error> = serde_json::from_str(&event);

            match event {
                Ok(event) => {
                    bridge.send(event); 
                }
                Err(e) => {
                    godot_print!("attempted to send invalid tcp event:\n{}", e);
                }
            }
            return true;
        } else {
            return false;
        }
    }

    #[export]
    fn _process(&self, owner: &Node, _delta: f64) {
        if let Some(bridge) = &self.bridge {
            if let Some(event) = bridge.receive() {
                if let Ok(event) =  serde_json::to_string(&event) {
                    owner.emit_signal("Event", &[Variant::new(event)]);
                }
            }
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<TcpClient>();
}

godot_init!(init);