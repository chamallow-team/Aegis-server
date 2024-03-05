use std::{
    fmt,
    io::{self, BufRead, BufReader, Read},
};

use smol::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader as AsyncBufReader};

use crate::{v10, Version};

// ======================= ParseErrorId =======================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorId {
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
    /// a specific header is missing
    MissHeader,
    /// controls that are not expected here
    UnxptCtrl,
    /// invalid number, mostly caused by EOF before fully reading an n bytes number
    InvNum,
    /// invalid Length header and data length, they should be roughtly the same
    InvDataLen,
    /// invalid data compression
    InvDataComp,
    /// motly caused by an invalid utf-8 string
    InvStr,
    /// when the packet took to long to be gathered
    TimedOut,
    /// for all error that don't have an id to report
    Unknown,

    // not sent to the client, only used internally
    ConnectionClosed,
}

impl ParseErrorId {
    pub fn description(&self) -> &'static str {
        match self {
            ParseErrorId::DupHeader => "Duplicated header: {1}.",
            ParseErrorId::UkwnHeader => "Unknown header: {1}.",
            ParseErrorId::UkwnHeaderVal => "Unknown value for [{1}]: {2}.",
            ParseErrorId::UkwnCtrl => "Unknown control: {1}.",
            ParseErrorId::MissCtrl => "Missing control: {1}.",
            ParseErrorId::MissHeader => "Missing header: {1}",
            ParseErrorId::UnxptCtrl => "Unexpected control: {1}.",
            ParseErrorId::InvNum => "Invalid number: expected {1} bytes, found {2}.",
            ParseErrorId::InvDataLen => "Invalid length header, data length mismatch it.",
            ParseErrorId::InvDataComp => "Invalid data compression: {1}",
            ParseErrorId::InvStr => "Invalid utf-8 string.",
            ParseErrorId::TimedOut => {
                "Took to long to gather the rest of the packet, usually means that it's corrupted"
            }
            ParseErrorId::ConnectionClosed => "Connection closed",
            ParseErrorId::Unknown => "{1}",
        }
    }

    pub fn from_str<T: ToString>(id: T) -> Option<Self> {
        let id = id.to_string().to_uppercase();

        match id.as_str() {
            "DUP_HEADER" => Some(ParseErrorId::DupHeader),
            "UKWN_HEADER" => Some(ParseErrorId::UkwnHeader),
            "UKWN_HEADER_VAL" => Some(ParseErrorId::UkwnHeaderVal),
            "UKWN_CTRL" => Some(ParseErrorId::UkwnCtrl),
            "MISS_CTRL" => Some(ParseErrorId::MissCtrl),
            "MISS_HEADER" => Some(ParseErrorId::MissHeader),
            "UNXPT_CTRL" => Some(ParseErrorId::UnxptCtrl),
            "INV_NUM" => Some(ParseErrorId::InvNum),
            "INV_DATA_LEN" => Some(ParseErrorId::InvDataLen),
            "INV_DATA_COMP" => Some(ParseErrorId::InvDataComp),
            "INV_STR" => Some(ParseErrorId::InvStr),
            "TIMEDOUT" => Some(ParseErrorId::TimedOut),
            "CON_CLOSED" => Some(ParseErrorId::ConnectionClosed),
            "UNKNOWN" => Some(ParseErrorId::Unknown),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ParseErrorId::DupHeader => "DUP_HEADER",
            ParseErrorId::UkwnHeader => "UKWN_HEADER",
            ParseErrorId::UkwnHeaderVal => "UKWN_HEADER_VAL",
            ParseErrorId::UkwnCtrl => "UKWN_CTRL",
            ParseErrorId::MissCtrl => "MISS_CTRL",
            ParseErrorId::MissHeader => "MISS_HEADER",
            ParseErrorId::UnxptCtrl => "UNXPT_CTRL",
            ParseErrorId::InvNum => "INV_NUM",
            ParseErrorId::InvDataLen => "INV_DATA_LEN",
            ParseErrorId::InvDataComp => "INV_DATA_COMP",
            ParseErrorId::InvStr => "INV_STR",
            ParseErrorId::TimedOut => "TIMEDOUT",
            ParseErrorId::ConnectionClosed => "CON_CLOSED",
            ParseErrorId::Unknown => "UNKNOWN",
        }
    }
}

