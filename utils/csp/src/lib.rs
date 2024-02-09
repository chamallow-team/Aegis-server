use std::{fmt, io::{self, Read}, result, vec};

use parser::{Parser, ParserError};

mod parser;

// ======================= Csp constants =======================

/// see /doc/protocols/CSP.md#control-characters
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Control {
    /// Ends of an header part.
    /// 
    /// If no Length header have been found, then the Packet ends here, otherwise, a DataStart control
    /// shall be found next, and using the Length header, the data is parsed
    HeaderEnd,
    /// Starts of a data part
    /// 
    /// Found next to the HeaderEnd, it is read only if the Length header is set
    DataStart,
    /// Starts of a string
    /// 
    /// This special control is used to define the start of an unknown length value, named string, and is associated
    /// to a fiew Headers.
    StringStart,
    /// Ends of a string
    /// 
    /// Same use as StringStart, but to specify the end of this string
    StringEnd,
}

impl Control {
    pub fn from_u8(byte: u8) -> Option<Control>{
        match byte {
            1 => Some(Self::HeaderEnd),
            2 => Some(Self::DataStart),
            3 => Some(Self::StringStart),
            4 => Some(Self::StringEnd),
            _ => None,
        }
    }
    pub fn to_u8(ctrl: Control) -> u8{
        match ctrl {
            Self::HeaderEnd => 1,
            Self::DataStart => 2,
            Self::StringStart => 3,
            Self::StringEnd => 4,
        }
    }
}

impl TryFrom<u8> for Control {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Control as TryFrom<u8>>::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(())
        }
    }
}

impl Into<u8> for Control {
    fn into(self) -> u8 {
        Self::to_u8(self)
    }
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Control::HeaderEnd => write!(f, "header_end"),
            Control::DataStart => write!(f, "data_start"),
            Control::StringStart => write!(f, "string_start"),
            Control::StringEnd => write!(f, "string_end"),
        }
    }
}

/// see /doc/protocols/headers/method.md
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Method {
    /// Connect to the Server
    /// 
    /// Allways send by the cCient
    Connect,
    /// Request or try an authentification
    /// 
    /// this allow the Client to get his identity
    /// this steps can be avoided with an identity header on the connect packet
    Auth,
    /// disconnect from the Client/Server
    /// 
    /// used for various reasons, such as resetting the connection after getting a corrupted packet (which invalidate the whole tcp buffer)
    /// or just closing the Client/Server
    Disconnect,
    /// Perform an admin action
    /// 
    /// It's separated from Action to easily filters real admins
    Admin,
    /// Send from the Server, used to provide new information about the game
    /// 
    /// When an unit moves, a message is received, ect...
    Update,
    /// the Client perform an action
    /// 
    /// Building something, moving an unit, ect...
    Action,
    /// send a Error packet
    /// 
    /// Usually send as a response of a packet
    Error,
    /// send for getting the initial state of something or send files
    /// 
    /// usually used to send large data such as the game's state (maps, units, messages...)
    /// it can be used to send chunks of information using the id header
    State,
}

impl Method {
    pub fn from_u8(byte: u8) -> Option<Method> {
        match byte {
            32 => Some(Method::Connect),
            33 => Some(Method::Auth),
            34 => Some(Method::Disconnect),
            35 => Some(Method::Admin),
            36 => Some(Method::Update),
            37 => Some(Method::Action),
            38 => Some(Method::Error),
            39 => Some(Method::State),
            _=> None
        }
    }

    pub fn to_u8(method: &Method) -> u8 {
        match method {
            Method::Connect => 32,
            Method::Auth => 33,
            Method::Disconnect => 34,
            Method::Admin => 35,
            Method::Update => 36,
            Method::Action => 37,
            Method::Error => 38,
            Method::State => 39,
        }
    }
}

impl TryFrom<u8> for Method {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Method as TryFrom<u8>>::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(())
        }
    }
}

