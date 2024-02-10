use std::{
    fmt,
    io::{self, BufReader, Read},
};

use smol::io::{AsyncRead, AsyncReadExt, BufReader as AsyncBufReader};

use crate::*;

// ======================= Errors =======================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorId {
    /// duplicated headers are forbiden
    DupHeader,
    /// headers have statics value between 32 and 255 (u8 max)
    /// only a few are valid depending of the csp version
    UkwnHeader,
    /// single byte headers value have statics value between 32 and 255 (u8 max)
    /// only a few are valid depending of the csp version and header
    UkwnHeaderVal,
    /// controls have statics value between 1 and 31
    /// only a few are valid depending of the csp version
    UkwnCtrl,
    /// sometimes, controls are expected, such in a string, data start...
    MissCtrl,
    /// controls that are not expected here
    UnxptCtrl,
    /// invalid number, mostly caused by EOF before fully reading an n bytes number
    InvNum,
    /// invalid Length header and data length, they should be roughtly the same
    InvDataLen,
    /// motly caused by an invalid utf-8 string
    InvStr,
    /// for all error that don't have an id to report
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
    /// id of the error, mostly used by the software to handle specific error
    pub id: ParserErrorId,
    /// description, used for debugging, or reporting the error to the user
    pub description: String,
    /// same use ad description, to report where the error occured
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

pub type ParseResult<R> = std::result::Result<R, ParserError>;

// FIXME: might be useless
/// does nothing except telling that those who implement this trait are csp parser
pub trait CspParser {
    /// tells the position of the parser's cursor
    ///
    /// # Example
    /// ```rs
    /// let reader: &[u8] = &[1,2,3,4,5,6,7];
    /// let mut parser = csp::parser::Parser::new(reader);
    /// 
    /// assert_eq!(parser.pos(), 0);
    /// 
    /// parser.read(3);
    /// assert_eq!(parser.pos(), 3);
    /// ```
    fn pos(&self) -> usize;

    /// reset the parser's cursor and return the old cursor
    /// used when a packet is parsed and we want to reuse the parser for parsing other packets
    ///
    /// # Example
    /// ```rs
    /// let reader: &[u8] = &[1,2,3,4,5,6,7];
    /// let mut parser = csp::parser::Parser::new(reader);
    /// 
    /// assert_eq!(parser.pos(), 0);
    /// 
    /// parser.read(3);
    /// assert_eq!(parser.pos(), 3);
    /// 
    /// assert_eq!(parser.reset(), 3);
    /// assert_eq!(parser.pos(), 0);
    /// ```
    fn reset(&mut self) -> usize;
}

// ======================= Parser Sync =======================

pub struct Parser<T>
where
    T: Read,
{
    cursor: usize,
    buf: BufReader<T>,
}

impl<T: Read> Parser<T> {
    /// create a new parser
    ///
    /// # Arguments
    /// * `reader` - a type that implement Read
    ///
    /// # Example
    /// ```
    /// let reader1: &[u8] = &[3, 49, 46, 48, 46, 51, 4];
    /// let reader2 = std::fs::File::open("Cargo.toml").expect("cargo.toml: no file or directory");
    /// // # that works too
    /// // let reader3 = std::new::TcpStream::connect("127.0.0.0:8080")
    /// //    .expect("failed to open connection to 127.0.0.0:8080");
    ///  
    /// let parser1 = csp::parser::Parser::new(reader1);
    /// let parser2 = csp::parser::Parser::new(reader2);
    /// ```
    pub fn new(reader: T) -> Parser<T> {
        Parser {
            cursor: 0,
            buf: BufReader::new(reader),
        }
    }

    /// read exactly `size` bytes, unless the reader hit EOF, where it return the last bytes
    /// when reading 0 bytes, that means that the reader has no longer data
    /// -> on TcpStream, that means the connection has been closed
    ///
    /// # Arguments
    /// * `size` - number of bytes to read
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[1,2,3,4,5,6,7];
    /// let mut parser = csp::parser::Parser::new(reader);
    /// 
    /// assert_eq!(parser.read(3), vec![1,2,3]);
    /// assert_eq!(parser.read(2), vec![4,5]);
    /// assert_eq!(parser.read(20), vec![6,7]);
    /// assert_eq!(parser.read(2), vec![]);
    /// ```
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
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => {
                    // TODO: use a proper logger
                    eprint!("got {:?} when attempt to read buffer", e);
                    // FIXME: handle error, but what kind of error should we expect...?
                    s = 0;
                }
            };
        }

        buffer.resize(s, 0);
        return buffer;
    }
    
    /// read exactly one byte
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[1];
    /// let mut parser = csp::parser::Parser::new(reader);
    /// 
    /// assert_eq!(parser.read_byte(), Ok(1));
    /// assert_eq!(parser.read_byte().unwrap_err().id, csp::parser::ParserErrorId::UkwnHeader);
    /// ```
    pub fn read_byte(&mut self) -> ParseResult<u8> {
        let buf = self.read(1);
        if buf.len() == 0 {
            let mut error = ParserError::new(ParserErrorId::UkwnHeader, self.pos());
            error.set_desc("{1}", "EOF");
            return Err(error);
        }

        return Ok(buf[0]);
    }

    /// read a csp string
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[3, 49, 46, 48, 46, 51, 4];
    /// let mut parser = csp::parser::Parser::new(reader);
    /// 
    /// assert_eq!(parser.read_string(), Ok("1.0.3".to_string()));
    /// ```
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

    /// read 4 bytes and parse them as little endian u32
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[16, 4, 0, 0, 2];
    /// let mut parser = csp::parser::Parser::new(reader);
    /// 
    /// assert_eq!(parser.read_u32(), Ok(1040));
    /// ```
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

    /// read 8 bytes and parse them as little endian u64
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[16, 4, 0, 0, 1, 0, 0, 0];
    /// let mut parser = csp::parser::Parser::new(reader);
    /// 
    /// assert_eq!(parser.read_u64(), Ok(4294968336));
    /// ```
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