impl TryFrom<String> for ParseErrorId {
    type Error = ();

    fn try_from(value: String) -> Result<Self, <ParseErrorId as TryFrom<String>>::Error> {
        match Self::from_str(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl ToString for ParseErrorId {
    fn to_string(&self) -> String {
        Self::to_str(self).to_string()
    }
}

// ======================= ParseError =======================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// id of the error, mostly used by the software to handle specific error
    pub id: ParseErrorId,
    /// description, used for debugging, or reporting the error to the user
    pub description: String,
    /// same use as description, to report where the error occured
    pub pos: usize,
}

impl ParseError {
    pub fn new(id: ParseErrorId, pos: usize) -> Self {
        Self {
            description: id.description().to_string(),
            id,
            pos,
        }
    }

    pub fn set_desc<S: ToString>(&mut self, p: &str, s: S) {
        match self.description.find(p) {
            Some(r) => self
                .description
                .replace_range(r..r + p.len(), s.to_string().as_str()),
            None => {}
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: At position {} {}",
            self.id, self.pos, self.description
        )
    }
}

pub type ParseResult<R> = std::result::Result<R, ParseError>;

// ======================= Parser =======================

pub struct Parser<T>
where
    T: Read,
{
    version: Version,
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
    /// let parser1 = csp::parser::Parser::new(reader1, csp::Version::V10);
    /// let parser2 = csp::parser::Parser::new(reader2, csp::Version::V10);
    /// ```
    pub fn new(reader: T, version: Version) -> Parser<T> {
        Parser {
            version,
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
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.read(3).unwrap(), vec![1,2,3]);
    /// assert_eq!(parser.read(2).unwrap(), vec![4,5]);
    /// assert_eq!(parser.read(20).unwrap(), vec![6,7]);
    /// assert_eq!(parser.read(2).unwrap(), vec![]);
    /// ```
    pub fn read(&mut self, size: usize) -> ParseResult<Vec<u8>> {
        let mut buffer = vec![0; size];

        let mut slice = buffer.as_mut_slice();
        let mut s: usize = 0;

        while !slice.is_empty() {
            match self.buf.read(slice) {
                Ok(0) => break,
                Ok(n) => {
                    s += n;
                    self.cursor += n;
                    let tmp = slice;
                    slice = &mut tmp[n..];
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // normally implemented where timout of reader.set_read_timout() hit
                    // on a parser, it indicate that we should not wait for more data as it'll likelly not come
                    // (at least for the current packet)
                    return Err(ParseError::new(ParseErrorId::TimedOut, self.pos()));
                }
                Err(e) if e.kind() == io::ErrorKind::ConnectionReset => {
                    return Err(ParseError::new(ParseErrorId::ConnectionClosed, self.pos()))
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
        Ok(buffer)
    }

    /// peek the buffer for more data
    ///
    /// usefull when attempting to wait for a packet without consumming the buffer
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[1,2,3];
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.peek(), Some(1));
    /// assert_eq!(parser.read(3).unwrap(), vec![1,2,3]);
    /// assert_eq!(parser.peek(), None);
    /// ```
    pub fn peek(&mut self) -> Option<u8> {
        match self.buf.fill_buf() {
            Ok(t) if t.is_empty() => None,
            Ok(t) => Some(t[0]),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => None,
            Err(_) => None,
        }
    }

    /// read exactly one byte
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[1];
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.read_byte(), Ok(1));
    /// assert_eq!(parser.read_byte().unwrap_err().id, csp::parser::ParseErrorId::ConnectionClosed);
    /// ```
    pub fn read_byte(&mut self) -> ParseResult<u8> {
        let buf = self.read(1)?;
        if buf.is_empty() {
            return Err(ParseError::new(ParseErrorId::ConnectionClosed, self.pos()));
        }

        Ok(buf[0])
    }

    /// read a csp string
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[2, 49, 46, 48, 46, 51, 3];
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.read_string(), Ok("1.0.3".to_string()));
    /// ```
    pub fn read_string(&mut self) -> ParseResult<String> {
        let mut s = vec![];
        let mut start = false;

        let string_start = match self.version {
            Version::V10 => v10::spec::Control::StringStart,
        };
        let string_end = match self.version {
            Version::V10 => v10::spec::Control::StringEnd,
        };

        loop {
            let buf = self.read(1)?;

            if buf.is_empty() {
                let mut error = ParseError::new(ParseErrorId::MissCtrl, self.pos());
                if s.is_empty() {
                    error.set_desc("{1}", string_start.to_string());
                } else {
                    error.set_desc("{1}", string_end.to_string());
                }
                return Err(error);
            }

            if start {
                if buf[0] == string_end.into() {
                    break;
                }
                s.push(buf[0]);
            } else if buf[0] == string_start.into() {
                start = true;
            } else {
                let mut error = ParseError::new(ParseErrorId::MissCtrl, self.pos());
                error.set_desc("{1}", string_start.to_string());
                return Err(error);
            }
        }

        match String::from_utf8(s) {
            Ok(s) => Ok(s),
            Err(_) => {
                let error = ParseError::new(ParseErrorId::InvStr, self.pos());
                Err(error)
            }
        }
    }

    /// read 4 bytes and parse them as little endian u32
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[16, 4, 0, 0, 2];
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.read_u32(), Ok(1040));
    /// ```
    pub fn read_u32(&mut self) -> ParseResult<u32> {
        let mut buf = self.read(4)?;
        if buf.len() < 4 {
            let mut error = ParseError::new(ParseErrorId::InvNum, self.pos());
            error.set_desc("{1}", "4");
            error.set_desc("{2}", buf.len());
            return Err(error);
        }
        let mut bytes: [u8; 4] = [0; 4];
        bytes.clone_from_slice(&mut buf[..4]);

        Ok(u32::from_le_bytes(bytes))
    }

    /// read 8 bytes and parse them as little endian u64
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[16, 4, 0, 0, 1, 0, 0, 0];
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.read_u64(), Ok(4294968336));
    /// ```
    pub fn read_u64(&mut self) -> ParseResult<u64> {
        let mut buf = self.read(8)?;
        if buf.len() < 8 {
            let mut error = ParseError::new(ParseErrorId::InvNum, self.pos());
            error.set_desc("{1}", "8");
            error.set_desc("{2}", buf.len());
            return Err(error);
        }
        let mut bytes: [u8; 8] = [0; 8];
        bytes.clone_from_slice(&mut buf[..8]);

        Ok(u64::from_le_bytes(bytes))
    }

    /// tells the position of the parser's cursor
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[1,2,3,4,5,6,7];
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.pos(), 0);
    ///
    /// parser.read(3);
    /// assert_eq!(parser.pos(), 3);
    /// ```
    pub fn pos(&self) -> usize {
        self.cursor
    }

