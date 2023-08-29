use std::cell::RefCell;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;

use crate::net::codes::Codes;
use crate::net::error::{ProtocolError, Result};
use crate::net::packet::{Direction, Header, HeaderBuilder, PacketType, StartPacketBuilder};
use crate::net::traits::{Packet, PacketDyn}; //TODO move validate to packet
use crate::tools::{calc_crc, ll_dump};

use log::trace;

macro_rules! PKTSCC {
    ($self:ident, $sc:expr, $rc:expr, $closure: tt) => {{
        PKT!($self, $sc)?;

        let cde = $self.read_u8()?; //.expect("Signle byte expected");

        if cde != $rc as u8 {
            return Err(ProtocolError::new_pckt_error(cde.try_into()?, $rc));
        } else {
            $closure()?
        }
        // return Ok(());
    }};
}

//Implement both macros inside protocol to ommit self parameter;
macro_rules! PKTCS {
    ($($self:ident).+, $d:expr, $ic:expr, $rc:expr) => {
        if $d == $ic as u8 {
            PKT!($($self).+, $rc).unwrap();
        } else {
            //TODO impl Display for Codes to remove {:?}
            return Err(ProtocolError::CommunicationError(format!(
                "Incorrect {:?} packet",
                $ic
            )));
        }
    };
}

macro_rules! PKT {
    ($($self:ident).+, $c:expr) => {
        $($self).+.write_code($c)
    };
}

#[derive(Debug)]
pub struct ReadState<T> {
    // packet: Box<dyn Packet>,
    pub packet: Option<T>, //NONE for one byte packets ????
    pub code: Codes,
    // marker: PhantomData<T>
}

#[derive(Debug)]
pub(crate) struct Tango {
    refinput: RefCell<BufReader<TcpStream>>,
    input: BufReader<TcpStream>,
    output: BufWriter<TcpStream>,
}

//Client
impl Tango {
    pub fn new(stream: TcpStream) -> Result<Self> {
        Ok(Self {
            refinput: RefCell::new(BufReader::new(stream.try_clone()?)),
            input: BufReader::new(stream.try_clone()?),
            output: BufWriter::new(stream),
        })
    }

    pub fn read_packet<T: Packet>(&mut self) -> Result<ReadState<T::Output>> {
        trace!("read_packet [{:?}]", T::get_type());
        //TODO: Search in spec for correct maximum buffer size, or specify it in the Packet trait
        // so and force implmentation.

        //
        // TODO: create another method read_frame and put there all low level stuff
        //
        // Maybe I should have OutFrame and InFrame
        //

        let mut buf = [0u8; 512];
        //TODO:
        //Chck for other codes NAK, EOT, WACK etc
        if let Ok(size) = self.input.read(&mut buf) {
            //TODO: remove repetition WACK/EOT/NACK have same handling
            //TODO: handle single packets separately and then check for STX
            //TODO: maybe it should be handled in HEADER

            ll_dump(&buf[..size], || {});

            // let frame = Frame::<T>::from_bytes(&buf)?;

            let header = Header::from_bytes(&buf)?;

            // if frame.header.is_none() {
            //     return Err(ProtocolError::CommunicationError(
            //         "Incorrect packet header".to_owned(),
            //     ));
            // }

            // let header = frame.header.unwrap();
            let rsp: Codes = header.code.try_into()?;

            /*
            let ret:Result<ReadState<T::Output>> = match rsp {
                Codes::NAK | Codes::WACK => Ok(ReadState{packet:None, code:rsp}),
                Codes::STX => Ok(ReadState{packet:None, code:rsp}),
                _ => Ok(ReadState{packet:None, code:rsp})
            };
            */

            if rsp == Codes::Wack || rsp == Codes::Eot {
                return Ok(ReadState {
                    packet: None,
                    code: rsp,
                });
            }

            /*
            if Some(buf[0].try_into()?) == Some(Codes::EOT) {
                return Ok(ReadState {
                    packet: None,
                    code: Codes::EOT,
                });
            }
            */

            if rsp == Codes::Nak {
                eprintln!("Communication interrupted with NAK");
                return Ok(ReadState {
                    packet: None,
                    code: Codes::Eot,
                });
            }

            //Now check if packet is correct
            if rsp == Codes::Stx {
                if size < 5 {
                    return Err(ProtocolError::CommunicationError(format!(
                        "Wrong packet length {}",
                        size
                    )));
                }

                let end: Codes = buf[size - 5].try_into()?;

                if end != Codes::Etx && end != Codes::Etb {
                    return Err(ProtocolError::CommunicationError(format!(
                        "Wrong packet end expected 'Etx' got = '{:?}'",
                        Codes::try_from(buf[size - 5]).expect("Unknow code returned")
                    )));
                }

                // println!("{:?} {:?}", &buf[0], &buf[size - 5]);
                // Here we should validate a frame not packet
                // STX ... ETX
                // CRC

                // let header = Header::deserialize(&buf)?;
                // Header::validate(&buf)?;
                //Validate buffer
                // T::validate(&buf[..size])?;

                // frame.validate_crc(&buf[..size])?;

                //let pckt = <T as crate::net::traits::Deserialize>::deserialize(&buf[..size]);

                let h_size = header.get_size();

                let pckt = T::from_bytes(&buf[..size])?;

                let rs = ReadState {
                    packet: Some(pckt),
                    code: Codes::Stx,
                };

                PKT!(self, Codes::Ack)?;
                trace!("--- ACK ---");

                // T::deserialize(&buf[..size])

                Ok(rs)
            } else {
                Err(ProtocolError::CommunicationError(format!(
                    "Wrong packet start expected 'STX' got = '{:?}'",
                    Codes::try_from(buf[0]).expect("Unknow code returned")
                )))
            }
        } else {
            Err(ProtocolError::CommunicationError(
                "No data to read".to_owned(),
            ))
        }
    }

