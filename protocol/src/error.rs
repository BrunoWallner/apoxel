#[derive(Copy, Clone, Debug)]
pub enum Error {
    Invalid, 
    Register,
    Login,
    Logoff,
}
impl Error {
    pub fn from_value(value: u8) -> Self {
        use Error::*;
        match value {
            0x01 => Register,
            0x02 => Login,
            0x03 => Logoff,
            _ => Invalid,
        }
    }

    pub fn to_value(&self) -> u8 {
        use Error::*;
        match self {
            Invalid => 0x00,
            Register => 0x01,
            Login => 0x02,
            Logoff => 0x03,
        }
    }
}