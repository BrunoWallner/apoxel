#[derive(Copy, Clone, Debug)]
pub enum Header {
    Invalid,
    Error,

    Register,
    Login,
    Logoff,

    ReceiveToken
}
impl Header {
    pub fn from_value(value: u8) -> Self {
        use Header::*;
        match value {
            0x01 => Error,
            0x02 => Register,
            0x03 => Login,
            0x04 => Logoff,
            0x05 => ReceiveToken,
            _ => Invalid,
        }
    }

    pub fn to_value(&self) -> u8 {
        use Header::*;
        match self {
            Invalid => 0x00,
            Error => 0x01,
            Register => 0x02,
            Login => 0x03,
            Logoff => 0x04,
            ReceiveToken => 0x05,
        }
    }
}