impl<T: Read> CspParser for Parser<T> {
    fn pos(&self) -> usize {
        self.cursor
    }

    fn reset(&mut self) -> usize {
        let s = self.cursor;
        self.cursor = 0;
        return s;
    }
}

// ======================= Parser Async =======================

pub struct AsyncParser<T>
where
    T: AsyncRead,
{
    cursor: usize,
    buf: AsyncBufReader<T>,
}

impl<T: AsyncRead + std::marker::Unpin> AsyncParser<T> {
    /// create a new async parser
    ///
    /// # Arguments
    /// * `reader` - a type that implement AsyncRead
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader1: &[u8] = &[3, 49, 46, 48, 46, 51, 4];
    ///     let reader2 = smol::fs::File::open("Cargo.toml").await.expect("cargo.toml: no file or directory");
    ///     // # that works too
    ///     // let reader2 = soml::net::TcpStream::connect("127.0.0.0:8080").await
    ///     //    .expect("failed to open connection to 127.0.0.0:8080");
    ///  
    ///     let parser1 = csp::parser::AsyncParser::new(reader1);
    ///     let parser2 = csp::parser::AsyncParser::new(reader2);
    /// } )
    /// ```
    pub fn new(reader: T) -> AsyncParser<T> {
        AsyncParser {
            cursor: 0,
            buf: AsyncBufReader::new(reader),
        }
    }

    /// read exactly `size` bytes, unless the reader hit EOF, where it return the last bytes
    /// when reading 0 bytes, that means that the reader has no longer data
    /// -> on TcpStream, that means the connection has been closed
    ///
    /// # Arguments
    /// * `size` - number of bytes to read
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[1,2,3,4,5,6,7];
    ///     let mut parser = csp::parser::AsyncParser::new(reader);
    /// 
    ///     assert_eq!(parser.read(3).await, vec![1,2,3]);
    ///     assert_eq!(parser.read(2).await, vec![4,5]);
    ///     assert_eq!(parser.read(20).await, vec![6,7]);
    ///     assert_eq!(parser.read(2).await, vec![]);
    /// } )
    /// ```
    pub async fn read(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0; size];

        let mut slice = buffer.as_mut_slice();
        let mut s: usize = 0;

        while !slice.is_empty() {
            match self.buf.read(&mut slice).await {
                Ok(0) => break,
                Ok(n) => {
                    s += n;
                    self.cursor += n;
                    let tmp = slice;
                    slice = &mut tmp[n..];
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    // FIXME: does async read thow ErrorKind::Interrupted ?
                }
                Err(e) => {
                    // TODO: use a proper logger
                    eprint!("got {:?} when attempt to read buffer", e);
                    // FIXME: handle error, but what kind of error should we expect...?
                    s = 0;
                }
            };
        }