impl Into<u8> for Method {
    fn into(self) -> u8 {
        Self::to_u8(&self)
    }
}

/// see /doc/protocols/headers/csp.md
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Version {
    /// current version
    V10,
}

impl Version {
    pub fn from_u8(byte: u8) -> Option<Version> {
        match byte {
            32 => Some(Version::V10),
            _ => None
        }
    }

    pub fn to_u8(version: &Version) -> u8 {
        match version {
            Version::V10 => 32,
        }
    }
}

impl TryFrom<u8> for Version {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Version as TryFrom<u8>>::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(())
        }
    }
}

impl Into<u8> for Version {
    fn into(self) -> u8 {
        Self::to_u8(&self)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::V10 => write!(f, "1.0"),
        }
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self, f)
    }
}


/// see /doc/protocols/CSP.md#header-keys
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header {
    /// Method used by the packet
    Method(Method),
    /// Server where the packet is heading or from which server it comes
    Server(u32),
    /// the length of data if associated.
    /// 
    /// Should be exactly the same length, if not, the data can be wrongly parsed, 
    /// or the packet can be concidered as corrupted
    Length(u64),
    /// Crediential of the Client
    /// 
    /// Used to "identify" a client, or to quickly authenticate
    Identity(String),
    /// Version of the client
    /// 
    /// Server can accept or reject a Client based of this header
    Version(String),
    /// Used by the client to get an update of the Server's configuration...
    /// 
    /// Mostly used in a first connection
    Update(bool),
    /// Id of the packet
    /// 
    /// This is used when the packet awaits a response (eg Action Method), the other end will respond with a packet using the same id
    /// If a response is not provided, the packet is resend after a period of time
    Id(u64),
    /// used primally by the server with the Disconnect method
    /// 
    /// this allow the TCP connection to be kept open and at the same time totally reset the connection
    /// This includes the auth, the awaitings packets, buffers, the states...
    Reconnect(bool),
    /// Csp version
    /// 
    /// its used for compatibility, in case of addition of header, ect.. try to avoid error based of csp version
    Csp(Version),
}

impl Header {
    pub fn matches(a: &Header, b: &Header) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    pub fn from_str<T: ToString>(s: T) -> Option<Header> {
        match s.to_string().to_lowercase().as_str() {
            "method" => Some(Header::Method(Method::Connect)),
            "server" => Some(Header::Server(0)),
            "length" => Some(Header::Length(0)),
            "identity" => Some(Header::Identity("".to_string())),
            "version" => Some(Header::Version("".to_string())),
            "update" => Some(Header::Update(false)),
            "id" => Some(Header::Id(0)),
            "reconnect" => Some(Header::Reconnect(false)),
            "csp" => Some(Header::Csp(Version::V10)),
            _ => None
        }
    }
    pub fn to_str(header: &Header) -> &'static str {
        match header {
            Header::Method(_) => "method",
            Header::Server(_) => "server",
            Header::Length(_) => "length",
            Header::Identity(_) => "identity",
            Header::Version(_) => "version",
            Header::Update(_) => "update",
            Header::Id(_) => "id",
            Header::Reconnect(_) => "reconnect",
            Header::Csp(_) => "csp",
        }
    }

    pub fn from_u8(byte: u8) -> Option<Header> {
        match byte {
            32 => Some(Header::Method(Method::Connect)),
            33 => Some(Header::Server(0)),
            34 => Some(Header::Length(0)),
            35 => Some(Header::Identity("".to_string())),
            36 => Some(Header::Version("".to_string())),
            37 => Some(Header::Update(false)),
            38 => Some(Header::Id(0)),
            39 => Some(Header::Reconnect(false)),
            40 => Some(Header::Csp(Version::V10)),
            _ => None
        }

    }
    pub fn to_u8(header: &Header) -> u8 {
        match header {
            Header::Method(_) => 32,
            Header::Server(_) => 33,
            Header::Length(_) => 34,
            Header::Identity(_) => 35,
            Header::Version(_) => 36,
            Header::Update(_) => 37,
            Header::Id(_) => 38,
            Header::Reconnect(_) => 39,
            Header::Csp(_) => 40,
        }
    }

    pub fn to_buffer(header: &Header) -> Vec<u8> {
        match header {
            // String
            Header::Identity(ref v) | Header::Version(ref v) => {
                let mut vector = vec![header.clone().into()];
                vector.push(Control::StringStart.into());
                vector.append(&mut v.clone().into_bytes());
                vector.push(Control::StringEnd.into());
                return vector
            },
            // u32
            Header::Server(v) => {
                let mut vector = vec![header.clone().into()];
                vector.append(&mut v.to_le_bytes().to_vec());
                return vector
            },
            // u64
            Header::Length(v) | Header::Id(v) => {
                let mut vector = vec![header.clone().into()];
                vector.append(&mut v.to_le_bytes().to_vec());
                return vector
            },
            // bool
            Header::Update(v) | Header::Reconnect(v) if *v => vec![header.clone().into()],
            // u8
            Header::Csp(v) => vec![header.clone().into(), v.clone().into()],
            Header::Method(v) => vec![header.clone().into(), v.clone().into()],
            _ => vec![]
        }
    }
    pub fn from_buffer<T: Read>(bytes: Parser<T>) -> result::Result<Header, ParserError> {
        todo!()
    }
    
    pub fn to_vec(&self) -> Vec<u8> {
        return Self::to_buffer(self)
    }
    pub fn from_vec<T: Read>(&self, bytes:&mut Parser<T>) -> result::Result<Header, ParserError> {
        todo!()
    }
}