    fn init_upload(
        &mut self,
        pt: PacketType,
        begin: Option<&str>,
        end: Option<&str>,
    ) -> Result<()> {
        PKTSCC!(
            self,
            Codes::Enq,
            Codes::Ack,
            (|| -> Result<()> {
                // self.write_frame(&FrameBuilder::new().packet(&packet).build())?;

                self.write_start_packet(pt, begin, end, Direction::Upload)?;

                let rsp = self.read_u8()?;

                if rsp != Codes::Ack as u8 {
                    return Err(ProtocolError::CommunicationError(format!(
                        "Incorrect {:?} packet",
                        rsp
                    )));
                }

                Ok(())
            })
        );

        Ok(())
    }

    pub(crate) fn init_download<T>(
        &mut self,
        pt: PacketType,
        begin: Option<&str>,
        end: Option<&str>,
    ) -> Result<()>
    where
        T: Packet<Output = T>,
    {
        trace!(
            "init_download pt={:?}, begin={:?}, end={:?}",
            pt,
            begin,
            end
        );

        // TODO Create Protocol Frame

        //Init ENQ ACK
        // PKTSC!(self, Codes::ENQ, Codes::ACK);
        PKTSCC!(
            self,
            Codes::Enq,
            Codes::Ack,
            (|| -> Result<()> {
                self.write_start_packet(pt, begin, end, Direction::Download)?;

                let rsp = self.read_u8()?;

                if rsp != Codes::Ack as u8 {
                    return Err(ProtocolError::CommunicationError(format!(
                        "Intorrect {:?} packet",
                        rsp
                    )));
                }

                PKTSCC!(
                    self,
                    Codes::Eot,
                    Codes::Enq,
                    (|| -> Result<()> {
                        PKT!(self, Codes::Ack)?;
                        Ok(())
                    })
                );

                Ok(())
            })
        );

        Ok(())
    }