        buffer.resize(s, 0);
        return buffer;
    }

    /// read exactly one byte
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[1];
    ///     let mut parser = csp::parser::AsyncParser::new(reader);
    /// 
    ///     assert_eq!(parser.read_byte().await, Ok(1));
    ///     assert_eq!(parser.read_byte().await.unwrap_err().id, csp::parser::ParserErrorId::UkwnHeader);
    /// } )
    /// ```
    pub async fn read_byte(&mut self) -> ParseResult<u8> {
        let buf = self.read(1).await;
        if buf.len() == 0 {
            let mut error = ParserError::new(ParserErrorId::UkwnHeader, self.pos());
            error.set_desc("{1}", "EOF");
            return Err(error);
        }

        return Ok(buf[0]);
    }

    /// read a csp string
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[3, 49, 46, 48, 46, 51, 4];
    ///     let mut parser = csp::parser::AsyncParser::new(reader);
    /// 
    ///     assert_eq!(parser.read_string().await, Ok("1.0.3".to_string()));
    /// } )
    /// ```
    pub async fn read_string(&mut self) -> ParseResult<String> {
        let mut s = vec![];
        let mut start = false;

        loop {
            let buf = self.read(1).await;

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

    /// read 4 bytes and parse them as little endian u32
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[16, 4, 0, 0, 2];
    ///     let mut parser = csp::parser::AsyncParser::new(reader);
    /// 
    ///     assert_eq!(parser.read_u32().await, Ok(1040));
    /// } )
    /// ```
    pub async fn read_u32(&mut self) -> ParseResult<u32> {
        let mut buf = self.read(4).await;
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

    /// read 8 bytes and parse them as little endian u64
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[16, 4, 0, 0, 1, 0, 0, 0];
    ///     let mut parser = csp::parser::AsyncParser::new(reader);
    /// 
    ///     assert_eq!(parser.read_u64().await, Ok(4294968336));
    /// } )
    /// ```
    pub async fn read_u64(&mut self) -> ParseResult<u64> {
        let mut buf = self.read(8).await;
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

impl<T: AsyncRead> CspParser for AsyncParser<T> {
    fn pos(&self) -> usize {
        self.cursor
    }

    fn reset(&mut self) -> usize {
        let s = self.cursor;
        self.cursor = 0;
        return s;
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

        assert_eq!(5, parser.reset());

        assert_eq!(parser.read(20), vec![6, 7, 8, 9, 10]);
        assert_eq!(5, parser.pos());

        assert_eq!(parser.read(2), vec![]);
        assert_eq!(5, parser.pos());
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

    #[test]
    fn async_parser_read() {
        smol::block_on( async {
            let reader: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let mut parser = AsyncParser::new(reader);

            assert_eq!(parser.read(2).await, vec![1, 2]);
            assert_eq!(2, parser.pos());

            assert_eq!(parser.read(3).await, vec![3, 4, 5]);
            assert_eq!(5, parser.pos());

            assert_eq!(5, parser.reset());

            assert_eq!(parser.read(20).await, vec![6, 7, 8, 9, 10]);
            assert_eq!(5, parser.pos());

            assert_eq!(parser.read(2).await, vec![]);
            assert_eq!(5, parser.pos());
        })
    }

    #[test]
    fn asnyc_parser_parse_byte() {
        smol::block_on( async {
                let reader: &[u8] = &[32];

                let mut parser = AsyncParser::new(reader);
                assert_eq!(parser.read_byte().await, Ok(32));
                assert_eq!(
                    parser.read_byte().await.unwrap_err().id,
                    ParserErrorId::UkwnHeader
                );
        })
    }

    #[test]
    fn async_parser_parse_string() {
        smol::block_on( async {
            let reader1: &[u8] = &[3, 49, 46, 48, 46, 51, 4];
            let reader2: &[u8] = &[3, 195, 169, 4];
            let reader3: &[u8] = &[49, 46, 48, 46, 51, 4];
            let reader4: &[u8] = &[3, 49, 46, 48, 46, 51];
            let reader5: &[u8] = &[3, 0, 195, 4];

            let mut parser1 = AsyncParser::new(reader1);
            let mut parser2 = AsyncParser::new(reader2);
            let mut parser3 = AsyncParser::new(reader3);
            let mut parser4 = AsyncParser::new(reader4);
            let mut parser5 = AsyncParser::new(reader5);

            assert_eq!(parser1.read_string().await, Ok("1.0.3".to_string()));
            assert_eq!(parser2.read_string().await, Ok("é".to_string()));
            assert_eq!(
                parser3.read_string().await.unwrap_err().id,
                ParserErrorId::MissCtrl
            );
            assert_eq!(
                parser4.read_string().await.unwrap_err().id,
                ParserErrorId::MissCtrl
            );
            assert_eq!(parser5.read_string().await.unwrap_err().id, ParserErrorId::InvStr);
        })
    }

    #[test]
    fn async_parser_parse_u32() {
        smol::block_on( async {
            let reader1: &[u8] = &[11, 0, 45, 05];
            let reader2: &[u8] = &[22, 05, 0];

            let mut parser1 = AsyncParser::new(reader1);
            let mut parser2 = AsyncParser::new(reader2);

            assert_eq!(parser1.read_u32().await, Ok(86835211));
            assert_eq!(parser2.read_u32().await.unwrap_err().id, ParserErrorId::InvNum);
        })
    }

    #[test]
    fn async_parser_parse_u64() {
        smol::block_on( async {
            let reader1: &[u8] = &[11, 0, 45, 05, 0, 3, 0, 0];
            let reader2: &[u8] = &[22, 05, 0, 0, 5, 33];

            let mut parser1 = AsyncParser::new(reader1);
            let mut parser2 = AsyncParser::new(reader2);

            assert_eq!(parser1.read_u64().await, Ok(3298621718539));
            assert_eq!(parser2.read_u64().await.unwrap_err().id, ParserErrorId::InvNum);
        })
    }

}
