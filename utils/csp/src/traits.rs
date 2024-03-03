use std::{fmt::Debug, io::{Read, Write}};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use rmp_serde::{decode, encode, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use smol::io::AsyncRead;

use crate::{parser::{AsyncParser, ParseError, ParseResult, Parser}, Version};

// ======================= Header =======================

pub trait CspHeader: Debug + Clone + PartialEq + Eq {
    /// checks if an header is the same as an other, regardless of it's content
    fn matches(&self, b: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
    }

    fn get_version() -> crate::Version;
    fn version(&self) -> crate::Version;

    /// get an header with default content from a string
    fn from_str<T: ToString>(s: T) -> Option<Self>;
    /// get a string representation of an header, regardless of it's content
    fn to_str(&self) -> &'static str;

    /// parse an byte to it's header representation with default content
    fn from_u8(byte: u8) -> Option<Self>;
    /// get the byte representation of an header
    fn to_u8(&self) -> u8;

    // exports the header and it's content to a u8 vector
    fn to_buffer(&self) -> Vec<u8>;

    fn from_buffer(parser: &mut Parser<impl Read>) -> ParseResult<Self>;
    
    #[allow(async_fn_in_trait)]
    async fn from_buffer_async(parser: &mut AsyncParser<impl AsyncRead + Unpin>) -> ParseResult<Self>;
}

// ======================= Method =======================

pub trait CspMethod: Debug + Copy + Clone + PartialEq + Eq {
    fn get_version() -> crate::Version;
    fn version(&self) -> crate::Version;

    /// parse a byte to a Method
    fn from_u8(byte: u8) -> Option<Self>;
    /// get the byte representation of a Method
    fn to_u8(&self) -> u8;

    /// parse a string to a Method
    fn to_str(&self) -> &'static str;
    /// get the string representation of a Method
    fn from_str<T: ToString>(s: T) -> Option<Self>;
}

// ======================= Control =======================

pub trait CspControl: Debug + Copy + Clone + PartialEq + Eq {
    fn get_version() -> crate::Version;
    fn version(&self) -> crate::Version;

    /// parse a byte to a Control
    fn from_u8(byte: u8) -> Option<Self>;
    /// get the byte representation of a Control
    fn to_u8(&self) -> u8;

    /// parse a string to a Control
    fn to_str(&self) -> &'static str;
    /// get the string representation of a Control
    fn from_str<T: ToString>(s: T) -> Option<Self>;
}

// ======================= Packet =======================

pub trait CspPacket {
    type HEADER;
    type METHOD;

    fn new() -> Self;
    
    fn len(&self) -> usize;
    
    fn version(&self) -> Version;

    fn method(&self) -> Option<Self::METHOD>;
    fn set_method(&mut self, method: Self::METHOD);


    fn get_header<T: ToString>(&self, header: T) -> Option<Self::HEADER>;
    fn get_headers(&self) -> Vec<Self::HEADER>;

    fn set_header(&mut self, header: Self::HEADER);
    fn set_headers(&mut self, headers: &Vec<Self::HEADER>);

    fn pop_header<T: ToString>(&mut self, header: T) -> Option<Self::HEADER>;

    // FIXME offer an interface to dump data as raw bytes
    fn data<Data: for <'a> CspData<'a>>(&mut self) -> Result<Data, CspDataError>;
    fn set_data<'de>(&mut self, data: &impl CspData<'de>) -> Result<(), CspDataError>;

    fn prepare(&mut self) -> Result<Vec<u8>, ParseError>;
    fn is_empty(&self) -> bool;

    fn clear_headers(&mut self);
    fn clear_data(&mut self);
    fn clear(&mut self);
    
    fn parse(&mut self, parser: &mut Parser<impl Read>) -> ParseResult<usize>;
    fn parse_async(&mut self, parser: &mut AsyncParser<impl AsyncRead + Unpin>) -> ParseResult<usize>;
}

// ======================= CspData =======================

#[derive(Debug)]
pub enum CspDataError {
    Compression(std::io::Error),
    Serialization(encode::Error),
    Deserialization(decode::Error)
}

pub trait CspData<'de>: Serialize + Deserialize<'de> {

    fn to_msgpack(&self) -> Result<Vec<u8>, CspDataError> {
        let mut buf = Vec::new();

        if let Err(err) = self.serialize(&mut Serializer::new(&mut buf)) {
            Err(CspDataError::Serialization(err))
        } else {
            Ok(buf)
        }
    }

    fn to_msgpack_compressed(&self) -> Result<Vec<u8>, CspDataError> {
        let buf = self.to_msgpack()?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        
        if let Err(err) = encoder.write_all(buf.as_slice()) {
            return Err(CspDataError::Compression(err))
        }

        let res = encoder.finish();

        if let Err(err) = res {
            Err(CspDataError::Compression(err))
        } else {
            Ok(res.unwrap())
        }
    }


    fn from_msgpack<T: Read>(reader: T) -> Result<Self, CspDataError> {
        let res = Self::deserialize(&mut Deserializer::new(reader));
        if let Err(err) = res {
            return Err(CspDataError::Deserialization(err))
        }
        Ok(res.unwrap())
    }

    fn from_msgpack_compressed<T: Read>(reader: T) -> Result<Self, CspDataError> {
        let mut buf = Vec::new();
        
        let mut decoder = GzDecoder::new(reader);
        if let Err(err) = decoder.read_to_end(&mut buf) {
            return Err(CspDataError::Compression(err))
        }

        Ok(Self::from_msgpack(buf.as_slice())?)
    }
}