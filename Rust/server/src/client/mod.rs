use crate::CONFIG;
use crate::channel::Sender;
use crate::chunks::ChunkHandle;
use crate::users::UserModInstruction;
use crate::users::Users;
use protocol::error::ClientError;
use protocol::event::{Event, ServerToClient};
use protocol::reader::Reader;
use protocol::{chunk::CHUNK_SIZE, PlayerCoord, Token};
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

    let mut last_chunkload_pos: PlayerCoord = [0.0f64; 3];

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
                                    last_chunkload_pos = [
                                        user.pos[0],
                                        user.pos[1] + CHUNK_SIZE as f64 + 1.0,
                                        user.pos[2],
                                    ];
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

                        // chunkload
                        let distance: f64 =
                            protocol::calculate_distance(&coord, &last_chunkload_pos);
                        if distance as usize > CHUNK_SIZE / 4 {
                            last_chunkload_pos = coord;

                            let origin = protocol::chunk::get_chunk_coords(&[
                                coord[0] as i64,
                                coord[1] as i64,
                                coord[2] as i64,
                            ]).0;
                            let offset = CONFIG.chunks.render_distance as i64;
                            for x in origin[0] - offset..=origin[0] + offset {
                                for y in origin[1] - offset..=origin[1] + offset {
                                    for z in origin[2] - offset..=origin[2] + offset {
                                        if let Some(chunk) = chunk_handle.request_chunk([x, y, z]).unwrap() {
                                            let _ = sender.send(Event::ServerToClient(ServerToClient::ChunkUpdate(chunk)));
                                        }
                                    }
                                }
                            }
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
    // log user off when no further event can be fetched(disconnection)
    if let Some(token) = user_token {
        users.logoff(token);
    }
}
