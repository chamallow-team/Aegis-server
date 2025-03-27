use queue::Queue;
use std::fmt::Display;

// === exports ===
// csp versions
pub mod v10;

// utilities
pub mod parser;
pub mod queue;
pub mod traits;

// traits
pub use crate::traits::*;

// ======================= Version =======================

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Version {
    /// current version
    V10,
}

impl Version {
    /// return the Version from byte representation
    ///
    /// # Example
    /// ```
    /// use csp::Version;
    /// let v10 = Version::from_u8(32);
    /// let errored = Version::from_u8(3);
    ///
    /// assert_eq!(v10, Some(Version::V10));
    /// assert_eq!(errored, None);
    /// ```
    pub fn from_u8(byte: u8) -> Option<Version> {
        match byte {
            32 => Some(Version::V10),
            _ => None,
        }
    }

    /// returns the byte representation of a version
    ///
    /// # Example
    /// ```
    /// use csp::Version;
    /// let byte = Version::V10.to_u8();
    ///
    /// assert_eq!(byte, 32);
    /// ```
    pub fn to_u8(&self) -> u8 {
        match self {
            Version::V10 => 32,
        }
    }

    /// returns human readable str of a version
    ///
    /// # Example
    /// ```
    /// use csp::Version;
    /// let s = Version::V10.to_str();
    ///
    /// assert_eq!(s, "1.0");
    /// ```
    pub fn to_str(&self) -> &'static str {
        match self {
            Version::V10 => "1.0",
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::to_str(self))
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

pub struct PacketqHandler {
    #[allow(dead_code)]
    reicv_queue: Queue,
    #[allow(dead_code)]
    send_queue: Queue,
}
