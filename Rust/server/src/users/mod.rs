mod init;

use crate::channel::*;
use protocol::{Token, PlayerCoord};


// type UserModFn = Arc<dyn Fn(&mut User) + Send + Sync>;
// type UserModFn = fn(&mut User);

#[derive(Debug, Clone)]
enum Instruction {
    Register{name: String, token: Sender<Option<Token>>},
    Login{token: Token, success: Sender<bool>},
    Logoff{token: Token, success: Sender<bool>},
    GetUser{token: Token, user: Sender<Option<User>>},
    ModUser{token: Token, mod_instruction: UserModInstruction },
}


#[derive(Debug, Clone)]
pub enum UserModInstruction {
    Move(PlayerCoord)
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub pos: PlayerCoord,

    pub health: u8,

    pub online: bool,
}
impl User {
    pub fn new(name: String) -> (Token, Self) {
        let token = User::gen_token();
        (
            token, 
            Self {
                name,
                pos: [0.0, 100.0, 0.0],
                health: 100,

                online: false,
            }
        )
    }

    // currently not secure
    fn gen_token() -> Token {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut token: Token = [0_u8; 16];
        for t in token.iter_mut() {
            *t = rng.gen();
        }

        token
    }
}

// clonable 'Remote' to user-handle, which runs on seperate thread
#[derive(Debug, Clone)]
pub struct Users {
    sender: Sender<Instruction>
}
impl Users {
    pub fn init() -> Self {
        let (tx, rx) = channel();
        init::init(rx);
        Self {
            sender: tx
        }
    }

    pub fn register(&self, name: String) -> Option<Token> {
        let (tx, rx) = channel();
        self.sender.send(Instruction::Register{name, token: tx}).unwrap();
        match rx.recv() {
            Some(success) => success,
            None => None
        }
    }

    pub fn login(&self, token: Token) -> bool {
        let (tx, rx) = channel();
        self.sender.send(Instruction::Login{token, success: tx}).unwrap();
        rx.recv().unwrap_or(false)
    }

    pub fn logoff(&self, token: Token) -> bool {
        let (tx, rx) = channel();
        self.sender.send(Instruction::Logoff{token, success: tx}).unwrap();
        rx.recv().unwrap_or(false)
    }

    pub fn get_user(&self, token: Token) -> Option<User> {
        let (tx, rx) = channel();
        self.sender.send(Instruction::GetUser{token, user: tx}).unwrap();
        match rx.recv() {
            Some(user) => user,
            None => None
        }
    }

    pub fn mod_user(&self, token: Token, mod_instruction: UserModInstruction) {
        self.sender.send(Instruction::ModUser{token, mod_instruction}).unwrap();
    }
}
