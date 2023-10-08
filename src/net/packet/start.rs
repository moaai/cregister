//#![allow(unused)]
//! This module implements extended start packet.
//!
use std::fmt::Debug;

use crate::net::error::{ProtocolError, Result};
use crate::net::packet::PacketType;

use crate::net::traits::{Packet, Validate};

use super::PacketTag;

// const HASH: u8 = b'#';

// const SP_CODE: usize = 0;
const SP_TAG: usize = 1; //'S'
// const SP_TPE: usize = 2;
// const SP_STPE: usize = 3;
// const SP_STPE_EXT: Range<usize> = 3..6;
// const SP_DIR: usize = 3;
// const SP_DIR_EXT: usize = 6;
// const SP_BEG: Range<usize> = 4..4 + 18;
// const SP_END: Range<usize> = 22..22 + 18;
// const SP_BEG_EXT: Range<usize> = 7..7 + 18;
// const SP_END_EXT: Range<usize> = 25..25 + 18;
// const SP_ECODE: usize = 40;
// const SP_ECODE_EXT: usize = 43;

macro_rules! PAD {
    ($l:expr, $el: expr) => {
        "0".as_bytes()
            .iter()
            .cycle()
            .take($el - $l)
            .map(|&x| x - '0' as u8)
    };
}

/// Start Packet Implementation
#[derive(Debug, Default)]
#[allow(dead_code)]
pub(crate) struct StartPacket {
    beg: [u8; 18],
    end: [u8; 18],
}

impl StartPacket {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Validate for StartPacket {
    fn validate(buf: &[u8]) -> Result<()> {
        StartPacket::validate_crc(buf)
    }
}

pub(crate) struct StartPacketBuilder {
    sp: StartPacket,
}

impl StartPacketBuilder {
    pub fn new() -> Self {
        Self {
            sp: StartPacket::new(),
        }
    }

    pub fn build(&self) -> StartPacket {
        StartPacket { ..self.sp }
    }

    fn build_e_b<T: AsRef<str>>(elem: &mut [u8], data: T) {
        let data = data.as_ref();
        let l = data.len();
        let mut v: Vec<u8> = Vec::with_capacity(elem.len());

        if l < elem.len() {
            v.extend(PAD!(l, elem.len()));
        }

        // Move above if statement is append instead prepend
        v.extend(data.as_bytes().iter());

        elem.copy_from_slice(&v);
    }

    pub fn begin<T: AsRef<str>>(mut self, b: Option<T>) -> Self {
        //TODO Check for b length ... assert?
        if let Some(b) = b {
            StartPacketBuilder::build_e_b(&mut self.sp.beg, b);
        }
        self
    }

    pub fn end<T: AsRef<str>>(mut self, e: Option<T>) -> Self {
        //TODO Check for b length ... assert?
        if let Some(e) = e {
            StartPacketBuilder::build_e_b(&mut self.sp.end, e);
        }
        self
    }
}

impl Packet for StartPacket {
    type Output = StartPacket;
    const P_SIZE: usize = 66;
    fn to_bytes(&self, buf: &mut impl std::io::Write) -> Result<usize> {
        let mut out: Vec<u8> = Vec::with_capacity(std::mem::size_of::<StartPacket>() - 1_usize);

        out.extend(self.beg.iter());
        out.extend(self.end.iter());

        buf.write_all(&out)?;

        Ok(out.len())
    }

    fn from_bytes(buf: &[u8]) -> Result<Self::Output> {
        //Only for extended packet size
        //assert!(size > 47);

        //TODO Move to some comon header parser and add validation
        // mostly used by server
        let marker = buf[SP_TAG];

        //TODO Common validator
        if marker as char != 'S' {
            return Err(ProtocolError::CommunicationError(
                "Incorrect start packet".to_owned(),
            ));
        }

        let sp = StartPacket {
            beg: Default::default(), //Refactor
            end: Default::default(), // beg,
        };

        Ok(sp)
    }

    fn get_type() -> PacketType {
        PacketType::StartPacket
    }

    fn get_tag(&self) -> PacketTag {
        PacketTag::S
    }
}
