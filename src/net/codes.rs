use std::convert::TryFrom;

use super::error::ProtocolError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Codes {
    Stx = 0x02,
    Etx = 0x03,
    Eot = 0x04,
    Enq = 0x05,
    Ack = 0x06,
    Wack = 0x09,
    Nak = 0x15,
    Etb = 0x17,
    Rvi = 0x40,
}

impl TryFrom<u8> for Codes {
    type Error = ProtocolError;

    fn try_from(value: u8) -> std::result::Result<Codes, Self::Error> {
        //<==== that is broken
        match value {
            0x02 => Ok(Codes::Stx),
            0x03 => Ok(Codes::Etx),
            0x04 => Ok(Codes::Eot),
            0x05 => Ok(Codes::Enq),
            0x06 => Ok(Codes::Ack),
            0x09 => Ok(Codes::Wack),
            0x15 => Ok(Codes::Nak),
            0x17 => Ok(Codes::Etb),
            0x40 => Ok(Codes::Rvi),
            _ => Err(ProtocolError::CommunicationError(
                "Unexpexted code byte".to_owned(),
            )),
        }
    }
}

// impl From<u8> for Codes {
//     fn from(v: u8) -> Self {
//         println!("WTF IS H {}", v);
//         match v {
//             0x02 => Codes::STX,
//             0x03 => Codes::ETX,
//             0x04 => Codes::EOT,
//             0x05 => Codes::ENQ,
//             0x06 => Codes::ACK,
//             0x09 => Codes::WACK,
//             0x15 => Codes::NAK,
//             0x17 => Codes::ETB,
//             0x40 => Codes::RVI,
//             _ => todo!(),
//         }
//     }
// }

// impl From<Codes> for u8 {
//     fn from(code: Codes) -> Self {
//         match code {
//             Codes::STX => 0x02,
//             Codes::ETX => 0x03,
//             Codes::EOT => 0x04,
//             Codes::ENQ => 0x05,
//             Codes::ACK => 0x06,
//             Codes::WACK => 0x09,
//             Codes::NAK => 0x15,
//             Codes::ETB => 0x17,
//             Codes::RVI => 0x40,
//         }
//     }
// }