impl Into<u8> for Header {
    fn into(self) -> u8 {
        Self::to_u8(&self)
    }
}

impl Into<Vec<u8>> for Header {

    fn into(self) -> Vec<u8>{
        Self::to_buffer(&self)
    }
}

impl TryInto<String> for Header {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Header::Version(v) => Ok(v),
            Header::Identity(v) => Ok(v),
            _ => Err(())
        }
    }
}

impl TryInto<Method> for Header {
    type Error = ();

    fn try_into(self) -> Result<Method, Self::Error> {
        match self {
            Header::Method(v) => Ok(v),
            _ => Err(())
        }
    }
}

impl TryInto<Version> for Header {
    type Error = ();

    fn try_into(self) -> Result<Version, Self::Error> {
        match self {
            Header::Csp(v) => Ok(v),
            _ => Err(())
        }
    }
}

impl TryInto<u32> for Header {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Header::Server(v) => Ok(v),
            _ => Err(())
        }
    }
}

impl TryInto<u64> for Header {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Header::Length(v) => Ok(v),
            Header::Id(v) => Ok(v),
            _ => Err(())
        }
    }
}

impl TryInto<bool> for Header {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Header::Update(v) => Ok(v),
            Header::Reconnect(v) => Ok(v),
            _ => Err(())
        }
    }
}

pub type Headers = Vec<Header>;


// ======================= Packet =======================

#[derive(Debug, Default)]
pub struct Packet {
    headers: Headers,
    data: Vec<u8>,
    buffer: Vec<u8>,
    // states

    /// if the headers/data has been modified, it invalidate the internal buffer
    /// when invalid, the buffer needs to be rebuild, with the prepare function
    buffer_valid: bool,
}

impl Packet {
    /// initialize a Packet
    /// 
    /// # Arguments
    /// 
    /// * `method` - the method used by the packet
    /// * `headers` - a vector of headers tuple
    /// 
    /// # Example
    /// ```
    ///  let headers = vec![
    ///     csp::Header::Version("1.0.3".to_string()),
    ///     csp::Header::Server(1000),
    /// ];
    /// let packet = csp::Packet::new(Some(headers));
    /// ```
    pub fn new(headers: Option<Headers>) -> Self {
        let headers: Headers = match headers {
            Some(v) => v,
            None => vec![],
        };

        Packet{
            headers,
            data: vec![],
            buffer: vec![],

            buffer_valid: false
        }
    }

