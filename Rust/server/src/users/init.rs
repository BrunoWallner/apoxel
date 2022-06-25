use super::Instruction;
use super::Instruction as UserInstruction;
use super::User;
use super::UserModInstruction;
use protocol::prelude::*;
use protocol::Token;
use std::collections::HashMap;
use std::thread;

// should be selfe contained, and have no access to other handles like chunke or entity...
// to make sure it does not become spaghetti code
// must not panic so send errors are unimportant
pub(super) fn init(rx: Receiver<Instruction>) {
    thread::spawn(move || {
        let mut users: HashMap<Token, User> = HashMap::default();
        loop {
            let instruction = rx.recv().unwrap();
            match instruction {
                UserInstruction::Register { name, token } => {
                    let mut already_exist: bool = false;
                    users.iter().for_each(|x| {
                        if x.1.name == name {
                            already_exist = true
                        }
                    });
                    if !already_exist {
                        let (tok, user) = User::new(name);
                        users.insert(tok, user);
                        let _ = token.send(Some(tok), false);
                    } else {
                        let _ = token.send(None, false);
                    }
                }
                UserInstruction::Login { token, success } => {
                    if let Some(mut user) = users.get_mut(&token) {
                        if !user.online {
                            user.online = true;
                            let _ = success.send(true, false);
                        } else {
                            let _ = success.send(false, false);
                        }
                    } else {
                        let _ = success.send(false, false);
                    }
                }
                UserInstruction::Logoff { token, success } => {
                    if let Some(mut user) = users.get_mut(&token) {
                        user.online = false;
                        let _ = success.send(true, false);
                    } else {
                        let _ = success.send(false, false);
                    }
                }
                UserInstruction::GetUser { token, user } => {
                    if let Some(usr) = users.get(&token) {
                        let _ = user.send(Some(usr.clone()), false);
                    } else {
                        let _ = user.send(None, false);
                    }
                }
                UserInstruction::ModUser {
                    token,
                    mod_instruction,
                } => {
                    if let Some(mut user) = users.get_mut(&token) {
                        match mod_instruction {
                            UserModInstruction::Move(coord) => user.pos = coord,
                        }
                    }
                }
            }
        }
    });

}
