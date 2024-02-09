use std::{fmt, io};

use crate::Packet;


// ======================= Errors =======================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorId {
    DupHeader, UkwnHeader, UkwnHeaderVal, UkwnCtrl, MissCtrl, UnxptCtrl, InvNum, InvDataLen, Unknown
}

impl ParserErrorId {
    pub fn description(&self) -> &'static str{
        match self {
            ParserErrorId::DupHeader => "Duplicated header: {1}.",
            ParserErrorId::UkwnHeader => "Unknown header: {1}.",
            ParserErrorId::UkwnHeaderVal => "Unknown value for [{1}]: {2}.",
            ParserErrorId::UkwnCtrl => "Unknown control: {1}.",
            ParserErrorId::MissCtrl => "Missing control: {1}.",
            ParserErrorId::UnxptCtrl => "Unexpected control: {1}.",
            ParserErrorId::InvNum => "Invalid number: expected {1} bytes, found {2}.",
            ParserErrorId::InvDataLen => "Invalid length header, data length mismatch it.",
            ParserErrorId::Unknown => "",
        }
    }

    pub fn from_str<T: ToString>(id: T) -> Option<ParserErrorId> {
        let id = id.to_string().to_uppercase();

        match id.as_str() {
            "DUP_HEADER" => Some(ParserErrorId::DupHeader),
            "UKWN_HEADER" => Some(ParserErrorId::UkwnHeader),
            "UKWN_HEADER_VAL" => Some(ParserErrorId::UkwnHeaderVal),
            "UKWN_CTRL" => Some(ParserErrorId::UkwnCtrl),
            "MISS_CTRL" => Some(ParserErrorId::MissCtrl),
            "UNXPT_CTRL" => Some(ParserErrorId::UnxptCtrl),
            "INV_NUM" => Some(ParserErrorId::InvNum),
            "INV_DATA_LEN" => Some(ParserErrorId::InvDataLen),
            "UNKNOWN" => Some(ParserErrorId::Unknown),
            _=> None
        }
    }

    pub fn to_str(id: &ParserErrorId) -> &'static str {
        match id {
            ParserErrorId::DupHeader => "DUP_HEADER",
            ParserErrorId::UkwnHeader => "UKWN_HEADER",
            ParserErrorId::UkwnHeaderVal => "UKWN_HEADER_VAL",
            ParserErrorId::UkwnCtrl => "UKWN_CTRL",
            ParserErrorId::MissCtrl => "MISS_CTRL",
            ParserErrorId::UnxptCtrl => "UNXPT_CTRL",
            ParserErrorId::InvNum => "INV_NUM",
            ParserErrorId::InvDataLen => "INV_DATA_LEN",
            ParserErrorId::Unknown => "UNKNOWN",
        }
    }
}

impl ToString for ParserErrorId {
    fn to_string(&self) -> String {
        Self::to_str(self).to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub id: ParserErrorId,
    pub description: String,
    pub pos: usize,
}

impl ParserError {
    pub fn new(id: ParserErrorId, pos: usize) -> ParserError {
        ParserError {
            description: id.description().to_string(),
            id, pos,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: At position {} {}", self.id, self.pos, self.description)
    }
}


// ======================= Parser =======================

pub struct Parser<'a, T> where T: io::Read {
  pos: usize,
  reader: T,
  buffer: Vec<u8>,
  capacity: usize,
  packet: &'a Packet
}

pub type Result<R> = std::result::Result<R, ParserError>;

impl<'a, 'b, T: io::Read> Parser<'a, T> {

  pub fn new(reader: T, packet: &'a mut Packet) -> Parser<'a, T> {
    if !packet.is_empty() {
      packet.clear()
    }
    let capacity = 1024;

    Parser {
      pos: 0,
      capacity,
      buffer: Vec::with_capacity(capacity),
      packet, reader  
    }
  }

  pub fn peek(&mut self, size: usize) -> Result<Vec<u8>> {
    todo!()
  }

  pub fn consume(&mut self, size: usize) -> Result<Vec<u8>> {
    todo!()
  }


  pub fn bound(&mut self) -> Result<Vec<u8>> {
    todo!()
  }

  pub fn peek_byte(&self) -> Result<u8> {
    todo!()
  }

  pub fn consume_byte(&mut self) -> Result<u8> {
    todo!()
  }

  pub fn peek_string(&mut self) -> Result<String> {
    todo!()
  }

  pub fn consume_string(&mut self) -> Result<String> {
    todo!()
  }

  pub fn peek_u32(&mut self) -> Result<u32> {
    todo!()
  }

  pub fn consume_u32(&mut self) -> Result<u32> {
    todo!()
  }

  pub fn peek_u64(&mut self) -> Result<u64> {
    todo!()
  }

  pub fn consume_u64(&mut self) -> Result<u64> {
    todo!()
  }


  pub fn len(&self) -> Result<u64> {
    todo!()
  }

  pub fn capacity(&self) -> Result<u64> {
    todo!()
  }

  pub fn set_capacity(&mut self) -> Result<u64> {
    todo!()
  }

  pub fn clear_buf(&mut self) -> Result<u64> {
    todo!()
  }

  pub fn fill_buf(&mut self) -> Result<u64> {
    todo!()
  }

  pub fn reset(&mut self) -> Result<u64> {
    todo!()
  }
}




#[cfg(test)]
mod tests {

}