    /// return a copy of the associated value given a header
    /// 
    /// # Arguments
    /// * `header` - a header string key, not case sensitive
    /// 
    /// # Example
    /// ```
    /// let headers = vec![csp::Header::Version("1.0.3".to_string())];
    /// let mut packet = csp::Packet::new(Some(headers));
    /// 
    /// assert_eq!(packet.get_header("version").expect("no header Version found"), csp::Header::Version("1.0.3".to_string()));
    /// assert_eq!(packet.get_header("veRsiOn").expect("no header Version found"), csp::Header::Version("1.0.3".to_string()));
    /// assert_eq!(packet.get_header("VERSION").expect("no header Version found"), csp::Header::Version("1.0.3".to_string()));
    /// ```
    pub fn get_header<T: ToString>(&self, header: T) -> Option<Header> {
        let header = Header::from_str(header.to_string());
        if matches!(header, None) {return None}
        
        let header = header.unwrap(); // safe cause we already checked before
        
        let mut value = None;
        for k in self.headers.iter() {
            if Header::matches(k, &header) {
                value = Some(k.clone())
            }
        }

        return value
    }

    /// set or modify the header to provided value
    /// invalidate the internal buffer
    /// 
    /// # Arguments
    /// * `header` - a header key
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Version("1.0.3".to_string()));
    /// let version = packet.get_header("version").expect("no header Version found");
    /// 
    /// assert_eq!(version, csp::Header::Version("1.0.3".to_string()));
    /// ```
    pub fn set_header(&mut self, header: Header) {
        self.buffer_valid = false;

        let mut pos: usize = usize::MAX;

        for (k, v) in self.headers.iter().enumerate() {
            if Header::matches(v, &header) {
                pos = k;
            }
        }

        if pos < self.headers.len(){
            self.headers[pos as usize] = header
        } else {
            self.headers.push(header)
        }
    }

    /// remove a specific header from the packet and returns it
    /// invalidate the internal buffer
    /// 
    /// # Arguments
    /// * `header` - a header string key, not case sensitive
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Server(10));
    /// 
    /// assert_eq!(packet.pop_header("server"), Some(csp::Header::Server(10)));
    /// assert_eq!(packet.pop_header("server"), None);
    /// ```
    pub fn pop_header<T: ToString>(&mut self, header: T) -> Option<Header> {
        let header = Header::from_str(header.to_string());
        if matches!(header, None) {return None}
        
        let header = header.unwrap(); // safe cause we already checked before
        let mut pos: usize = usize::MAX;
    
        for (k,v) in self.headers.iter().enumerate() {
            if Header::matches(v, &header) {
                pos = k;
            }
        }
        if pos < self.headers.len() {
            self.buffer_valid = false;
            return Some(self.headers.remove(pos))
        } else {
            return None
        }

    }

    /// return a clone of the packet's headers
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Version("1.0.3".to_string()));
    /// let version = packet.get_header("version").expect("no header Version found");
    /// 
    /// assert_eq!(version, csp::Header::Version("1.0.3".to_string()));
    /// ```
    pub fn get_headers(&self) -> Headers {
        self.headers.clone()
    }

    /// sets or modify headers with the provided vector
    /// invalidate the internal buffer
    /// 
    /// # Arguments
    /// * `header` - a header vector
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_headers(vec![
    ///     csp::Header::Version("1.0.3".to_string()),
    ///     csp::Header::Method(csp::Method::Connect)
    /// ]);
    /// 
    /// let version = packet.get_header("version").expect("no header Version found");
    ///
    /// let method = packet.get_header("method").expect("no header Method found");
    /// 
    /// assert_eq!(version, csp::Header::Version("1.0.3".to_string()));
    /// assert_eq!(method, csp::Header::Method(csp::Method::Connect));
    /// ```
    pub fn set_headers(&mut self, headers: Headers) {
        self.buffer_valid = false;
        for v in headers.iter() {
            self.set_header(v.clone());
        }
    }

    /// return a copy of the packet's data
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_data("Hello World");
    /// 
    /// assert_eq!(packet.get_data(), "Hello World".as_bytes().to_vec())
    /// ```
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// override the packet's data with provided data
    /// invalidate the internal buffer
    /// 
    /// # Arguments
    /// * `data` - any type that implement Into<u8>
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_data("Hello World");
    /// 
    /// assert_eq!(packet.get_data(), "Hello World".as_bytes().to_vec())
    /// ```
    pub fn set_data<T: Into<Vec<u8>>>(&mut self, data: T) {
        self.buffer_valid = false;
        self.data = data.into();

        self.set_header(Header::Length(self.data.len() as u64));
    }

    /// adds data to the end of the internal data
    /// invalidate the internal buffer
    /// 
    /// # Arguments
    /// * `data` - any type that implement Into<u8>
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_data("Hello");
    /// packet.append_data(" World");
    /// 
    /// assert_eq!(packet.get_data(), "Hello World".as_bytes().to_vec())
    /// ```
    pub fn append_data<T: Into<Vec<u8>>>(&mut self, data: T) {
        self.buffer_valid = false;
        self.data.append(&mut data.into());

        self.set_header(Header::Length(self.data.len() as u64));
    }

    /// adds data to the start of the internal data
    /// invalidate the internal buffer
    /// 
    /// # Arguments
    /// * `data` - any type that implement Into<u8>
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_data("World");
    /// packet.prepend_data("Hello ");
    /// 
    /// assert_eq!(packet.get_data(), "Hello World".as_bytes().to_vec())
    /// ```
    pub fn prepend_data<T: Into<Vec<u8>>>(&mut self, data: T) {
        self.buffer_valid = false;
        let mut tmp: Vec<u8> = data.into();
        tmp.append(&mut self.data);
        self.data = tmp;

        self.set_header(Header::Length(self.data.len() as u64));
    }

    /// build and return the internal packet buffer
    /// 
    /// if the buffer is valid (no modifications of the packet) it simply returns the buffer
    /// or build it
    /// 
    /// #Examples
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Method(csp::Method::Connect));
    ///
    /// assert_eq!(packet.prepare(), vec![32, 32, 01]);
    /// 
    /// // the buffer has not be invalided, so it've not been rebuilt
    /// assert_eq!(packet.prepare(), vec![32, 32, 01]);
    /// 
    /// packet.set_data(":)");
    /// assert_eq!(packet.prepare(), vec![32, 32, 34, 02, 0, 0, 0, 0, 0, 0, 0, 01, 02, 58, 41]); 
    /// ```
    pub fn prepare(&mut self) -> Vec<u8> {
        if self.buffer_valid {
            return self.buffer.clone()
        }

        self.buffer.clear();

        for header in self.headers.iter() {
            self.buffer.append(&mut header.to_vec())
        }
        self.buffer.push(Control::HeaderEnd.into());

        if !self.data.is_empty() {
            self.buffer.push(Control::DataStart.into());
            self.buffer.append(&mut self.data.clone());
        }

        self.buffer.clone()
    }

    /// return the length of the data, using the Length header
    /// 
    /// #Examples
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_data(":)");
    /// assert_eq!(2, packet.len());
    /// 
    /// packet.prepend_data("Hi ");
    /// assert_eq!(5, packet.len());
    /// ```
    pub fn len(&self) -> u64 {
        return self.get_header("length")
            .unwrap_or(Header::Length(0))
            .try_into().expect("failed to convert length to u64.. wtf?")
    }

    /// checks if the packet is brand new, and not data/header/buffer have been set
    /// 
    /// # Example
    /// ```
    /// let mut packet1 = csp::Packet::default();
    /// let mut packet2 = csp::Packet::default();
    /// let mut packet3 = csp::Packet::default();
    /// 
    /// packet2.set_data("hello world");
    /// packet3.set_header(csp::Header::Method(csp::Method::Connect));
    /// 
    /// assert!(packet1.is_empty());
    /// assert!(!packet2.is_empty());
    /// assert!(!packet3.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.headers.is_empty() && self.data.is_empty() && self.buffer.is_empty()
    }

    /// Totally clear the packet from any data, as if it's just been initialized
    /// invalidate the internal buffer
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Method(csp::Method::Connect));
    /// packet.set_data("hello world");
    /// 
    /// assert!(!packet.is_empty());
    /// packet.clear();
    /// assert!(packet.is_empty());
    /// 
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.buffer.clear();
        self.headers.clear();

        self.buffer_valid = false;
    }

    /// Clear the headers
    /// invalidate the internal buffer
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Method(csp::Method::Connect));
    /// packet.set_data("hello world");
    /// 
    /// packet.clear_headers();
    /// assert_eq!(packet.get_header("method"), None);
    /// assert_eq!(11, packet.len());
    /// ```
    pub fn clear_headers(&mut self) {
        self.headers.clear();
        if !self.data.is_empty() {
            self.set_header(Header::Length(self.data.len() as u64))
        }
        self.buffer_valid = false;
    }

    /// Clear the data
    /// invalidate the internal buffer
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Method(csp::Method::Connect));
    /// packet.set_data("hello world");
    /// 
    /// packet.clear_data();
    /// assert_eq!(packet.get_header("method"), Some(csp::Header::Method(csp::Method::Connect)));
    /// assert_eq!(0, packet.len());
    /// ```
    pub fn clear_data(&mut self) {
        self.data.clear();
        self.pop_header("length");
        self.buffer_valid = false;
    }

    /// Clear the buffer
    /// invalidate the internal buffer (obviously)
    /// 
    /// # Example
    /// ```
    /// let mut packet = csp::Packet::default();
    /// packet.set_header(csp::Header::Method(csp::Method::Connect));
    /// packet.set_data("hello world");
    /// 
    /// packet.prepare();
    /// packet.clear_buffer();
    /// // internal buffer is totally rebuilt
    /// packet.prepare();
    /// ```
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
        self.buffer_valid = false;
    }

    pub fn parse<T: io::Read>(&mut self, reader: T) -> result::Result<usize, ParserError>{
        todo!()
    }

}