    /// reset the parser's cursor and return the old cursor
    /// used when a packet is parsed and we want to reuse the parser for parsing other packets
    ///
    /// # Example
    /// ```
    /// let reader: &[u8] = &[1,2,3,4,5,6,7];
    /// let mut parser = csp::parser::Parser::new(reader, csp::Version::V10);
    ///
    /// assert_eq!(parser.pos(), 0);
    ///
    /// parser.read(3);
    /// assert_eq!(parser.pos(), 3);
    ///
    /// assert_eq!(parser.reset(), 3);
    /// assert_eq!(parser.pos(), 0);
    /// ```
    pub fn reset(&mut self) -> usize {
        let s = self.cursor;
        self.cursor = 0;
        s
    }

    /// get the version that the parser use
    pub fn version(&self) -> Version {
        self.version
    }

    /// set the version that the parser will use
    pub fn set_version(&mut self, version: Version) {
        self.version = version
    }
}

// ======================= AsyncParser =======================

// TODO AsyncParser

pub struct AsyncParser<T>
where
    T: AsyncRead + Unpin,
{
    version: Version,
    cursor: usize,
    buf: AsyncBufReader<T>,
}

impl<T: AsyncRead + Unpin> AsyncParser<T> {
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
    ///     let parser1 = csp::parser::AsyncParser::new(reader1, csp::Version::V10);
    ///     let parser2 = csp::parser::AsyncParser::new(reader2, csp::Version::V10);
    /// } )
    /// ```
    pub fn new(reader: T, version: Version) -> AsyncParser<T> {
        AsyncParser {
            cursor: 0,
            version,
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
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.read(3).await.unwrap(), vec![1,2,3]);
    ///     assert_eq!(parser.read(2).await.unwrap(), vec![4,5]);
    ///     assert_eq!(parser.read(20).await.unwrap(), vec![6,7]);
    ///     assert_eq!(parser.read(2).await.unwrap(), vec![]);
    /// } )
    /// ```
    pub async fn read(&mut self, size: usize) -> ParseResult<Vec<u8>> {
        let mut buffer = vec![0; size];

        let mut slice = buffer.as_mut_slice();
        let mut s: usize = 0;

        while !slice.is_empty() {
            match self.buf.read(slice).await {
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
                Err(e) if e.kind() == io::ErrorKind::ConnectionReset => {
                    return Err(ParseError::new(ParseErrorId::ConnectionClosed, self.pos()))
                }
                Err(e) => {
                    // TODO: use a proper logger
                    eprint!("got {:?} when attempt to read buffer", e);
                    // FIXME: handle error, but what kind of error should we expect...?
                    // TODO: add timedout error
                    s = 0;
                }
            };
        }

        buffer.resize(s, 0);
        Ok(buffer)
    }

    /// peek the buffer for more data
    ///
    /// usefull when attempting to wait for a packet without consumming the buffer
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[1,2,3];
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.peek().await, Some(1));
    ///     assert_eq!(parser.read(3).await.unwrap(), vec![1,2,3]);
    ///     assert_eq!(parser.peek().await, None);
    /// })
    /// ```
    pub async fn peek(&mut self) -> Option<u8> {
        match self.buf.fill_buf().await {
            Ok(t) if t.is_empty() => None,
            Ok(t) => Some(t[0]),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => None,
            Err(_) => None,
        }
    }