    pub(crate) fn download_packet<T, F>(
        &mut self,
        begin: Option<&str>,
        end: Option<&str>,
        mut op: F,
    ) -> Result<()>
    where
        T: Packet<Output = T>,
        F: FnMut(T),
    {
        trace!("Download packet [{:?}]", T::get_type());

        self.init_download::<T>(T::get_type(), begin, end)
            .expect("Init communication");

        loop {
            // TODO - read frame (header/data ... what with footer??
            // then packet from_bytes will start from index 0 not from 7 (heaader is part of frame)
            match self.read_packet::<T>() {
                Ok(res) => {
                    if res.code == Codes::Wack {
                        continue;
                    } else if res.code == Codes::Stx {
                        op(res.packet.unwrap()); //TODO packet can be none ?? ok or not
                    } else {
                        self.write_code(Codes::Eot).unwrap();
                        break;
                    }
                }
                Err(e) => {
                    // eprintln!("Unabled to get products: {}", e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn upload_packets<T>(&mut self, packets: &Vec<T>) -> Result<()>
    where
        T: Packet + PacketDyn + std::fmt::Debug,
    {
        if packets.is_empty() {
            return Ok(());
        }

        trace!("upload_packets {:?}", T::get_type());

        self.init_upload(T::get_type(), None, None)?;

        for pckt in packets {
            self.write_packet(pckt, Direction::Upload)?;

            let rsp = self.read_u8()?;

            // Handle WACK
            if rsp != Codes::Ack as u8 {
                return Err(ProtocolError::CommunicationError(format!(
                    "Incorrect {:?} packet",
                    rsp
                )));
            }
        }
        PKT!(self, Codes::Eot)?;

        let rsp = self.read_u8()?;
        if rsp != Codes::Eot as u8 {
            return Err(ProtocolError::CommunicationError(format!(
                "Incorrect {:?} packet",
                rsp
            )));
        }

        Ok(())
    }

    fn write_start_packet(
        &mut self,
        pt: PacketType,
        begin: Option<&str>,
        end: Option<&str>,
        dir: Direction,
    ) -> Result<()> {
        trace!("write_start_packet");

        let mut out: Vec<u8> = Vec::new(); // TOOD Create with maximum available value ( find in
                                           // the spec)

        // If no begin or end is provided, use default one
        let packet = StartPacketBuilder::new().begin(begin).end(end).build();

        let header = HeaderBuilder::new()
            .tag(packet.get_tag().into())
            .tpe((&pt).into())
            .stpe((&pt).into())
            .dir(dir.into())
            .build();

        header.to_bytes(&mut out)?;
        packet.to_bytes(&mut out)?;

        out.push(Codes::Etx as u8);

        let mut crc = [0u8; 4];
        calc_crc(&out[1..out.len()], &mut crc);
        out.extend(crc.iter());

        ll_dump(&out, || {});

        // packet.serialize(&mut self.output)?;

        self.output.write_all(&out)?;
        self.output.flush()?;

        Ok(())
    }

    fn write_packet(&mut self, packet: &impl PacketDyn, dir: Direction) -> Result<()> {
        // TODO - add packet type here to prepare header
        //
        // Build header and crc here. Packets will only handle its internals
        // To do it I need to change function signature. It should have generic type from which I
        // should infer type for header

        // 1. Write header
        // 2. Write packet
        // 3. Write footer (EXT + CRC)
        //
        trace!("write_packet");

        // let header = HeaderBuilder::new().tpe().build();

        let header = HeaderBuilder::new()
            .tag(packet.get_tag().into())
            .tpe((&packet.get_dyn_type()).into())
            .stpe((&packet.get_dyn_type()).into())
            .dir(dir.into())
            .build();

        println!("Header = {:?}", header);

        let mut out: Vec<u8> = Vec::new();
        // packet.serialize(&mut out)?;

        header.to_bytes(&mut out)?;

        packet.to_bytes(&mut out)?;

        out.push(Codes::Etx as u8);

        let mut crc = [0u8; 4];
        calc_crc(&out[1..out.len()], &mut crc);
        out.extend(crc.iter());

        ll_dump(&out, || {});

        // packet.serialize(&mut self.output)?;

        self.output.write_all(&out)?;
        self.output.flush()?;
        Ok(())
    }

    // fn write_frame(&mut self, frame: &Frame) -> Result<()> {
    //         trace!("write_frame");
    //         ll_dump(&frame.data, || {});
    //         self.output.write_all(&frame.data)?;
    //         self.output.flush()?;
    //
    //         Ok(())
    // }

    //TODO: maybe return something else
    pub(crate) fn read_u8(&mut self) -> Result<u8> {
        //Remove after moving to same module
        let mut b = [0u8; 1];

        //TODO handle errors
        self.input.read_exact(&mut b)?;

        Ok(b[0])
    }

    pub(crate) fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.input.read(buf)?)
    }

    pub(crate) fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.output.write_all(buf)?;
        Ok(self.output.flush()?)
    }

    #[allow(dead_code)]
    pub fn debug_read(&self, buf: &mut [u8]) -> Result<usize> {
        // self.input.read(buf);
        Ok(self.refinput.borrow_mut().read(buf)?)
    }

    //TODO: add return
    // pub fn write_u8(&mut self, b: u8) -> std::io::Result<usize> {
    //     let size = self.output.write(&[b]);

    //     self.output.flush()?;

    //     //TODO check for response

    //     size
    // }

    pub(crate) fn write_code(&mut self, code: Codes) -> std::io::Result<()> {
        // trace!("Respond with code: {:?}", code);
        self.output.write_all(&[code as u8])?;
        self.output.flush()
    }
}

pub(crate) use PKT;
pub(crate) use PKTCS;

use super::frame::Frame;
