mod chunk_loader;

use crate::channel::*;
use crate::chunks::ChunkHandle;
use crate::users::UserModInstruction;
use crate::users::Users;
use protocol::error::ClientError;
use protocol::event::{Event, ServerToClient};
use protocol::reader::Reader;
use protocol::Coord;
use protocol::Token;
use std::net::SocketAddr;
use tokio::net::tcp::OwnedReadHalf;

use log::*;

// this function acts as kind of like a bridge between sync and async code
// the client part runs in the tokio runtime, to make many simultanious connectins possible
// all the handles run on seperate os threads, for performance predictability and ease of use reasons
pub async fn init(
    rw: (Reader<OwnedReadHalf>, Sender<Event>),
    addr: SocketAddr,
    users: Users,
    chunk_handle: ChunkHandle,
) {
    let mut reader = rw.0;
    let sender = rw.1;

    let mut user_token: Option<Token> = None; // for loggin off in case of unexpected disonnection
    let mut user_name: Option<String> = None;

    let mut chunk_loader = chunk_loader::ChunkLoader::new(chunk_handle, sender.clone());

    while let Ok(event) = reader.get_event().await {
        match event {
            Event::ClientToServer(event) => {
                use protocol::event::ClientToServer::*;
                match event {
                    Register { name } => {
                        if let Some(token) = users.register(name.clone()) {
                            user_token = Some(token);
                            user_name = Some(name);
                            sender
                                .send(Event::ServerToClient(ServerToClient::Token(token)))
                                .unwrap();
                        } else {
                            sender
                                .send(Event::ServerToClient(ServerToClient::Error(
                                    ClientError::Register,
                                )))
                                .unwrap();
                        }
                    }
                    Login { token } => {
                        user_token = Some(token);

                        if users.login(token) {
                            user_token = Some(token);
                            user_name = match users.get_user(token) {
                                Some(user) => {
                                    let name = user.name;
                                    info!("{} logged in at: {:?}", name, user.pos);
                                    // set chunkload pos to trigger initial load
                                    chunk_loader.set_player_pos(user.pos);
                                    Some(name)
                                }
                                None => None,
                            };
                        } else {
                            sender
                                .send(Event::ServerToClient(ServerToClient::Error(
                                    ClientError::Login,
                                )))
                                .unwrap();
                        }
                    }
                    // also triggers chunkload
                    Move { coord } => {
                        if let Some(token) = user_token {
                            users.mod_user(token, UserModInstruction::Move(coord));

                            // chunk request
                            chunk_loader.update_position(coord, token);
                        } else {
                            warn!(
                                "[{}][{}]: auth violation detected!",
                                user_name.unwrap_or_else(|| String::from("")),
                                addr
                            );
                            sender
                                .send(Event::ServerToClient(ServerToClient::Error(
                                    ClientError::ConnectionReset,
                                )))
                                .unwrap();
                            break;
                        }
                    }
                    #[allow(unused_variables)]
                    PlaceStructure { pos, structure } => {
                        if user_token.is_some() {
                        } else {
                            warn!(
                                "[{}][{}]: auth violation detected!",
                                user_name.unwrap_or_else(|| String::from("")),
                                addr
                            );
                            sender
                                .send(Event::ServerToClient(ServerToClient::Error(
                                    ClientError::ConnectionReset,
                                )))
                                .unwrap();
                            break;
                        }
                    }
                    Disconnect => break,
                }
            }
            Event::ServerToClient(event) => {
                warn!("{} sent an invalid event: {:?}", addr, event)
            }
            Event::Invalid => {
                warn!("{} sent an invalid event", addr)
            }
        }
    }
    // USER DISCONNECTION HANDLING
    if let Some(token) = user_token {
        users.logoff(token);
    }
}
