use bevy::prelude::*;
use super::Communicator;
use protocol::event::prelude::*;
use std::process::exit;

pub struct CommunicationPlugin;

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        let mut communicator = Communicator::init("localhost:8000").unwrap();
        communicator.send_event(Register{name: String::from("tsdarststr2")});

        // login
        while let Some(event) = communicator.get_event() {
            match event {
                Token(token) => {
                    communicator.set_token(token);
                    break
                },
                Error(err) => match err {
                    ClientError::Register => {
                        log::warn!("failed to register new user");
                        exit(1);
                    },
                    ClientError::Login => {
                        log::warn!("failed to log in");
                        exit(1);
                    }
                    _ => (),
                }
                ev => log::warn!("received unexpected event at login: {:?}", ev)
            }
        }
        communicator.send_event(Login{token: communicator.token});

        app.insert_resource(communicator);
    }
}
