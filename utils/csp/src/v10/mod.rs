use std::io::Read;

use crate::{
    parser::{ParseError, ParseErrorId},
    traits::{CspControl, CspData, CspDataError, CspHeader, CspMethod, CspPacket},
    Version,
};
use flate2::read::GzDecoder;
pub use spec::*;

pub mod spec;

const HEADER_LOWER_VALUE: u8 = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    headers: [Option<Header>; 8],
    method: Option<Method>,
    version: Version,
    data: Vec<u8>,
}

const DEFAULT_HEADER_FILLER: std::option::Option<spec::Header> = None;
impl CspPacket for Packet {
    type HEADER = Header;
    type METHOD = Method;

    fn new() -> Self {
        Packet {
            headers: [DEFAULT_HEADER_FILLER; 8],
            method: None,
            version: Version::V10,
            data: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn version(&self) -> Version {
        self.version
    }

    fn method(&self) -> Option<Self::METHOD> {
        self.method
    }

    fn set_method(&mut self, method: Self::METHOD) {
        self.method = Some(method)
    }

    fn get_header<T: ToString>(&self, header: T) -> Option<Self::HEADER> {
        let h = Header::from_str(header);
        h.as_ref()?;

        let at = h.unwrap().to_u8() - HEADER_LOWER_VALUE;
        self.headers[at as usize].clone()
    }

    fn get_headers(&self) -> Vec<Self::HEADER> {
        self.headers.iter().filter_map(|h| h.clone()).collect()
    }

    fn set_header(&mut self, header: Self::HEADER) {
        let at = header.to_u8() - HEADER_LOWER_VALUE;
        self.headers[at as usize] = Some(header);
    }

    fn set_headers(&mut self, headers: &[Self::HEADER]) {
        for header in headers.iter() {
            self.set_header(header.clone())
        }
    }

    fn pop_header<T: ToString>(&mut self, header: T) -> Option<Self::HEADER> {
        let h = Header::from_str(header);
        h.as_ref()?;

        let at = h.unwrap().to_u8() - HEADER_LOWER_VALUE;
        let pop = self.headers[at as usize].clone();
        self.headers[at as usize] = None;

        pop
    }

    fn data<Data: for<'a> CspData<'a>>(&mut self) -> Result<Data, CspDataError> {
        Data::from_msgpack(self.data.as_slice())
    }

    fn set_data<'de>(&mut self, data: &impl CspData<'de>) -> Result<(), CspDataError> {
        self.data = data.to_msgpack()?;
        self.set_header(Header::Length(self.data.len() as u64));
        Ok(())
    }

    fn prepare(&mut self) -> Result<Vec<u8>, ParseError> {
        let mut buf = Vec::with_capacity(32 + self.len());
        if self.method.is_none() {
            let mut error = ParseError::new(ParseErrorId::MissHeader, 2);
            error.set_desc("{1}", "method");
            return Err(error);
        }
        buf.push(self.version.to_u8());
        buf.push(self.method.unwrap().into());

        if !self.data.is_empty() {
            self.set_header(Header::Length(self.data.len() as u64));
        }

        for header in self.headers.iter().flatten() {
            buf.append(&mut header.to_buffer());
        }
        buf.push(Control::HeaderEnd.into());

        if !self.data.is_empty() {
            buf.append(&mut self.data.clone());
        }

        Ok(buf)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty() && self.get_headers().is_empty() && self.method.is_none()
    }

    fn clear_headers(&mut self) {
        for i in 0..self.headers.len() {
            self.headers[i] = None;
        }
        self.method = None;
    }

    fn clear_data(&mut self) {
        self.data.clear()
    }

    fn clear(&mut self) {
        self.clear_data();
        self.clear_headers()
    }

    fn parse(
        &mut self,
        parser: &mut crate::parser::Parser<impl std::io::Read>,
    ) -> crate::parser::ParseResult<usize> {
        parser.reset();
        self.clear();

        let byte = parser.read_byte()?;
        let version = Version::from_u8(byte);
        if let Some(version) = version {
            self.version = version;
        } else {
            let mut error = ParseError::new(ParseErrorId::UkwnHeaderVal, 2);
            error.set_desc("{1}", "version");
            error.set_desc("{2}", byte);
            return Err(error);
        }

        let byte = parser.read_byte()?;
        let method = Method::from_u8(byte);
        if let Some(method) = method {
            self.method = Some(method);
        } else {
            let mut error = ParseError::new(ParseErrorId::UkwnHeaderVal, 2);
            error.set_desc("{1}", "method");
            error.set_desc("{2}", byte);
            return Err(error);
        }

        loop {
            let byte = parser.peek();
            if byte.is_none() {
                let error = ParseError::new(ParseErrorId::ConnectionClosed, 2);
                return Err(error);
            }
            let byte = byte.unwrap();

            if byte < 32 {
                match Control::try_from(parser.read_byte()?) {
                    Ok(Control::HeaderEnd) => break,
                    Ok(e) => {
                        let mut error = ParseError::new(ParseErrorId::UnxptCtrl, parser.pos());
                        error.set_desc("{1}", e.to_str());
                        return Err(error);
                    }
                    Err(_) => {
                        let mut error = ParseError::new(ParseErrorId::UkwnCtrl, parser.pos());
                        error.set_desc("{1}", format!("{:03}", byte));
                    }
                }
            } else {
                self.set_header(Header::from_buffer(parser)?);
            }
        }

        if let Some(Header::Length(length)) = self.get_header("length") {
            let length = length as usize;
            let data = parser.read(length)?;
            if data.len() != length {
                let error = ParseError::new(ParseErrorId::InvDataLen, parser.pos());
                return Err(error);
            }

            if let Some(Header::Compressed(true)) = self.get_header("compressed") {
                let mut buf = Vec::new();

                let mut decoder = GzDecoder::new(data.as_slice());
                if let Err(err) = decoder.read_to_end(&mut buf) {
                    let mut error = ParseError::new(ParseErrorId::InvDataComp, parser.pos());
                    error.set_desc("{1}", err.to_string());
                    return Err(error);
                }
            } else {
                self.data = data
            }
        }

        Ok(parser.pos())
    }

    fn parse_async(
        &mut self,
        _parser: &mut crate::parser::AsyncParser<impl smol::io::AsyncRead + Unpin>,
    ) -> crate::parser::ParseResult<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, v10::*};