    /// read exactly one byte
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[1];
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.read_byte().await, Ok(1));
    ///     assert_eq!(parser.read_byte().await.unwrap_err().id, csp::parser::ParseErrorId::ConnectionClosed);
    /// } )
    /// ```
    pub async fn read_byte(&mut self) -> ParseResult<u8> {
        let buf = self.read(1).await?;
        if buf.is_empty() {
            return Err(ParseError::new(ParseErrorId::ConnectionClosed, self.pos()));
        }

        Ok(buf[0])
    }

    /// read a csp string
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[2, 49, 46, 48, 46, 51, 3];
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.read_string().await, Ok("1.0.3".to_string()));
    /// } )
    /// ```
    pub async fn read_string(&mut self) -> ParseResult<String> {
        let mut s = vec![];
        let mut start = false;

        let string_start = match self.version {
            Version::V10 => v10::spec::Control::StringStart,
        };
        let string_end = match self.version {
            Version::V10 => v10::spec::Control::StringEnd,
        };

        loop {
            let buf = self.read(1).await?;

            if buf.is_empty() {
                let mut error = ParseError::new(ParseErrorId::MissCtrl, self.pos());
                if s.is_empty() {
                    error.set_desc("{1}", string_start.to_string());
                } else {
                    error.set_desc("{1}", string_end.to_string());
                }
                return Err(error);
            }

            if start {
                if buf[0] == string_end.into() {
                    break;
                }
                s.push(buf[0]);
            } else if buf[0] == string_start.into() {
                start = true;
            } else {
                let mut error = ParseError::new(ParseErrorId::MissCtrl, self.pos());
                error.set_desc("{1}", string_start.to_string());
                return Err(error);
            }
        }

        match String::from_utf8(s) {
            Ok(s) => Ok(s),
            Err(_) => {
                let error = ParseError::new(ParseErrorId::InvStr, self.pos());
                Err(error)
            }
        }
    }

    /// read 4 bytes and parse them as little endian u32
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[16, 4, 0, 0, 2];
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.read_u32().await, Ok(1040));
    /// } )
    /// ```
    pub async fn read_u32(&mut self) -> ParseResult<u32> {
        let mut buf = self.read(4).await?;
        if buf.len() < 4 {
            let mut error = ParseError::new(ParseErrorId::InvNum, self.pos());
            error.set_desc("{1}", "4");
            error.set_desc("{2}", buf.len());
            return Err(error);
        }
        let mut bytes: [u8; 4] = [0; 4];
        bytes.clone_from_slice(&mut buf[..4]);

        Ok(u32::from_le_bytes(bytes))
    }

    /// read 8 bytes and parse them as little endian u64
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[16, 4, 0, 0, 1, 0, 0, 0];
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.read_u64().await, Ok(4294968336));
    /// } )
    /// ```
    pub async fn read_u64(&mut self) -> ParseResult<u64> {
        let mut buf = self.read(8).await?;
        if buf.len() < 8 {
            let mut error = ParseError::new(ParseErrorId::InvNum, self.pos());
            error.set_desc("{1}", "8");
            error.set_desc("{2}", buf.len());
            return Err(error);
        }
        let mut bytes: [u8; 8] = [0; 8];
        bytes.clone_from_slice(&mut buf[..8]);

        Ok(u64::from_le_bytes(bytes))
    }

    /// tells the position of the parser's cursor
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[1,2,3,4,5,6,7];
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.pos(), 0);
    ///
    ///     parser.read(3).await;
    ///     assert_eq!(parser.pos(), 3);
    /// } )
    /// ```
    pub fn pos(&self) -> usize {
        self.cursor
    }

    /// reset the parser's cursor and return the old cursor
    /// used when a packet is parsed and we want to reuse the parser for parsing other packets
    ///
    /// # Example
    /// ```
    /// smol::block_on( async {
    ///     let reader: &[u8] = &[1,2,3,4,5,6,7];
    ///     let mut parser = csp::parser::AsyncParser::new(reader, csp::Version::V10);
    ///
    ///     assert_eq!(parser.pos(), 0);
    ///
    ///     parser.read(3).await;
    ///     assert_eq!(parser.pos(), 3);
    ///
    ///     assert_eq!(parser.reset(), 3);
    ///     assert_eq!(parser.pos(), 0);
    /// } )
    /// ```
    pub fn reset(&mut self) -> usize {
        let s = self.cursor;
        self.cursor = 0;
        s
    }

    /// get the version that the parser use
    pub fn version(&self) -> Version {
        self.version
    }

    /// set the version that the parser will use
    pub fn set_version(&mut self, version: Version) {
        self.version = version
    }
}

