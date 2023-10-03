//! Protocol serializers

use std::io::Write;

use super::packet::{PacketType, PacketTag};
use crate::net::error::{ProtocolError, Result};
use crate::tools::calc_crc;

// use crate::private::{self, Sealed};

// FIXME - That should go to packet ...
pub(crate) trait Packet: Validate {
    type Output;

    const P_SIZE: usize;

    fn to_bytes(&self, buf: &mut impl Write) -> Result<usize>;
    fn from_bytes(buf: &[u8]) -> Result<Self::Output>;
    fn get_type() -> PacketType;
    fn get_tag(&self) -> PacketTag;
}


pub(crate) trait PacketDyn: Packet {
    fn get_dyn_type(&self) -> PacketType;
}

impl<T: Packet> PacketDyn for T {
    fn get_dyn_type(&self) -> PacketType {
        Self::get_type()
    }
}

pub trait Validate {
    fn validate(buf: &[u8]) -> Result<()>;

    fn validate_crc(buf: &[u8]) -> Result<()> {
        if buf.len() < 5 {
            //TODO Not enough data Error --- buffer to short Error, unexpected EOF or like
            return Err(ProtocolError::CRCError);
        }

        let c_crc: &mut [u8; 4] = &mut [0u8; 4];

        calc_crc(&buf[1..buf.len() - 4], c_crc);

        if *c_crc != buf[buf.len() - 4..] {
            return Err(ProtocolError::CRCError);
        }

        Ok(())
    }
}
