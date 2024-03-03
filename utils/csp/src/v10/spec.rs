use std::io::Read;

use smol::io::AsyncRead;

// see /doc/csp/v1.0.md
use crate::{
    parser::{AsyncParser, ParseError, ParseErrorId, ParseResult, Parser},
    traits::{CspControl, CspHeader, CspMethod},
};

// ======================= Header =======================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header {
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
    Client(String),
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
    /// if a data part is compressed
    ///
    /// the compression algorythm is specific to the csp version
    /// in 1.0 it's gzip that is used
    Compressed(bool),
}

impl CspHeader for Header {
    fn get_version() -> crate::Version {
        crate::Version::V10
    }

    fn version(&self) -> crate::Version {
        crate::Version::V10
    }

    fn from_str<T: ToString>(s: T) -> Option<Self> {
        match s.to_string().to_lowercase().as_str() {
            "server" => Some(Header::Server(0)),
            "length" => Some(Header::Length(0)),
            "identity" => Some(Header::Identity("".to_string())),
            "update" => Some(Header::Update(false)),
            "client" => Some(Header::Client("".to_string())),
            "id" => Some(Header::Id(0)),
            "reconnect" => Some(Header::Reconnect(false)),
            "compressed" => Some(Header::Compressed(false)),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Header::Server(_) => "server",
            Header::Length(_) => "length",
            Header::Identity(_) => "identity",
            Header::Client(_) => "client",
            Header::Update(_) => "update",
            Header::Id(_) => "id",
            Header::Reconnect(_) => "reconnect",
            Header::Compressed(_) => "compressed",
        }
    }

    fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            32 => Some(Header::Server(0)),
            33 => Some(Header::Length(0)),
            34 => Some(Header::Identity("".to_string())),
            35 => Some(Header::Client("".to_string())),
            36 => Some(Header::Update(false)),
            37 => Some(Header::Id(0)),
            38 => Some(Header::Reconnect(false)),
            39 => Some(Header::Compressed(false)),
            _ => None,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            Header::Server(_) => 32,
            Header::Length(_) => 33,
            Header::Identity(_) => 34,
            Header::Client(_) => 35,
            Header::Update(_) => 36,
            Header::Id(_) => 37,
            Header::Reconnect(_) => 38,
            Header::Compressed(_) => 39,
        }
    }

    fn to_buffer(&self) -> Vec<u8> {
        match self {
            // string
            Header::Identity(ref v) | Header::Client(ref v) => {
                let mut vector = vec![self.to_u8()];
                vector.push(Control::StringStart.into());
                vector.append(&mut v.clone().into_bytes());
                vector.push(Control::StringEnd.into());
                vector
            }
            // u32
            Header::Server(v) => {
                let mut vector = vec![self.to_u8()];
                vector.append(&mut v.to_le_bytes().to_vec());
                vector
            }
            // u64
            Header::Length(v) | Header::Id(v) => {
                let mut vector = vec![self.to_u8()];
                vector.append(&mut v.to_le_bytes().to_vec());
                vector
            }
            // bool
            Header::Update(v) | Header::Reconnect(v) | Header::Compressed(v) => {
                if *v {
                    vec![self.to_u8()]
                } else {
                    vec![]
                }
            }
        }
    }

    fn from_buffer(parser: &mut Parser<impl Read>) -> ParseResult<Self> {
        let byte = parser.read_byte()?;
        let header: Header = match byte.try_into() {
            Ok(h) => h,
            Err(_) => {
                let mut err = ParseError::new(ParseErrorId::UkwnHeader, parser.pos());
                err.set_desc("{1}", format!("{:03}", byte).as_str());
                return Err(err);
            }
        };

        match header {
            Header::Server(_) => Ok(Header::Server(parser.read_u32()?)),
            Header::Length(_) => Ok(Header::Length(parser.read_u64()?)),
            Header::Id(_) => Ok(Header::Id(parser.read_u64()?)),
            Header::Identity(_) => Ok(Header::Identity(parser.read_string()?)),
            Header::Client(_) => Ok(Header::Client(parser.read_string()?)),
            Header::Update(_) => Ok(Header::Update(true)),
            Header::Reconnect(_) => Ok(Header::Reconnect(true)),
            Header::Compressed(_) => Ok(Header::Compressed(true)),
        }
    }

    async fn from_buffer_async(parser: &mut AsyncParser<impl AsyncRead + Unpin>) -> ParseResult<Self> {
        let byte = parser.read_byte().await?;
        let header: Header = match byte.try_into() {
            Ok(h) => h,
            Err(_) => {
                let mut err = ParseError::new(ParseErrorId::UkwnHeader, parser.pos());
                err.set_desc("{1}", format!("{:03}", byte).as_str());
                return Err(err);
            }
        };

        match header {
            Header::Server(_) => Ok(Header::Server(parser.read_u32().await?)),
            Header::Length(_) => Ok(Header::Length(parser.read_u64().await?)),
            Header::Id(_) => Ok(Header::Id(parser.read_u64().await?)),
            Header::Identity(_) => Ok(Header::Identity(parser.read_string().await?)),
            Header::Client(_) => Ok(Header::Client(parser.read_string().await?)),
            Header::Update(_) => Ok(Header::Update(true)),
            Header::Reconnect(_) => Ok(Header::Reconnect(true)),
            Header::Compressed(_) => Ok(Header::Compressed(true)),
        }
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        Self::to_str(self).to_string()
    }
}

