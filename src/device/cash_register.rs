use std::convert::From;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

use log::info;

use crate::net::protocol::{DeviceNovitusNext, Tango};

pub struct CashRegister {}

impl CashRegister {
    fn handle_client(stream: TcpStream) {
        let protocol = Tango::new(stream).unwrap();
        let mut device = DeviceNovitusNext::new(protocol);

        if let Err(e) = device.handle_receive() {
            eprintln!("Communication error: {}", e); // FIXME: Or ignore ...
        }
    }

    //Starts device
    pub fn start() {
        let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 5001))).unwrap();
        info!("Server is listening on port 5001");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || CashRegister::handle_client(stream));
                }
                Err(e) => {
                    eprintln!("Error {}", e);
                }
            }
        }
    }
}