// ======================= Tests =======================

#[cfg(test)]
mod tests {
    use super::*;
    use super::parser::*;

    #[test]
    fn header_serialization() {
        let hu64: Vec<u8> = Header::Length(1006).to_vec();
        let vu64: Vec<u8> = vec![34, 238, 3, 0, 0, 0, 0, 0, 0];

        let hu32: Vec<u8> = Header::Server(78453).to_vec();
        let vu32: Vec<u8> = vec![33, 117, 50, 1, 0,];

        let hstr: Vec<u8> = Header::Version("1.0.3".to_string()).to_vec();
        let vstr: Vec<u8> = vec![36, 3, 49, 46, 48, 46, 51, 4];

        let hmet: Vec<u8> = Header::Method(Method::Error).to_vec();
        let vmet: Vec<u8> = vec![32, 38];

        let hver: Vec<u8> = Header::Csp(Version::V10).to_vec();
        let vver: Vec<u8> = vec![40, 32];


        assert_eq!(hu64, vu64);
        assert_eq!(hu32, vu32);
        assert_eq!(hstr, vstr);
        assert_eq!(hmet, vmet);
        assert_eq!(hver, vver);
    }

    #[test]
    fn header_matches_eq() {
        let h1 = Header::Identity("blahblah".to_string());
        let h2 = Header::Identity("blahblah".to_string());
        let h3 = Header::Identity("blepblep".to_string());
        let h4 = Header::Method(Method::Action);

        assert!(Header::matches(&h1, &h2));
        assert!(Header::matches(&h1, &h3));
        assert!(!Header::matches(&h1, &h4));

        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert_ne!(h1, h4);
    }

