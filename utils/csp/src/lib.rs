

pub mod v10;

pub mod queue;
pub mod traits;
pub mod parser;

// ======================= Version =======================

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Version {
    /// current version
    V10,
}

impl Version {
    pub fn from_u8(byte: u8) -> Option<Version> {
        match byte {
            32 => Some(Version::V10),
            _ => None,
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            Version::V10 => 32,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Version::V10 => "1.0",
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        Self::to_str(self).to_string()
    }
}

impl TryFrom<u8> for Version {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Version as TryFrom<u8>>::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl From<Version> for u8 {
    fn from(val: Version) -> Self {
        Version::to_u8(&val)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::V10
    }
}

// ======================= PacketHandler =======================

pub struct PacketHandler {
 // TODO PacketHandler
}