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
        }
    }
}

impl std::fmt::Debug for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // writeln!(f, "---- Error Debug ---");
        // match *self {
        //     ProtocolError::CommunicationError(ref err) => {
        //         writeln!(f, "{}", err)
        //     },
        //     _ => {
        //         writeln!(f, "FU FU {}", self)
        //     }
        // }

        // let mut builder = f.debug_struct("Protocol BAD");

        // builder.field("what", &"the fuck");

        // builder.finish()

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
        // eprintln!("{:?} {:?} {:?}", e.source(), e.kind(), e.raw_os_error());
        Self::IoError(e)
    }
}

impl From<std::array::TryFromSliceError> for ProtocolError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        Self::DeserializeError(e)
    }
}
