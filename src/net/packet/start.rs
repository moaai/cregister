//! Start packet
//! This is a start packet documentation which is not present in render

use std::fmt::Debug;
use std::ops::Range;

use crate::net::error::{ProtocolError, Result};
use crate::net::packet::PacketType;

// use crate::net::protocol::frame::{Frame, FrameBuilder};
use crate::net::traits::{Packet, PacketDyn, Validate};

use super::{PacketTag};
// use crate::tools::{calc_crc, ll_dump};

const HASH: u8 = b'#';
// const EMPTY_STPE: [u8; 3] = ['0' as u8, '0' as u8, '0' as u8];

const SP_CODE: usize = 0;
const SP_MARKER: usize = 1; //'S'
const SP_TPE: usize = 2;
const SP_STPE: usize = 3;
const SP_STPE_EXT: Range<usize> = 3..6;
const SP_DIR: usize = 3;
const SP_DIR_EXT: usize = 6;
const SP_BEG: Range<usize> = 4..4 + 18;
const SP_END: Range<usize> = 22..22 + 18;
const SP_BEG_EXT: Range<usize> = 7..7 + 18;
const SP_END_EXT: Range<usize> = 25..25 + 18;
const SP_ECODE: usize = 40;
const SP_ECODE_EXT: usize = 43;
// const SP_CRC: Range<usize> = 40..40 + 4;
// const SP_CRC_EXT: Range<usize> = 44..44 + 4;

// struct VValidator<'a> {
//     buf: &'a [u8],
// }

// impl<'a> VValidator<'a> {
//     fn new(buf: &'a [u8]) -> Self {
//         VValidator { buf }
//     }
//     fn validate_crc(&self, rng: Range<usize>) {}
// }

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
    //Header not part of the packet
    // code: u8,
    // marker: u8,
    // pub(crate) tpe: u8,
    // stpe: Option<[u8; 3]>, //TODO  this have to be removed from bytes if None
    // pub(crate) dir: u8,
    beg: [u8; 18],
    end: [u8; 18],
    // ecode: u8,
    // crc: [u8; 4], //Remove ...
}

impl StartPacket {
    pub fn new() -> Self {
        Default::default()
    }

    // pub(crate) fn into_frame(self) -> Frame {
    //     let mut bytes: Vec<u8> = Vec::new();
    //     self.to_bytes(&mut bytes);
    //
    //     FrameBuilder::new()
    //         .header(self.get_dyn_type(), Direction::Upload)
    //         .packet(&bytes)
    //         .build()
    // }
}

impl Validate for StartPacket {
    fn validate(buf: &[u8]) -> Result<()> {
        StartPacket::validate_crc(buf)
    }
}

/*
impl Deserialize for StartPacket {
    type Output = StartPacket;
    fn deserialize(buf: &[u8]) -> Result<Self::Output> {
        //Only for extended packet size
        //assert!(size > 47);

        let code = buf[SP_CODE];

        //TODO Move to some comon header parser
        let marker = buf[SP_MARKER];

        //TODO Common validator
        if marker as char != 'S' {
            return Err(ProtocolError::CommunicationError(
                "Incorrect start packet".to_owned(),
            ));
        }

        let tpe = buf[SP_TPE];
        //        let stpe;

        let stpe = if buf[SP_STPE] as char == '#' {
            Some(buf[SP_STPE_EXT].try_into().unwrap())
        } else {
            None
        };

        let dir;
        let beg;
        let end;
        let ecode;
        // let crc: [u8; 4];

        if stpe.is_some() {
            dir = buf[SP_DIR_EXT];
            beg = buf[SP_BEG_EXT].try_into().unwrap();
            end = buf[SP_END_EXT].try_into().unwrap();
            ecode = buf[SP_ECODE_EXT];
            // crc = buf[SP_CRC_EXT].try_into().unwrap();
        } else {
            dir = buf[SP_DIR];
            beg = buf[SP_BEG].try_into().unwrap();
            end = buf[SP_END].try_into().unwrap();
            ecode = buf[SP_ECODE];
            // crc = buf[SP_CRC].try_into().unwrap();
        }

        //Execute validator

        let sp = StartPacket {
            code,
            marker,
            tpe,
            stpe,
            dir,
            beg, end, ecode,
            // crc,
        };

        Ok(sp)
    }
}
*/

