use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientError {
    Register,
    Login,
    Logoff,
    ConnectionReset,
}