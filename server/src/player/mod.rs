pub mod handle;
use protocol::Token;
use protocol::Coord;

#[derive(Hash, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub token: Token,
    pub coord: Coord,

    pub health: u8,

    pub online: bool,
}
impl Player {
    pub fn new(name: String) -> Self {
        let token = get_token();
        Self {
            name,
            token,
            coord: [0, 0, 0],
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

    pub fn register(&mut self, name: String) -> Option<[u8; 64]> {
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

    pub fn get_coords(&self) -> Vec<Coord> {
        self.list.iter().map(|x| x.coord).collect()
    }

    pub fn login(&mut self, token: &[u8; 64]) -> bool {
        if let Some(i) = self.contains_token(token) {
            self.list[i].online = true;
            return true;
        } else {
            return false;
        }
    }

    pub fn logoff(&mut self, token: &[u8; 64]) -> bool {
        if let Some(i) = self.contains_token(token) {
            self.list[i].online = false;
            return true;
        } else {
            return false;
        }
    }

    pub fn contains_token(&self, token: &[u8; 64]) -> Option<usize> {
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
    let mut token = [0_u8; 64];
    for i in 0..64 {
        token[i] = rng.gen();
    }
    
    token
}