mod chunk_loader;

use crate::chunks::ChunkHandle;
use crate::queque::Queque;
use crate::users::UserModInstruction;
use crate::users::Users;
use protocol::error::ClientError;
use protocol::event::ServerToClient;
use protocol::event::prelude::*;
use protocol::Token;
use std::net::SocketAddr;

use log::*;

// this function acts as kind of like a bridge between sync and async code
// the client part runs in the tokio runtime, to make many simultanious connectins possible
// all the handles run on seperate os threads, for performance predictability and ease of use reasons
pub async fn init(
    reader: Queque<ClientToServer>,
    writer: Queque<ServerToClient>,
    addr: SocketAddr,
    users: Users,
    chunk_handle: ChunkHandle,
) {
    let mut user_token: Option<Token> = None; // for loggin off in case of unexpected disonnection
    let mut user_name: Option<String> = None;

    let mut chunk_loader = chunk_loader::ChunkLoader::new(chunk_handle.clone(), writer.clone());

    while let Some(event) = reader.recv() {
        // log::info!("read: {} MB", reader.bytes_read() as f64 / 1_000_000.0);
        use protocol::event::ClientToServer::*;
        match event {
            Register { name } => {
                if let Some(token) = users.register(name.clone()) {
                    user_token = Some(token);
                    user_name = Some(name);
                    writer
                        .send(ServerToClient::Token(token), false)
                        .unwrap();
                } else {
                    writer
                        .send(ServerToClient::Error(ClientError::Register), false)
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
                    writer
                        .send(ServerToClient::Error(ClientError::Login), false)
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
                    writer
                        .send(ServerToClient::Error(ClientError::ConnectionReset), false)
                        .unwrap();
                    break;
                }
            }
            PlaceStructure { coord, structure } => {
                if let Some(token) = user_token {
                    chunk_handle.place_structure(coord, structure, token)
                } else {
                    warn!(
                        "[{}][{}]: auth violation detected!",
                        user_name.unwrap_or_else(|| String::from("")),
                        addr
                    );
                    writer
                        .send(ServerToClient::Error(ClientError::ConnectionReset), false)
                        .unwrap();
                    break;
                }
            }
            Disconnect => break,
        }
    }
    // USER DISCONNECTION HANDLING
    if let Some(token) = user_token {
        users.logoff(token);
        chunk_loader.unload_all_chunks(token);
    }
}
