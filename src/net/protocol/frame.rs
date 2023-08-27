use std::io::Write;

use crate::{
    net::{
        error::{ProtocolError, Result},
        packet::{Direction, Header, HeaderBuilder, PacketType},
    },
    tools::calc_crc,
};

#[derive(Debug)]
pub(crate) struct Frame {
    pub(crate) header: Option<Header>,
    packet: Vec<u8>,
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            header: None,
            packet: Vec::new(),
        }
    }

    // pub(crate) fn from_bytes(buf: &[u8]) -> Result<Self> {
    //     Frame::validate_crc(buf)?;
    //
    //     // TODO Add header and packet validation
    //
    //     let header = Frame::read_header(buf)?;
    //
    //     // TODO: keep packet as bytes (do not pass Gneric param to Frame)
    //     // to avoid lines as above
    //     let packet = T::from_bytes(buf)?;
    //
    //     Ok(Self {
    //         header: Some(header),
    //         packet: Some(packet),
    //     })
    // }

    fn read_header(buf: &[u8]) -> Result<Header> {
        let header = Header::from_bytes(buf)?;
        Ok(header)
    }

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

pub(crate) struct FrameBuilder {
    frame: Frame,
}

impl FrameBuilder {
    pub(crate) fn new() -> Self {
        Self {
            frame: Frame::new(),
        }
    }

    pub(crate) fn header(mut self, tpe: PacketType, dir: Direction) -> Self {
        let header = HeaderBuilder::new()
            .dir(dir.into())
            .tpe((&tpe).into())
            .stpe((&tpe).into())
            .build();

        self.frame.header = Some(header);
        self
    }

    // pub(crate) fn header(mut self, dir: Direction, tag: PacketTag) -> Self {
    // let header = HeaderBuilder::new()
    //     .dir(dir.into())
    //     .tpe((&T::get_type()).into())
    //     .stpe((&T::get_type()).into())
    //     .build();
    // self.frame.header = Some(header);
    // self
    // }

    pub(crate) fn packet(mut self, buf: &[u8]) -> Self {
        // let mut buf: Vec<u8> = Vec::new();
        // packet.to_bytes(&mut buf).unwrap();
        // self.frame.packet = Some(T::from_bytes(&buf).unwrap());
        
        self.frame.packet.write_all(buf).unwrap();

        self
    }

    pub(crate) fn build(self) -> Frame {
        Frame { ..self.frame }
    }
}

impl Default for FrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// pub(crate) struct FrameBuilder {
//     header: Option<Header>,
//     packet: Vec<u8>,
//     footer: Vec<u8>,
// }
//
// impl FrameBuilder {
//     pub fn new() -> Self {
//         FrameBuilder {
//             header: None,
//             packet: Vec::new(),
//             footer: Vec::new(),
//         }
//     }
//
//     pub fn packet(mut self, packet: &impl Packet) -> FrameBuilder {
//         packet.to_bytes(&mut self.packet).unwrap();
//         FrameBuilder {
//             header: self.header,
//             packet: self.packet,
//             footer: self.footer,
//         }
//     }
//
//     // pub fn footer(mut self) -> Self {
//     //     self
//     // }
// }
//
// impl FrameBuilder {
// pub fn build(self) -> Frame<dyn Packet<Output = Product>> {
//     let mut frame = Frame::new();
//
//     if self.header.is_some() {
//         let mut buf: Vec<u8> = Vec::new();
//         let header = self.header.unwrap();
//         header.serialize(&mut buf).unwrap();
//         frame.data.extend(buf);
//     }
//
//     frame.data.extend(self.packet);
//
//     let etx: u8 = Codes::Etx as u8;
//     frame.data.push(etx);
//
//     let mut crc = [0u8; 4];
//     calc_crc(&frame.data[1..frame.data.len()], &mut crc);
//
//     frame.data.extend(crc.iter());
//     frame
// }
// }