    #[test]
    fn packet_headers() {
        let mut packet = Packet::new();
        assert_eq!(packet.version(), Header::get_version());
        assert_eq!(packet.version(), Method::get_version());

        assert_eq!(packet.method(), None);
        packet.set_header(Header::Length(31));
        packet.set_header(Header::Server(1001));
        packet.set_header(Header::Identity("abcde".to_string()));
        packet.set_header(Header::Compressed(true));
        packet.set_method(Method::Connect);

        assert_eq!(packet.get_header("length"), Some(Header::Length(31)));
        assert_eq!(packet.get_header("server"), Some(Header::Server(1001)));
        assert_eq!(
            packet.get_header("identity"),
            Some(Header::Identity("abcde".to_string()))
        );
        assert_eq!(
            packet.get_header("compressed"),
            Some(Header::Compressed(true))
        );
        assert_eq!(packet.method(), Some(Method::Connect));

        packet.set_headers(&vec![
            Header::Id(1),
            Header::Server(110),
            Header::Compressed(false),
        ]);

        packet.pop_header("length");

        for packet in packet.get_headers() {
            match packet {
                Header::Server(e) => assert_eq!(e, 110),
                Header::Identity(e) => assert_eq!(e, "abcde".to_string()),
                Header::Id(e) => assert_eq!(e, 1),
                Header::Compressed(e) => assert_eq!(e, false),
                _ => panic!("Wrong packet header {:?}", packet),
            }
        }
    }

    #[test]
    fn packet_data() {
        #[derive(serde::Deserialize, serde::Serialize, Debug, Default, PartialEq, Eq)]
        struct User {
            username: String,
            age: u8,
            hobbies: Vec<String>,
        }
        impl CspData<'_> for User {}

        let mut packet = Packet::new();

        let user = User {
            username: "little endian".to_string(),
            age: 20,
            hobbies: vec!["biologie".to_string(), "dev".to_string()],
        };

        packet.set_data(&user).unwrap();

        let user2 = packet.data().unwrap();

        assert_eq!(user, user2);
    }

    #[test]
    fn packet_parse_prepare() {
        #[derive(serde::Deserialize, serde::Serialize, Debug, Default, PartialEq, Eq)]
        struct User {
            username: String,
            hash: String,
        }
        impl CspData<'_> for User {}

        let mut packet = Packet::new();

        packet.set_method(Method::Connect);
        packet.set_headers(&vec![
            Header::Update(true),
            Header::Server(45),
            Header::Identity("abcde".to_string()),
        ]);

        let user = User {
            username: "little endian".to_string(),
            hash: "ae45f4c89aa436d78e4592c214effa4d".to_string(),
        };
        packet.set_data(&user).unwrap();

        let buf = packet.prepare().unwrap();
        let mut valid_buf: Vec<u8> = vec![Version::V10.into(), Method::Connect.into()];

        let data_buf = b"\x92\xadlittle endian\xd9 ae45f4c89aa436d78e4592c214effa4d";

        valid_buf.append(&mut Header::Server(45).to_buffer());
        valid_buf.append(&mut Header::Length(data_buf.len() as u64).to_buffer());
        valid_buf.append(&mut Header::Identity("abcde".to_string()).to_buffer());
        valid_buf.append(&mut Header::Update(true).to_buffer());
        valid_buf.push(Control::HeaderEnd.to_u8());
        valid_buf.append(&mut data_buf.to_vec());

        assert_eq!(buf, valid_buf);

        let mut parser = Parser::new(valid_buf.as_slice(), Version::V10);
        let mut packet2 = Packet::new();
        packet2.parse(&mut parser).unwrap();

        assert_eq!(packet2, packet);
    }
}
