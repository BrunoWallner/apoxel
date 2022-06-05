use crate::channel::*;
use super::Instruction;
use std::thread;
use std::collections::HashMap;
use protocol::Token;
use super::User;
use super::Instruction::*;
use super::UserModInstruction::*;

// should be selfe contained, and have no access to other handles like chunke or entity...
// to make sure it does not become spaghetti code
// must not panic so send errors are unimportant
pub(super) fn init(rx: Receiver<Instruction>) {
    thread::spawn(move || {
        // let users ...
        // loop
        // match incoming instructions
        let mut users: HashMap<Token, User> = HashMap::default();

        loop {
            match rx.recv() {
                Some(instruction) => match instruction {
                    Register{ name, token } => {
                        let mut already_exist: bool = false;
                        users.iter().for_each(|x| if x.1.name == name {already_exist = true});
                        if !already_exist {
                            let (tok, user) = User::new(name);
                            users.insert(tok.clone(), user);
                            let _ = token.send(Some(tok));
                        } else {
                            let _ = token.send(None);
                        }
                    },
                    Login { token, success } => {
                        if let Some(mut user) = users.get_mut(&token) {
                            user.online = true;
                            let _ = success.send(true);
                        } else {
                            let _ = success.send(false);
                        }
                    },
                    Logoff { token, success } => {
                        if let Some(mut user) = users.get_mut(&token) {
                            user.online = false;
                            let _ = success.send(true);
                        } else {
                            let _ = success.send(false);
                        }
                    },
                    GetUser { token, user } => {
                        if let Some(usr) = users.get(&token) {
                            let _ = user.send(Some(usr.clone()));
                        } else {
                            let _ = user.send(None);
                        }
                    },
                    ModUser { token, mod_instruction } => {
                        if let Some(mut user) = users.get_mut(&token) {
                            match mod_instruction {
                                Move(coord) => user.pos = coord,
                            }
                        }
                    }
                }
                None => {
                    // every clone of the 'Remote' has been dropped, terminating ...
                    break
                }
            }
        }
    });
}