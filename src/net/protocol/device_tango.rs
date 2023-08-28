use std::io::Write;

use crate::net::codes::Codes;
use crate::net::error::{ProtocolError, Result};
use crate::net::packet::{Header, PacketType, Product, RawProductFile};
use crate::net::traits::{Packet, Validate};
use crate::tools::ll_dump;

use super::tango::{PKT, PKTCS};
use super::Tango;

// use log::Level::Debug;
use log::{info, trace};

//TODO move, to not export Serialize/Deserialize traits - wonder if it is possible with macro
//TODO Should expand to nothing when < TRACE
macro_rules! DUMP {
    ($data:ident) => {{
        if log_enabled!(Debug) {
            let mut v: Vec<u8> = Vec::new();
            let size = $data.serialize(&mut v).unwrap();
            tools::ll_bump(&v[..size], |out| {
                debug!("{}", out);
            })
            .unwrap();
        }
    }};
}

pub(crate) struct DeviceTango {
    proto: Tango,
}

//Server - aka. device ... move to separate crate. Use protocol ll stuff, server/device is no part of the protocol
// FIXME - That is DeviceNovitusNext
impl DeviceTango {
    pub(crate) fn new(proto: Tango) -> Self {
        Self { proto }
    }

    pub fn handle_receive(&mut self) -> Result<()> {
        let mut data = [0_u8; 512];

        if let Ok(size) = self.proto.read(&mut data) {
            if size == 0 {
                return Ok(());
            }
            trace!("[handle_receive] Received {:?}", data[0]); //TODO u8 to enum
            PKTCS!(self.proto, data[0], Codes::Enq, Codes::Ack);

            trace!("--- ENQ --- ACK ---");

            //--- COMMON CODE ---
            //1. Check if STX -- ie. if correct packet
            //2. Check Code
            //3. Check subcode
            //4. Start packet contains request type

            //Detect start packet and redirect to correct place

            self.proto.read(&mut data)?;

            let header = Header::from_bytes(&data)?;

            trace!("--- HEADER ---");
            // let sp = self.proto.read_packet::<StartPacket>();

            //
            // if let Err(sp) = sp {
            //     if PKT!(self.proto, Codes::Rvi).is_err() {
            //         return Err(ProtocolError::CommunicationError(
            //             "Failed to respond".to_owned(),
            //         ));
            //     }
            //     return Err(sp);
            // }

            //Give client what it wants
            //TODO do packet may be none?
            // let sp = sp.unwrap().packet.unwrap();

            // let dir = sp.dir as char;
            let dir = header.dir as char;

            // let dir = '1'; // Refactor

            //DUMP!(sp);

            if dir == '1' {
                // println!("{:?}", std::mem::size_of::<Product>());
                let mut data = [0_u8; 138]; //TODO: Read up to size of Product
                info!("Client have some data for us");

                let mut file = std::fs::File::create("output.bin").unwrap();

                //TODO redirect packet to correct reader
                //Consume it
                while let Ok(_size) = self.proto.read(&mut data) {
                    if data[0] == Codes::Eot as u8 {
                        trace!("End of transmission");
                        PKT!(self.proto, Codes::Eot).unwrap();
                        break;
                    }
                    Product::validate(&data)?;
                    // let product = Product::from_slice(&data).unwrap();
                    // let product =
                    //     <Product as crate::net::traits::Deserialize>::deserialize(&data).unwrap();

                    let product = Product::from_bytes(&data)?;

                    // THAT IS INCORRECT. SAVE DATA, AFTER VALIDATION, OTHERWISE THERE WILL BE NO
                    // CRC

                    println!("{}", product);
                    ll_dump(&data[.._size], || {});
                    file.write_all(&data).unwrap();
                    PKT!(self.proto, Codes::Ack).unwrap();
                }
            } else if dir == '0' {
                info!("Client want something from us");

                PKT!(self.proto, Codes::Ack).unwrap();

                PKTCS!(self.proto, self.proto.read_u8()?, Codes::Eot, Codes::Enq);
                trace!("READ [{:?}] WRITE [{:?}]", Codes::Eot, Codes::Enq);

                // Read data
                // Check for EOT
                // Send ENQ

                // Check for ACK

                if self.proto.read_u8()? == Codes::Ack as u8 {
                    trace!("--- Send Data ---");

                    /* Refctor */
                    match (header.tpe as char).into() {
                        PacketType::ProductExt => {
                            info!("Client requested: {:?}", PacketType::ProductExt);
                            self.send_products()?;
                        }
                        PacketType::Status => {
                            todo!()
                        }
                        _ => todo!()
                    }

                    // Send Data
                    //self.respond_with_code(Codes::NAK).unwrap();
                    //PKT!(self, Codes::EOT).unwrap(); // Right now It is our data

                    // c = self.read_u8();

                    // println!("--------------------- {}", c);

                    PKT!(self.proto, Codes::Eot).unwrap(); //EOT

                    trace!("--- EOT ---");

                    // println!("--------------------- {}", c);

                    if self.proto.read_u8()? == Codes::Eot as u8 {
                        trace!("--- Finished ---");
                    }

                    trace!("--- EOT ---");
                } else {
                    todo!()
                }
            }

            //Route to appropriate handler
            /*

            if let Ok(size) = self.input.read(&mut data) {
                ll_dump(&data[..size], || {});
            }

            self.respond_with_code(Codes::EOT).unwrap();
            */
        }
        Ok(())
    }

    fn send_products(&mut self) -> Result<()> {
        // trace!("--- ACK ---");

        //let pf = ProductFile::new(String::from("./one_product.bin")).unwrap();

        // let pf = match RawProductFile::new(String::from("./one_product.bin")) {
        let pf = match RawProductFile::new(String::from("./output_python.bin")) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Opening product file failed with: {}", err);
                std::process::exit(1);
            }
        };

        let mut err: Result<()> = Ok(());

        //FIXME: Why convert to Product, send bytes from file as they are.
        //FIXME: ProductFile iterator should return internal raw product ???

        for product in pf.scan(&mut err, |err, res| match res {
            Ok(o) => Some(o),
            Err(e) => {
                **err = Err(e);
                None
            }
        }) {
            // let data = unsafe { struct_to_u8::<Product>(&product) };
            //let mut data: Vec<u8> = Vec::new();

            //product.serialize(&mut data)?;

            //DUMP!(product);

            // Proto should wrap packet with header and crc

            self.proto.write_all(&product).unwrap();

            trace!("--- DATA ---");

            if self.proto.read_u8()? != Codes::Ack as u8 {
                trace!("--- LOOP END ---");
                break;
            }
            trace!("--- ACK ---");
        }

        if err.is_err() {
            //TODO that scan for Error above is giving me a headache
            trace!("!!!! ERORR PROCESSING !!!!");
            //Incorrect data packet
            PKT!(self.proto, Codes::Rvi).unwrap();
            err?;
        }

        Ok(())
    }
}
