//! Extended packet header
//!
//! In the specification, the header is not defined as a separate unit.
//! To Reduce code duplication, the first 7 first bytes were extracted from the packet
//! and implemented as common header.
//!
//! Every extended packet starts at offset 8.
//! Regular headers are not supported.
//!
//! Offset  Length  Description
//! 0       1       code (start packet code STX)
//! 1       1       packet tag (for example 'S' is a start packet)
//! 3       1       packet type (for example for extended product 'I')
//! 3       3       packet sub-type (for example #00 for extended product)
//! 6       1       dir,    0 - download from device
//!                         1 - upload to device
//!                         2 - report download (not supported)
//!

use std::convert::TryInto;

use crate::net::codes::Codes;
use crate::net::error::Result;
use crate::net::traits::Validate;

const HASH: u8 = b'#';

const CODE: usize = 0;
const TAG: usize = 1;
const TPE: usize = 2;
const STPE: usize = 3;
const DIR: usize = 3;
const DIR_EXT: usize = STPE + DIR;

#[derive(Debug, Clone, Copy, Default)]
pub struct Header {
    //Fixme: temporarly pub
    pub code: u8,
    pub tag: u8,
    pub tpe: u8,
    pub stpe: Option<[u8; 3]>,
    pub dir: u8,
}

impl Header {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn from_bytes(buf: &[u8]) -> Result<Self> {
        //Header is common
        let code = buf[CODE];
        let tag = buf[TAG];
        let tpe = buf[TPE];

        let stpe = if buf[STPE] as char == '#' {
            Some(buf[STPE..STPE + 3].try_into()?)
        } else {
            None
        };

        let dir = if stpe.is_some() {
            buf[DIR_EXT]
        } else {
            buf[DIR]
        };

        let header = Header {
            code,
            tag,
            tpe,
            stpe,
            dir,
        };

        Ok(header)
    }

    pub(crate) fn to_bytes(self, buf: &mut impl std::io::Write) -> Result<usize> {
        let mut out: Vec<u8> = Vec::with_capacity(std::mem::size_of::<Self>()); //TODO how to handle extra size from Option

        out.push(self.code);
        out.push(self.tag);
        out.push(self.tpe);

        if self.stpe.is_some() {
            out.extend(self.stpe.unwrap().iter());
        }

        out.push(self.dir);

        buf.write_all(&out)?;

        Ok(out.len())
    }
}

impl Validate for Header {
    fn validate(_buf: &[u8]) -> Result<()> {
        Ok(())
    }
}

pub(crate) struct HeaderBuilder {
    header: Header,
}

impl HeaderBuilder {
    pub(crate) fn new() -> Self {
        Self {
            header: Header::new(),
        }
    }

    pub(crate) fn stpe(mut self, stpe: Option<(u8, u8)>) -> Self {
        if let Some(stpe) = stpe {
            // if stpe.is_some() {
            self.header.stpe = Some([HASH, stpe.0, stpe.1]);
        } else {
            self.header.stpe = None;
        }

        self
    }

    pub(crate) fn tpe(mut self, tpe: u8) -> Self {
        self.header.tpe = tpe;
        self
    }

    pub(crate) fn dir(mut self, dir: u8) -> Self {
        self.header.dir = dir;
        self
    }

    pub(crate) fn tag(mut self, tag: u8) -> Self {
        self.header.tag = tag;
        self
    }

    pub(crate) fn build(self) -> Header {
        Header {
            code: Codes::Stx as u8,
            ..self.header
        }
    }
}