    #[test]
    fn packet_get_set_header() {
        let mut packet = Packet::default();
        packet.set_header(Header::Version("1.0.3".to_string()));

        let version: String = packet.get_header("version")
            .expect("No Version header")
            .try_into().expect("not a string");

        assert_eq!("1.0.3".to_string(), version);
    }

    #[test]
    fn packet_get_set_headers() {
        let mut packet = Packet::default();
        // check if it override the headers
        packet.set_header(Header::Version("0.0.2".to_string()));
        
        packet.set_headers(vec![
            Header::Version("1.0.3".to_string()),
            Header::Method(Method::Connect),
        ]);

        let headers = packet.get_headers();
        let mut it = headers.iter();
        assert_eq!(it.next().expect("missing Version header"), &Header::Version("1.0.3".to_string()));
        assert_eq!(it.next().expect("missing Method header"), &Header::Method(Method::Connect));
        assert!(matches!(it.next(), None));
    }

    #[test]
    fn packet_data() {
        let mut packet = Packet::default();

        packet.set_data("hello world");
        assert_eq!(packet.get_data(), "hello world".as_bytes().to_vec());
        assert_eq!(packet.len(), 11);

        packet.set_data(" error ");
        assert_eq!(packet.len(), 7);

        packet.append_data("occured");
        assert_eq!(packet.len(), 14);

        packet.prepend_data("an");
        assert_eq!(packet.len(), 16);

        assert_eq!(packet.get_data(), "an error occured".as_bytes().to_vec());


    }