/*
impl Serialize for StartPacket {
    fn serialize(&self, buf: &mut impl std::io::Write) -> Result<usize> {
        trace!("serialize");
        // let mut data = unsafe { struct_to_u8::<StartPacket>(self) };

        //TODO Extend packet with STX

        // println!("Size of StartPacket = {}", std::mem::size_of::<StartPacket>());
        // println!("Size of Option<[u8; 3]> = {}", std::mem::size_of::<Option<[u8; 3]>>());
        // println!("Size of [u8; 3] = {}", std::mem::size_of::<[u8; 3]>());

        let mut out: Vec<u8> = Vec::with_capacity(std::mem::size_of::<StartPacket>() - 1_usize);

        out.push(self.code);
        out.push(self.marker);
        out.push(self.tpe);

        if self.stpe.is_some() {
            out.extend(self.stpe.unwrap().iter());
        }

        //TODO Handle direction
        out.push(self.dir);

        // out.extend("211013".as_bytes());
        // out.extend([0].iter().cycle().take(12));
        // out.extend("211013".as_bytes());
        // out.extend([0].iter().cycle().take(12));

        out.extend(self.beg.iter());
        out.extend(self.end.iter());

        out.push(self.ecode);

        //TODO why not build CRC here???? because it forces mut for self

        // println!("=====> {:?}", &out[1..out.len()]);

        // let mut crc = [0u8; 4];
        //TODO move 1..data.len() - 4 to calc_crc
        // calc_crc(&out[1..out.len()], &mut crc);

        //TODO - is there way to fix it ... I don't want to pass mut to serialize
        //RefCell to the rescue, but so far I am not sure where put crc should be
        // self.crc = crc;

        // out.extend(crc.iter());

        buf.write_all(&out)?;

        // println!("{:?}", out);

        // ll_dump(&out, || {});

        Ok(out.len())
    }
}
*/

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
    //
    // //
    // pub fn dir(mut self, dir: u8) -> Self {
    //     self.sp.dir = dir;
    //     self
    // }

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

        // out.push(self.code);
        // out.push(self.marker);
        // out.push(self.tpe);
        //
        // if self.stpe.is_some() {
        //     out.extend(self.stpe.unwrap().iter());
        // }
        //
        // //TODO Handle direction
        // out.push(self.dir);

        // out.extend("211013".as_bytes());
        // out.extend([0].iter().cycle().take(12));
        // out.extend("211013".as_bytes());
        // out.extend([0].iter().cycle().take(12));

        out.extend(self.beg.iter());
        out.extend(self.end.iter());

        // out.push(self.ecode);

        //TODO why not build CRC here???? because it forces mut for self

        // println!("=====> {:?}", &out[1..out.len()]);

        // let mut crc = [0u8; 4];
        //TODO move 1..data.len() - 4 to calc_crc
        // calc_crc(&out[1..out.len()], &mut crc);

        //TODO - is there way to fix it ... I don't want to pass mut to serialize
        //RefCell to the rescue, but so far I am not sure where put crc should be
        // self.crc = crc;

        // out.extend(crc.iter());

        buf.write_all(&out)?;

        // println!("{:?}", out);

        // ll_dump(&out, || {});

        Ok(out.len())
    }

    fn from_bytes(buf: &[u8]) -> Result<Self::Output> {
        //Only for extended packet size
        //assert!(size > 47);

        let code = buf[SP_CODE];

        //TODO Move to some comon header parser
        let marker = buf[SP_MARKER];

        //TODO Common validator
        if marker as char != 'S' {
            return Err(ProtocolError::CommunicationError(
                "Incorrect start packet".to_owned(),
            ));
        }

        let tpe = buf[SP_TPE];
        //        let stpe;

        // let stpe = if buf[SP_STPE] as char == '#' {
        //     Some(buf[SP_STPE_EXT].try_into()?)
        // } else {
        //     None
        // };

        // let dir;
        // let beg;
        // let end;
        // let ecode;
        // let crc: [u8; 4];

        // if stpe.is_some() {
        //     dir = buf[SP_DIR_EXT];
        //     beg = buf[SP_BEG_EXT].try_into()?;
        //     end = buf[SP_END_EXT].try_into()?;
        //     ecode = buf[SP_ECODE_EXT];
        //     // crc = buf[SP_CRC_EXT].try_into().unwrap();
        // } else {
        //     dir = buf[SP_DIR];
        //     beg = buf[SP_BEG].try_into()?;
        //     end = buf[SP_END].try_into()?;
        //     ecode = buf[SP_ECODE];
        //     // crc = buf[SP_CRC].try_into().unwrap();
        // }
        //
        //Execute validator

        let sp = StartPacket {
            // code,
            // marker,
            // tpe,
            // stpe,
            // dir,
            beg: Default::default(), //Refactor
            end: Default::default(), // beg,
                                     // crc,
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
