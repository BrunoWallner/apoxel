use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Error {
    Invalid, 
    Register,
    Login,
    Logoff,
}