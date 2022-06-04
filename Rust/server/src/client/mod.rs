use std::net::SocketAddr;

use tokio::net::tcp::OwnedReadHalf;
use crate::channel::Sender;
use protocol::reader::Reader;
use protocol::event::{Event, ServerToClient};
use protocol::error::ClientError;
use protocol::Token;
use crate::users::Users;

use log::*;

// this function acts as kind of like a bridge between sync and async code
// the client part runs in the tokio runtime, to make many simultanious connectins possible
// all the handles run on seperate os threads, for performance predictability and ease of use reasons
pub async fn init(
    rw: (Reader<OwnedReadHalf>, Sender<Event>),
    addr: SocketAddr,
    users: Users,
) {
    let mut reader = rw.0;
    let sender = rw.1;

    let mut user_token: Option<Token> = None; // for loggin off in case of unexpected disonnect
    let mut user_name: Option<String> = None;

    loop {
        if let Ok(event) = reader.get_event().await {
            info!("new event just dropped: {:#?}", event);
            match event {
                Event::ClientToServer(event) => {
                    use protocol::event::ClientToServer::*;
                    match event {
                        Register { name } => {
                            if let Some(token) = users.register(name.clone()) {
                                user_token = Some(token);
                                user_name = Some(name);
                                sender.send(Event::ServerToClient(ServerToClient::Token(token))).unwrap();
                            } else {
                                sender.send(Event::ServerToClient(ServerToClient::Error(ClientError::Register))).unwrap();
                            }
                        },
                        Login { token } => {
                            user_token = Some(token);
                            // user_name = 

                            if users.login(token) {
                                user_token = Some(token);
                                user_name = match users.get_user(token) {
                                    Some(user) => Some(user.name),
                                    None => None
                                };
                            } else {
                                sender.send(Event::ServerToClient(ServerToClient::Error(ClientError::Login))).unwrap();
                            }
                        },
                        Move { coord } => {
                            if user_token.is_some() {

                            } else {
                                warn!("[{}][{}]: auth violation detected!", user_name.unwrap_or(String::from("")), addr);
                                sender.send(Event::ServerToClient(ServerToClient::Error(ClientError::ConnectionReset))).unwrap();
                                break;
                            }
                        },
                        PlaceStructure { pos, structure } => {
                            if user_token.is_some() {

                            } else {
                                warn!("[{}][{}]: auth violation detected!", user_name.unwrap_or(String::from("")), addr);
                                sender.send(Event::ServerToClient(ServerToClient::Error(ClientError::ConnectionReset))).unwrap();
                                break;
                            }
                        }
                    }
                }
                Event::ServerToClient(event) => {
                    warn!("{} sent an invalid event: {:?}", addr, event)
                }
                Event::Invalid => {
                    warn!("{} sent an invalid event", addr)
                }
            }
        } else {
            // client disconnected, preventing empty inv loop, important
            if let Some(token) = user_token {
                users.logoff(token);
            }
            break
        }
    }
}