    #[test]
    fn packet_prepare() {
        let mut packet = Packet::default();

        packet.set_headers(vec![
            Header::Method(Method::Connect),
            Header::Update(true),
            Header::Server(45),
            Header::Csp(Version::V10),
            Header::Version("1.0.3".to_string()),
        ]);
        packet.set_data("Hello World");

        let buffer = packet.prepare();
        let alt_buffer = vec![
            32, 32,                         // method connect
            37,                             // update true
            33, 45, 0, 0, 0,                // server 45
            40, 32,                         // csp 1.0
            36, 03, 49, 46, 48, 46, 51, 04, // version 1.0.3
            34, 11, 0, 0, 0, 0, 0, 0, 0,    // length 11
            01, 02,                         // header_end data_start
            // Hello World
            72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100
        ];

        assert_eq!(buffer, alt_buffer);    
    }

    #[test]
    fn packet_parse() {
        let mut parser = Packet::default();

        let packets: Vec<(Option<ParserErrorId>, &[u8])> = vec![
            // unexpected control: data_start
            //(Some(ParserErrorId::UnxptCtrl), &[32, 32, 02, 01]),
            // unknown control: 006
            //(Some(ParserErrorId::UkwnCtrl), &[32, 32, 06, 01]),
            // unknown control: 090
            //(Some(ParserErrorId::UkwnHeader), &[32, 32, 90, 01]),
        ];


        for (pos , (id, packet)) in packets.iter().enumerate() {
            let result = parser.parse(&mut std::io::BufReader::new(*packet));

            match result {
                Ok(_)  => panic!("test {pos}: expected {:?}", id.clone().unwrap()),
                Err(err) => {
                    if err.id != id.clone().unwrap() {
                        panic!("test {pos}: expected {:?} got {:?}", err.id, id.clone().unwrap())
                    }
                }
            }

            parser.clear();
        }
    }
}
