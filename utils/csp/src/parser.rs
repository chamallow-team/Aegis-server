use std::{
    fmt,
    io::{self, BufReader, Read},
};

use crate::*;

// ======================= Errors =======================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorId {
    DupHeader,
    UkwnHeader,
    UkwnHeaderVal,
    UkwnCtrl,
    MissCtrl,
    UnxptCtrl,
    InvNum,
    InvDataLen,
    InvStr,
    Unknown,
}

impl ParserErrorId {
    pub fn description(&self) -> &'static str {
        match self {
            ParserErrorId::DupHeader => "Duplicated header: {1}.",
            ParserErrorId::UkwnHeader => "Unknown header: {1}.",
            ParserErrorId::UkwnHeaderVal => "Unknown value for [{1}]: {2}.",
            ParserErrorId::UkwnCtrl => "Unknown control: {1}.",
            ParserErrorId::MissCtrl => "Missing control: {1}.",
            ParserErrorId::UnxptCtrl => "Unexpected control: {1}.",
            ParserErrorId::InvNum => "Invalid number: expected {1} bytes, found {2}.",
            ParserErrorId::InvDataLen => "Invalid length header, data length mismatch it.",
            ParserErrorId::InvStr => "Invalid utf-8 string.",
            ParserErrorId::Unknown => "{1}",
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
            "INV_STR" => Some(ParserErrorId::InvStr),
            "UNKNOWN" => Some(ParserErrorId::Unknown),
            _ => None,
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
            ParserErrorId::InvStr => "INV_STR",
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
            id,
            pos,
        }
    }

    pub fn set_desc<T: ToString>(&mut self, p: &str, s: T) {
        match self.description.find(p) {
            Some(r) => self
                .description
                .replace_range(r..r + p.len(), s.to_string().as_str()),
            None => {}
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: At position {} {}",
            self.id, self.pos, self.description
        )
    }
}

// ======================= Parser =======================

pub struct Parser<T>
where
    T: io::Read,
{
    cursor: usize,
    buf: BufReader<T>,
}

pub type ParseResult<R> = std::result::Result<R, ParserError>;

impl<T: io::Read> Parser<T> {
    pub fn new(reader: T) -> Parser<T> {
        Parser {
            cursor: 0,
            buf: BufReader::new(reader),
        }
    }

    pub fn read(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0; size];

        let mut slice = buffer.as_mut_slice();
        let mut s: usize = 0;

        while !slice.is_empty() {
            match self.buf.read(&mut slice) {
                Ok(0) => break,
                Ok(n) => {
                    s += n;
                    self.cursor += n;
                    let tmp = slice;
                    slice = &mut tmp[n..];
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => {
                    // TODO: use a proper logger
                    eprint!("got {:?} when attempt to read buffer", e);
                    // FIXME: handle error, but what kind of error should we expect...?
                    slice.fill(0);
                }
            };
        }

        buffer.resize(s, 0);
        return buffer.to_vec();
    }

    pub fn pos(&self) -> usize {
        self.cursor
    }

    pub fn read_byte(&mut self) -> ParseResult<u8> {
        let buf = self.read(1);
        if buf.len() == 0 {
            let mut error = ParserError::new(ParserErrorId::UkwnHeader, self.pos());
            error.set_desc("{1}", "EOF");
            return Err(error);
        }

        return Ok(buf[0]);
    }

    pub fn read_string(&mut self) -> ParseResult<String> {
        let mut s = vec![];
        let mut start = false;

        loop {
            let buf = self.read(1);

            if buf.len() == 0 {
                let mut error = ParserError::new(ParserErrorId::MissCtrl, self.pos());
                if s.len() == 0 {
                    error.set_desc("{1}", Control::StringStart.to_string());
                } else {
                    error.set_desc("{1}", Control::StringEnd.to_string());
                }
                return Err(error);
            }

            if start {
                if buf[0] == Control::StringEnd.into() {
                    break;
                }
                s.push(buf[0]);
            } else if buf[0] == Control::StringStart.into() {
                start = true;
            } else {
                let mut error = ParserError::new(ParserErrorId::MissCtrl, self.pos());
                error.set_desc("{1}", Control::StringStart.to_string());
                return Err(error);
            }
        }

        match String::from_utf8(s) {
            Ok(s) => Ok(s),
            Err(_) => {
                let error = ParserError::new(ParserErrorId::InvStr, self.pos());
                return Err(error);
            }
        }
    }

