pub mod handle;
use protocol::Token;
use protocol::PlayerCoord;


#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub token: Token,
    pub pos: PlayerCoord,

    pub health: u8,

    pub online: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        let token = get_token();
        Self {
            name,
            token,
            pos: [0.0, 100.0, 0.0],
            health: 100,

            online: false,
        }
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.token == other.token
    }
}
impl Eq for Player {}

pub struct PlayerList {
    list: Vec<Player>,
}
impl PlayerList {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
        }
    }

    pub fn register(&mut self, name: String) -> Option<Token> {
        // only Ok because have custom implementation of Eq for Player
        // that only checks name
        if self.contains_name(&name).is_none() {
            let player = Player::new(name);
            let token = player.token.clone();
            self.list.push(player);
            return Some(token)
        } else {
            return None
        }
    }

    pub fn get_coords(&self) -> Vec<PlayerCoord> {
        self.list.iter().map(|x| x.pos).collect()
    }

    pub fn login(&mut self, token: &Token) -> bool {
        if let Some(i) = self.contains_token(token) {
            self.list[i].online = true;
            return true;
        } else {
            return false;
        }
    }

    pub fn logoff(&mut self, token: &Token) -> bool {
        if let Some(i) = self.contains_token(token) {
            self.list[i].online = false;
            return true;
        } else {
            return false;
        }
    }

    // TODO implement max speed
    pub fn move_player(&mut self, token: &Token, pos: PlayerCoord) {
        if let Some(i) = self.contains_token(token) {
            self.list[i].pos = pos;
        }
    }

    pub fn contains_token(&self, token: &Token) -> Option<usize> {
        for (i, val) in self.list.iter().enumerate() {
            if &val.token == token {
                return Some(i);
            }
        }
        return None
    }
    pub fn contains_name(&self, name: &String) -> Option<usize> {
        for (i, val) in self.list.iter().enumerate() {
            if &val.name == name {
                return Some(i);
            }
        }
        return None
    }
}

// currently not secure
fn get_token() -> Token {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut token: Token = [0_u8; 16];
    for i in 0..16 {
        token[i] = rng.gen();
    }
    
    token
}