use tokio::sync::mpsc;
use protocol::{Token, PlayerCoord};

#[derive(Clone, Debug)]
pub enum Request {
    // input name and get token back if success
    Register{name: String, sender: mpsc::Sender<Option<Token>>},
    // input token to log off, returns true if success
    Login{token: Token, sender: mpsc::Sender<bool>},
    // input token to log off, returns true if success
    Logoff{token: Token, sender: mpsc::Sender<bool>},

    Move{token: Token, pos: PlayerCoord},

    RequestCoords(mpsc::Sender<Vec<PlayerCoord>>)
}

// this part runs on main thread
#[derive(Clone, Debug)]
pub struct Handle {
    player_sender: mpsc::Sender<Request>,
}
impl Handle {
    pub fn init() -> Self {
        let (sender, receiver) = mpsc::channel(4096);

        // spawns handler on own thread
        tokio::spawn(async move {
            init(receiver).await;
        });

        Handle {
            player_sender: sender,
        }
    }

    // returns token
    pub async fn register(&self, name: String) -> Option<Token> {
        let (tx, mut rx) = mpsc::channel(1);
        self.player_sender.send(
            Request::Register {
                name,
                sender: tx,
            }
        ).await.unwrap();
        rx.recv().await.unwrap()
    }

    // returns true if success
    pub async fn login(&self, token: Token) -> bool {
        let (tx, mut rx) = mpsc::channel(1);
        self.player_sender.send(
            Request::Login {
                token,
                sender: tx,
            }
        ).await.unwrap();
        rx.recv().await.unwrap()
    }

    // returns true if success
    pub async fn logoff(&self, token: Token) -> bool {
        let (tx, mut rx) = mpsc::channel(1);
        self.player_sender.send(
            Request::Logoff{
                token,
                sender: tx,
            }
        ).await.unwrap();
        rx.recv().await.unwrap()
    }

    pub async fn move_player(&self, token: Token, pos: PlayerCoord) {
        self.player_sender.send(
            Request::Move{token, pos}
        ).await.unwrap();
    }

    pub async fn get_coords(&self) -> Vec<PlayerCoord> {
        let (tx, mut rx) = mpsc::channel(1);
        self.player_sender.send(
            Request::RequestCoords(tx)
        ).await.unwrap();
        rx.recv().await.unwrap()
    }
}

// this runs on another task
async fn init(mut receiver: mpsc::Receiver<Request>) {
    use Request::*;

    let mut player_list = super::PlayerList::new();
    
    loop {
        match receiver.recv().await {
            Some(request) => match request {
                Register{name, sender} => {
                    let token = player_list.register(name);
                    sender.send(token).await.unwrap();
                },
                Login{token, sender} => {
                    let success = player_list.login(&token);
                    sender.send(success).await.unwrap();
                },
                Logoff{token, sender} => {
                    let success = player_list.logoff(&token);
                    sender.send(success).await.unwrap();
                },
                RequestCoords(sender) => {
                    let coords = player_list.get_coords();
                    sender.send(coords).await.unwrap();
                },
                Move{token, pos} => {
                    player_list.move_player(&token, pos);
                }
            },
            None => {
                panic!("player handler panicked");
            }
        }
    }
}