    pub fn read_u32(&mut self) -> ParseResult<u32> {
        let mut buf = self.read(4);
        if buf.len() < 4 {
            let mut error = ParserError::new(ParserErrorId::InvNum, self.pos());
            error.set_desc("{1}", "4");
            error.set_desc("{2}", buf.len());
            return Err(error);
        }
        let mut bytes: [u8; 4] = [0; 4];
        bytes.clone_from_slice(&mut buf[..4]);

        return Ok(u32::from_le_bytes(bytes));
    }

    pub fn read_u64(&mut self) -> ParseResult<u64> {
        let mut buf = self.read(8);
        if buf.len() < 8 {
            let mut error = ParserError::new(ParserErrorId::InvNum, self.pos());
            error.set_desc("{1}", "8");
            error.set_desc("{2}", buf.len());
            return Err(error);
        }
        let mut bytes: [u8; 8] = [0; 8];
        bytes.clone_from_slice(&mut buf[..8]);

        return Ok(u64::from_le_bytes(bytes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_read() {
        let reader: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut parser = Parser::new(reader);

        assert_eq!(parser.read(2), vec![1, 2]);
        assert_eq!(2, parser.pos());

        assert_eq!(parser.read(3), vec![3, 4, 5]);
        assert_eq!(5, parser.pos());

        assert_eq!(parser.read(20), vec![6, 7, 8, 9, 10]);
        assert_eq!(10, parser.pos());

        assert_eq!(parser.read(2), vec![]);
        assert_eq!(10, parser.pos());
    }

    #[test]
    fn parser_parse_byte() {
        let reader: &[u8] = &[32];

        let mut parser = Parser::new(reader);
        assert_eq!(parser.read_byte(), Ok(32));
        assert_eq!(
            parser.read_byte().unwrap_err().id,
            ParserErrorId::UkwnHeader
        );
    }

    #[test]
    fn parser_parse_string() {
        let reader1: &[u8] = &[3, 49, 46, 48, 46, 51, 4];
        let reader2: &[u8] = &[3, 195, 169, 4];
        let reader3: &[u8] = &[49, 46, 48, 46, 51, 4];
        let reader4: &[u8] = &[3, 49, 46, 48, 46, 51];
        let reader5: &[u8] = &[3, 0, 195, 4];

        let mut parser1 = Parser::new(reader1);
        let mut parser2 = Parser::new(reader2);
        let mut parser3 = Parser::new(reader3);
        let mut parser4 = Parser::new(reader4);
        let mut parser5 = Parser::new(reader5);

        assert_eq!(parser1.read_string(), Ok("1.0.3".to_string()));
        assert_eq!(parser2.read_string(), Ok("é".to_string()));
        assert_eq!(
            parser3.read_string().unwrap_err().id,
            ParserErrorId::MissCtrl
        );
        assert_eq!(
            parser4.read_string().unwrap_err().id,
            ParserErrorId::MissCtrl
        );
        assert_eq!(parser5.read_string().unwrap_err().id, ParserErrorId::InvStr);
    }

    #[test]
    fn parser_parse_u32() {
        let reader1: &[u8] = &[11, 0, 45, 05];
        let reader2: &[u8] = &[22, 05, 0];

        let mut parser1 = Parser::new(reader1);
        let mut parser2 = Parser::new(reader2);

        assert_eq!(parser1.read_u32(), Ok(86835211));
        assert_eq!(parser2.read_u32().unwrap_err().id, ParserErrorId::InvNum);
    }

    #[test]
    fn parser_parse_u64() {
        let reader1: &[u8] = &[11, 0, 45, 05, 0, 3, 0, 0];
        let reader2: &[u8] = &[22, 05, 0, 0, 5, 33];

        let mut parser1 = Parser::new(reader1);
        let mut parser2 = Parser::new(reader2);

        assert_eq!(parser1.read_u64(), Ok(3298621718539));
        assert_eq!(parser2.read_u64().unwrap_err().id, ParserErrorId::InvNum);
    }
}
