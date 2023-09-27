//!
use std::net::{SocketAddr, TcpStream};
use std::path::Path;


use crate::net::error::Result;
use crate::net::packet::{Product, Row};
use crate::net::protocol::Tango;

#[derive(Debug)]
pub struct Client {
    proto: Tango,
}

//TODO: Create some config struct for packet params
// begin: Option<&str>,
// end: Option<&str>,
// PacketType::ProductExt

impl Client {
    pub fn connect(addr: SocketAddr) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            proto: Tango::new(stream)?,
        })
    }

    pub fn get_products<F>(&mut self, begin: Option<&str>, end: Option<&str>, op: F) -> Result<()>
    where
        F: FnMut(Product),
    {
        self.proto.download_packet::<Product, F>(begin, end, op)
    }

    pub fn write_products() -> Result<()> {
        Ok(())
    }

    //TODO
    pub fn send_product(
        &self,
        _ean: &str,
        _namee: &str,
        _price: &str,
        _quantity: &str,
        _ptu: &str,
    ) {
        todo!()
    }

    pub fn upload_products_from_file<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let mut csv_reader = csv::ReaderBuilder::new().from_path(path).unwrap();
        let mut products = Vec::new();

        for result in csv_reader.records() {
            let record = result.unwrap();
            let result: std::result::Result<Row, csv::Error> = record.deserialize(None);
            let record = result.expect("Correct csv structure expected");
            products.push(Product::from_row(&record));
        }

        self.proto.upload_packets::<Product>(&products)
    }
}
