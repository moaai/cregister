//! Custom error definition.

use super::codes::Codes;

pub type Result<T> = std::result::Result<T, ProtocolError>;

//TODO: Fix Error definitions

#[allow(clippy::enum_variant_names)]
pub enum ProtocolError {
    CommunicationError(String),
    CRCError,
    IoError(std::io::Error),
    PacketError { cur: Codes, exp: Codes },
    DeserializeError(std::array::TryFromSliceError),
    Utf8Error(std::str::Utf8Error),
}

impl ProtocolError {
    pub fn new_pckt_error(cur: Codes, exp: Codes) -> ProtocolError {
        ProtocolError::PacketError { cur, exp }
    }
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ProtocolError::CommunicationError(ref err) => std::fmt::Display::fmt(&err, f),
            ProtocolError::CRCError => write!(f, "Incorrect CRC"),
            ProtocolError::IoError(ref err) => write!(f, "IO error: {}", err),
            ProtocolError::PacketError { cur: _, exp: _ } => f.write_str("Packet error"),
            ProtocolError::DeserializeError(err) => write!(f, "Deserialize error {}", err),
            ProtocolError::Utf8Error(err) => write!(f, "Convertion failes {}", err),
        }
    }
}

impl std::fmt::Debug for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let ProtocolError::PacketError { cur, exp } = self {
            let mut builder = f.debug_struct(&format!("{}", self));

            builder.field("expected", exp);
            builder.field("current", cur);

            return builder.finish();
        }

        writeln!(f, "{}", self)
    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<std::array::TryFromSliceError> for ProtocolError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        Self::DeserializeError(e)
    }
}

impl From<std::str::Utf8Error> for ProtocolError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

impl std::error::Error for ProtocolError {}
