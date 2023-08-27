use std::convert::From;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

use log::info;

use crate::net::protocol::{DeviceTango, Tango};

pub struct CashRegister {}

/*
ctrlc::set_handler(move || {
println!("^C catched, closing server");
stream.shutdown(Shutdown::Both).unwrap();
}).expect("Error setting Ctrl C handler");
*/

//TODO: Add protocol as generic or create device builder
impl CashRegister {
    fn handle_client(stream: TcpStream) {
        let protocol = Tango::new(stream).unwrap();
        let mut device = DeviceTango::new(protocol);

        if let Err(e) = device.handle_receive() {
            eprintln!("Communication error: {}", e); // FIXME: Or ignore ...
        }
    }
    /// Starts all that jazz
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