impl From<Header> for u8 {
    fn from(val: Header) -> Self {
        Header::to_u8(&val)
    }
}

impl From<Header> for Vec<u8> {
    fn from(val: Header) -> Self {
        Header::to_buffer(&val)
    }
}

impl TryFrom<u8> for Header {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Header as TryFrom<u8>>::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl TryFrom<String> for Header {
    type Error = ();

    fn try_from(value: String) -> Result<Self, <Header as TryFrom<String>>::Error> {
        match Self::from_str(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl TryInto<String> for Header {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Header::Client(v) => Ok(v),
            Header::Identity(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl TryInto<u32> for Header {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Header::Server(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl TryInto<u64> for Header {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Header::Length(v) => Ok(v),
            Header::Id(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl TryInto<bool> for Header {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Header::Update(v) => Ok(v),
            Header::Reconnect(v) => Ok(v),
            Header::Compressed(v) => Ok(v),
            _ => Err(()),
        }
    }
}

// ======================= Method =======================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// Connect to the Server
    ///
    /// Allways sent by the Client
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

impl CspMethod for Method {
    fn get_version() -> crate::Version {
        crate::Version::V10
    }

    fn version(&self) -> crate::Version {
        crate::Version::V10
    }

    fn from_u8(byte: u8) -> Option<Method> {
        match byte {
            32 => Some(Method::Connect),
            33 => Some(Method::Auth),
            34 => Some(Method::Disconnect),
            35 => Some(Method::Admin),
            36 => Some(Method::Update),
            37 => Some(Method::Action),
            38 => Some(Method::Error),
            39 => Some(Method::State),
            _ => None,
        }
    }
    fn to_u8(&self) -> u8 {
        match self {
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

    fn to_str(&self) -> &'static str {
        match self {
            Method::Connect => "connect",
            Method::Auth => "auth",
            Method::Disconnect => "disconnect",
            Method::Admin => "admin",
            Method::Update => "update",
            Method::Action => "action",
            Method::Error => "error",
            Method::State => "state",
        }
    }

    fn from_str<T: ToString>(s: T) -> Option<Self> {
        match s.to_string().to_lowercase().as_str() {
            "connect" => Some(Method::Connect),
            "auth" => Some(Method::Auth),
            "disconnect" => Some(Method::Disconnect),
            "admin" => Some(Method::Admin),
            "update" => Some(Method::Update),
            "action" => Some(Method::Action),
            "error" => Some(Method::Error),
            "state" => Some(Method::State),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Method {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Method as TryFrom<u8>>::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        Self::to_str(self).to_string()
    }
}

impl From<Method> for u8 {
    fn from(val: Method) -> Self {
        Method::to_u8(&val)
    }
}

impl TryFrom<String> for Method {
    type Error = ();

    fn try_from(value: String) -> Result<Self, <Method as TryFrom<String>>::Error> {
        match Self::from_str(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl From<Method> for &str {
    fn from(val: Method) -> Self {
        Method::to_str(&val)
    }
}

// ======================= Control =======================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    /// Ends of an header part.
    ///
    /// If no Length header have been found, then the Packet ends here, otherwise, a DataStart control
    /// shall be found next, and using the Length header, the data is parsed
    HeaderEnd,
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

impl CspControl for Control {
    fn get_version() -> crate::Version {
        crate::Version::V10
    }

    fn version(&self) -> crate::Version {
        crate::Version::V10
    }

    fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::HeaderEnd),
            2 => Some(Self::StringStart),
            3 => Some(Self::StringEnd),
            _ => None,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            Self::HeaderEnd => 1,
            Self::StringStart => 2,
            Self::StringEnd => 3,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Control::HeaderEnd => "header_end",
            Control::StringStart => "string_start",
            Control::StringEnd => "string_end",
        }
    }

    fn from_str<T: ToString>(s: T) -> Option<Self> {
        match s.to_string().to_lowercase().as_str() {
            "header_end" => Some(Self::HeaderEnd),
            "string_start" => Some(Self::StringStart),
            "string_end" => Some(Self::StringEnd),
            _ => None,
        }
    }
}

impl ToString for Control {
    fn to_string(&self) -> String {
        Self::to_str(self).to_string()
    }
}

impl TryFrom<u8> for Control {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Control as TryFrom<u8>>::Error> {
        match Self::from_u8(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl From<Control> for u8 {
    fn from(val: Control) -> Self {
        Control::to_u8(&val)
    }
}

impl TryFrom<String> for Control {
    type Error = ();

    fn try_from(value: String) -> Result<Self, <Control as TryFrom<String>>::Error> {
        match Self::from_str(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl From<Control> for &str {
    fn from(val: Control) -> Self {
        Control::to_str(&val)
    }
}

// ======================= Tests =======================

#[cfg(test)]
mod tests {
    use crate::v10::spec::*;
    use crate::Version;

    #[test]
    fn header() {
        assert_eq!(Version::V10, Header::get_version());
        assert_eq!(Version::V10, Header::Reconnect(false).version());

        for byte in 32..=39 {
            let header: Header = byte
                .try_into()
                .expect("at byte {byte}, failed to serialize from byte");
            let header_str = header.to_str();
            let header_value = match header {
                Header::Server(_) => Header::Server(500),
                Header::Length(_) => Header::Length(1024),
                Header::Identity(_) => Header::Identity("a2c4e6".to_string()),
                Header::Client(_) => Header::Client("1.0.3-bevy-linux".to_string()),
                Header::Update(_) => Header::Update(true),
                Header::Id(_) => Header::Id(255),
                Header::Reconnect(_) => Header::Reconnect(true),
                Header::Compressed(_) => Header::Compressed(false),
            };

            assert!(header.matches(&header_value), "at byte {byte}");
            assert_eq!(byte, header.to_u8(), "at byte {byte}");
            assert_eq!(
                header,
                header_str.to_string().try_into().expect(
                    format!("at byte {byte}, failed to serialize {header_str} from string")
                        .as_str()
                ),
                "at byte {byte}"
            );

            match header_value {
                Header::Server(_) => {
                    assert_eq!(vec![032, 244, 001, 000, 000], header_value.to_buffer())
                }
                Header::Length(_) => {
                    assert_eq!(
                        vec![033, 000, 004, 000, 000, 000, 000, 000, 000],
                        header_value.to_buffer()
                    )
                }
                Header::Identity(_) => {
                    assert_eq!(
                        vec![034, 002, 097, 050, 099, 052, 101, 054, 003],
                        header_value.to_buffer()
                    )
                }
                Header::Client(_) => {
                    assert_eq!(
                        vec![
                            035, 002, 049, 046, 048, 046, 051, 045, 098, 101, 118, 121, 045, 108,
                            105, 110, 117, 120, 003
                        ],
                        header_value.to_buffer()
                    )
                }
                Header::Update(_) => {
                    assert_eq!(vec![036], header_value.to_buffer())
                }
                Header::Id(_) => {
                    assert_eq!(
                        vec![037, 255, 000, 000, 000, 000, 000, 000, 000],
                        header_value.to_buffer()
                    )
                }
                Header::Reconnect(_) => {
                    assert_eq!(vec![038], header_value.to_buffer())
                }
                Header::Compressed(_) => {
                    assert_eq!(header_value.to_buffer(), vec![])
                }
            }
        }
    }

    #[test]
    fn header_parse() {
        let mut reader: &[u8] = &[
            32, 44, 01, 00, 00, // Server 300
            33, 00, 04, 00, 00, 00, 00, 00, 00, // Length 1024
            34, 02, 97, 50, 99, 52, 101, 54, 03, // Identity a2c4e6
            // Client 1.0.3-bevy-linux
            35, 02, 49, 46, 48, 046, 51, 45, 98, 101, 118, 121, 45, 108, 105, 110, 117, 120, 03,
            36, // Update
            37, 255, 00, 00, 00, 00, 00, 00, 00, // Id 255
            38, // Reconnect
            39, // Compressed
            40, // ERR
        ];

        let mut parser = Parser::new(&mut reader, Version::V10);

        assert_eq!(Header::from_buffer(&mut parser), Ok(Header::Server(300)));
        assert_eq!(Header::from_buffer(&mut parser), Ok(Header::Length(1024)));
        assert_eq!(
            Header::from_buffer(&mut parser),
            Ok(Header::Identity("a2c4e6".to_string()))
        );
        assert_eq!(
            Header::from_buffer(&mut parser),
            Ok(Header::Client("1.0.3-bevy-linux".to_string()))
        );
        assert_eq!(Header::from_buffer(&mut parser), Ok(Header::Update(true)));
        assert_eq!(Header::from_buffer(&mut parser), Ok(Header::Id(255)));
        assert_eq!(
            Header::from_buffer(&mut parser),
            Ok(Header::Reconnect(true))
        );
        assert_eq!(
            Header::from_buffer(&mut parser),
            Ok(Header::Compressed(true))
        );
        assert_eq!(
            Header::from_buffer(&mut parser).err().unwrap().id,
            ParseErrorId::UkwnHeader
        );
    }

    #[test]
    fn header_async_parse() {
        smol::block_on(async {
            let mut reader: &[u8] = &[
                32, 44, 01, 00, 00, // Server 300
                33, 00, 04, 00, 00, 00, 00, 00, 00, // Length 1024
                34, 02, 97, 50, 99, 52, 101, 54, 03, // Identity a2c4e6
                // Client 1.0.3-bevy-linux
                35, 02, 49, 46, 48, 046, 51, 45, 98, 101, 118, 121, 45, 108, 105, 110, 117, 120, 03,
                36, // Update
                37, 255, 00, 00, 00, 00, 00, 00, 00, // Id 255
                38, // Reconnect
                39, // Compressed
                40, // ERR
            ];

            let mut parser = AsyncParser::new(&mut reader, Version::V10);

            assert_eq!(Header::from_buffer_async(&mut parser).await, Ok(Header::Server(300)));
            assert_eq!(Header::from_buffer_async(&mut parser).await, Ok(Header::Length(1024)));
            assert_eq!(
                Header::from_buffer_async(&mut parser).await,
                Ok(Header::Identity("a2c4e6".to_string()))
            );
            assert_eq!(
                Header::from_buffer_async(&mut parser).await,
                Ok(Header::Client("1.0.3-bevy-linux".to_string()))
            );
            assert_eq!(Header::from_buffer_async(&mut parser).await, Ok(Header::Update(true)));
            assert_eq!(Header::from_buffer_async(&mut parser).await, Ok(Header::Id(255)));
            assert_eq!(
                Header::from_buffer_async(&mut parser).await,
                Ok(Header::Reconnect(true))
            );
            assert_eq!(
                Header::from_buffer_async(&mut parser).await,
                Ok(Header::Compressed(true))
            );
            assert_eq!(
                Header::from_buffer_async(&mut parser).await.err().unwrap().id,
                ParseErrorId::UkwnHeader
            );
        })
    }

    #[test]
    fn method() {
        assert_eq!(Version::V10, Method::get_version());
        assert_eq!(Version::V10, Method::Connect.version());

        for byte in 32..=39 {
            let method: Method = byte
                .try_into()
                .expect("at byte {byte}, failed to serialize from byte");
            let method_str = method.to_str();

            assert_eq!(byte, method.to_u8(), "at byte {byte}");
            assert_eq!(
                method,
                method_str.to_string().try_into().expect(
                    format!("at byte {byte}, failed to serialize {method_str} from string")
                        .as_str()
                ),
                "at byte {byte}"
            );
        }
    }

    #[test]
    fn control() {
        assert_eq!(Version::V10, Control::get_version());
        assert_eq!(Version::V10, Control::HeaderEnd.version());

        for byte in 1..=3 {
            let control: Control = byte
                .try_into()
                .expect("at byte {byte}, failed to serialize from byte");
            let control_str = control.to_str();

            assert_eq!(byte, control.to_u8(), "at byte {byte}");
            assert_eq!(
                control,
                control_str.to_string().try_into().expect(
                    format!("at byte {byte}, failed to serialize {control_str} from string")
                        .as_str()
                ),
                "at byte {byte}"
            );
        }
    }
}