// ======================= Tests =======================

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::Version;

    #[test]
    fn read() {
        let reader: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut parser = Parser::new(reader, Version::V10);

        assert_eq!(parser.read(2).unwrap(), vec![1, 2]);
        assert_eq!(2, parser.pos());

        assert_eq!(parser.read(3).unwrap(), vec![3, 4, 5]);
        assert_eq!(5, parser.pos());

        assert_eq!(5, parser.reset());

        assert_eq!(parser.read(20).unwrap(), vec![6, 7, 8, 9, 10]);
        assert_eq!(5, parser.pos());

        assert_eq!(parser.read(2).unwrap(), vec![]);
        assert_eq!(5, parser.pos());
    }

    #[test]
    fn parse_byte() {
        let reader: &[u8] = &[32];

        let mut parser = Parser::new(reader, Version::V10);
        assert_eq!(parser.read_byte(), Ok(32));
        assert_eq!(
            parser.read_byte().unwrap_err().id,
            ParseErrorId::ConnectionClosed
        );
    }

    #[test]
    fn parse_string() {
        let reader1: &[u8] = &[2, 49, 46, 48, 46, 51, 3];
        let reader2: &[u8] = &[2, 195, 169, 3];
        let reader3: &[u8] = &[49, 46, 48, 46, 51, 3];
        let reader4: &[u8] = &[2, 49, 46, 48, 46, 51];
        let reader5: &[u8] = &[2, 0, 195, 3];

        let mut parser1 = Parser::new(reader1, Version::V10);
        let mut parser2 = Parser::new(reader2, Version::V10);
        let mut parser3 = Parser::new(reader3, Version::V10);
        let mut parser4 = Parser::new(reader4, Version::V10);
        let mut parser5 = Parser::new(reader5, Version::V10);

        assert_eq!(parser1.read_string(), Ok("1.0.3".to_string()));
        assert_eq!(parser2.read_string(), Ok("é".to_string()));
        assert_eq!(
            parser3.read_string().unwrap_err().id,
            ParseErrorId::MissCtrl
        );
        assert_eq!(
            parser4.read_string().unwrap_err().id,
            ParseErrorId::MissCtrl
        );
        assert_eq!(parser5.read_string().unwrap_err().id, ParseErrorId::InvStr);
    }

    #[test]
    fn parse_u32() {
        let reader1: &[u8] = &[11, 0, 45, 05];
        let reader2: &[u8] = &[22, 05, 0];

        let mut parser1 = Parser::new(reader1, Version::V10);
        let mut parser2 = Parser::new(reader2, Version::V10);

        assert_eq!(parser1.read_u32(), Ok(86835211));
        assert_eq!(parser2.read_u32().unwrap_err().id, ParseErrorId::InvNum);
    }

    #[test]
    fn parse_u64() {
        let reader1: &[u8] = &[11, 0, 45, 05, 0, 3, 0, 0];
        let reader2: &[u8] = &[22, 05, 0, 0, 5, 33];

        let mut parser1 = Parser::new(reader1, Version::V10);
        let mut parser2 = Parser::new(reader2, Version::V10);

        assert_eq!(parser1.read_u64(), Ok(3298621718539));
        assert_eq!(parser2.read_u64().unwrap_err().id, ParseErrorId::InvNum);
    }

    #[test]
    fn async_read() {
        smol::block_on(async {
            let reader: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let mut parser = AsyncParser::new(reader, Version::V10);

            assert_eq!(parser.read(2).await.unwrap(), vec![1, 2]);
            assert_eq!(2, parser.pos());

            assert_eq!(parser.read(3).await.unwrap(), vec![3, 4, 5]);
            assert_eq!(5, parser.pos());

            assert_eq!(5, parser.reset());

            assert_eq!(parser.read(20).await.unwrap(), vec![6, 7, 8, 9, 10]);
            assert_eq!(5, parser.pos());

            assert_eq!(parser.read(2).await.unwrap(), vec![]);
            assert_eq!(5, parser.pos());
        })
    }

    #[test]
    fn asnyc_parse_byte() {
        smol::block_on(async {
            let reader: &[u8] = &[32];

            let mut parser = AsyncParser::new(reader, Version::V10);
            assert_eq!(parser.read_byte().await, Ok(32));
            assert_eq!(
                parser.read_byte().await.unwrap_err().id,
                ParseErrorId::ConnectionClosed
            );
        })
    }

    #[test]
    fn async_parse_string() {
        smol::block_on(async {
            let reader1: &[u8] = &[2, 49, 46, 48, 46, 51, 3];
            let reader2: &[u8] = &[2, 195, 169, 3];
            let reader3: &[u8] = &[49, 46, 48, 46, 51, 3];
            let reader4: &[u8] = &[2, 49, 46, 48, 46, 51];
            let reader5: &[u8] = &[2, 0, 195, 3];

            let mut parser1 = AsyncParser::new(reader1, Version::V10);
            let mut parser2 = AsyncParser::new(reader2, Version::V10);
            let mut parser3 = AsyncParser::new(reader3, Version::V10);
            let mut parser4 = AsyncParser::new(reader4, Version::V10);
            let mut parser5 = AsyncParser::new(reader5, Version::V10);

            assert_eq!(parser1.read_string().await, Ok("1.0.3".to_string()));
            assert_eq!(parser2.read_string().await, Ok("é".to_string()));
            assert_eq!(
                parser3.read_string().await.unwrap_err().id,
                ParseErrorId::MissCtrl
            );
            assert_eq!(
                parser4.read_string().await.unwrap_err().id,
                ParseErrorId::MissCtrl
            );
            assert_eq!(
                parser5.read_string().await.unwrap_err().id,
                ParseErrorId::InvStr
            );
        })
    }

    #[test]
    fn async_parse_u32() {
        smol::block_on(async {
            let reader1: &[u8] = &[11, 0, 45, 05];
            let reader2: &[u8] = &[22, 05, 0];

            let mut parser1 = AsyncParser::new(reader1, Version::V10);
            let mut parser2 = AsyncParser::new(reader2, Version::V10);

            assert_eq!(parser1.read_u32().await, Ok(86835211));
            assert_eq!(
                parser2.read_u32().await.unwrap_err().id,
                ParseErrorId::InvNum
            );
        })
    }

    #[test]
    fn async_parse_u64() {
        smol::block_on(async {
            let reader1: &[u8] = &[11, 0, 45, 05, 0, 3, 0, 0];
            let reader2: &[u8] = &[22, 05, 0, 0, 5, 33];

            let mut parser1 = AsyncParser::new(reader1, Version::V10);
            let mut parser2 = AsyncParser::new(reader2, Version::V10);

            assert_eq!(parser1.read_u64().await, Ok(3298621718539));
            assert_eq!(
                parser2.read_u64().await.unwrap_err().id,
                ParseErrorId::InvNum
            );
        })
    }
}
