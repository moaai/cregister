//! Product packet
//!
//! Extended PLU packet (I#00)
//! 
//! Only the following fields are used:
//!
//! Offset  Length(bytes)   Description
//! 7       18              ean
//! 25      5               position
//! 30      40              name
//! 70      10              price
//! 80      1               ptu (tax identifier)
//! 110     20              quantity
//!

use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read};

use std::convert::TryInto;
use std::str::FromStr;

use encoding::all::WINDOWS_1250;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use hyphenation::{Hyphenator, Iter, Language, Load, Standard};

use log::{debug, trace};

use serde::ser::{Serialize, SerializeStruct};

use super::{PacketTag, PacketType};

use crate::i18n;

use crate::net::traits::{Packet, Validate};
use crate::net::{error::Result, packet::Row};

use crate::net::error::ProtocolError;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Product {
    ean: u32,
    position: u32,
    name: String,
    price: f32,
    ptu: char,
    precission: u8,
    flags: String, // 4 character text
    section: u8,
    halo: u8,
    tandem: String,
    unit: u8,
    quantity: i16,
    key_code: u16,
}

impl Packet for Product {
    // Or Extend Serialize with get_type method ...
    type Output = Product;
    const P_SIZE: usize = 138;

    fn to_bytes(&self, buf: &mut impl std::io::Write) -> Result<usize> {
        trace!("to_bytes");

        let name = Product::prepare_name(&self.name);

        let ean: [u8; 18] = format!("{:<18}", self.ean).as_bytes()[0..18]
            .try_into()
            .unwrap();

        buf.write_all(&ean)?;

        let position: [u8; 5] = "00000".as_bytes()[0..5].try_into().unwrap();

        buf.write_all(&position)?;

        buf.write_all(&name)?;

        let price: [u8; 10] = format!("{:>10}", self.price * 100.0).as_bytes()[0..10]
            .try_into()
            .unwrap();

        buf.write_all(&price)?;

        let ptu: u8 = self.ptu as u8;

        buf.write_all(&[ptu])?;

        // let _precision: u8 = b'0';

        buf.write_all(&[self.precission])?;

        let flags: [u8; 4] = self.flags.as_bytes()[0..4].try_into().unwrap();

        buf.write_all(&flags)?;

        // let _section: [u8; 2] = "01".as_bytes()[0..2].try_into().unwrap();
        let section: [u8; 2] = format!("{:>02}", self.section).as_bytes()[0..2]
            .try_into()
            .unwrap();
        buf.write_all(&section)?;

        let halo: [u8; 2] = format!("{:>02}", self.halo).as_bytes()[0..2]
            .try_into()
            .unwrap();
        buf.write_all(&halo)?;

        // TODO handle self.tandem
        let tandem: [u8; 18] = "                  ".as_bytes()[0..18].try_into().unwrap();
        buf.write_all(&tandem)?;

        // let _unit: [u8; 2] = [b'0', b'1'];
        let unit: [u8; 2] = format!("{:>02}", self.unit).as_bytes()[0..2]
            .try_into()
            .unwrap();
        buf.write_all(&unit)?;

        let quantity: [u8; 20] = format!("{:>20}", self.quantity).as_bytes()[0..20]
            .try_into()
            .unwrap();

        buf.write_all(&quantity)?;

        // TOOD handle self.key_code
        let key_code: [u8; 3] = [32, 32, 32];
        buf.write_all(&key_code)?;

        Ok(0)
    }

    fn from_bytes(buf: &[u8]) -> Result<Self::Output> {
        // assert!(buf.len() >= 138);

        // TODO Check if buffer is long enough for particular packet
        // Maybe add some lenght variable to packet implementation

        // TODO: Inside validation method check if packet is of Product type
        // Header::validate(buf)?;

        //TODO what if file is broken too short
        // Do I need to keep header. I guess not, only for check.

        //FIXME: It should be converted to some human readable values
        // (i.e. internal product and external product) not bytes
        // Or do not convert it, just provide methods to get human readable thigs
        //
        // Should I unwrap or propagate errors up ??? propagate
        //
        // Create some macros to convert  (7, 18, u32)
        let ean: [u8; 18] = buf[7..7 + 18].try_into()?; //TODO add ?
        let ean = Product::get_numeric::<u32>(&ean);

        let position: [u8; 5] = buf[25..25 + 5].try_into()?;
        let position = Product::get_numeric::<u32>(&position);

        let name: [u8; 40] = buf[30..30 + 40].try_into().unwrap();
        let name = Product::build_name(&name);

        // name: &'a[u8],
        let price: [u8; 10] = buf[70..70 + 10].try_into().unwrap();
        let price: f32 = Product::get_numeric::<f32>(&price) / 100.0;

        let ptu = buf[80] as char;
        let precission = buf[81];

        let flags: [u8; 4] = buf[82..82 + 4].try_into().unwrap();
        let flags = String::from(std::str::from_utf8(&flags).unwrap());

        let section: [u8; 2] = buf[86..86 + 2].try_into().unwrap();
        let section = Product::get_numeric::<u8>(&section);

        let halo: [u8; 2] = buf[88..88 + 2].try_into().unwrap();
        let halo = Product::get_numeric::<u8>(&halo);

        let tandem: [u8; 18] = buf[90..90 + 18].try_into().unwrap();
        let tandem = String::from(std::str::from_utf8(&tandem).unwrap());

        let unit: [u8; 2] = buf[108..108 + 2].try_into().unwrap();
        let unit = Product::get_numeric::<u8>(&unit);

        let quantity: [u8; 20] = buf[110..110 + 20].try_into().unwrap();
        let quantity = String::from(std::str::from_utf8(&quantity).unwrap()).replace(' ', "");
        let quantity = quantity.parse::<i16>().unwrap();

        // let quantity: [u8; 20] = buf[110..110 + 20].try_into().unwrap();
        // let quantity = Product::get_numeric::<u16>(&quantity);
        // println!("{:?}", quantity);

        let key_code: [u8; 3] = buf[130..130 + 3].try_into().unwrap(); // FIXME - 0 * 100 + 1 * 10 + 2
        let key_code = Product::get_numeric::<u16>(&key_code);

        Ok(Self {
            ean,
            position,
            name,
            price,
            ptu,
            precission,
            flags,
            section,
            halo,
            tandem,
            unit,
            quantity,
            key_code,
            // crc,
        })
    }

    fn get_type() -> PacketType {
        PacketType::ProductExt
    }

    fn get_tag(&self) -> PacketTag {
        PacketTag::D
    }
}

const RECORD_SIZE: usize = 138;

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, {:#?}, PRICE = {:?}",
            self.ean, self.name, self.price
        )
    }
}

/// Reduces the size of the product to 40 characters.
///
/// First iteration tries to hyphenate last words using polish dictionary
/// and leave only first hyphen.
/// Second iteration is executed if name is still > 40, leaves only first letter of
/// the product name.
fn hyphenate(name: &str) -> String {
    let path_to_dict = "./dict/pl.standard.bincode";
    let pl_pl = Standard::from_path(Language::Polish, path_to_dict).unwrap();

    let mut v: Vec<String> = name
        .replace('-', " ")
        .split(' ')
        .filter(|item| !item.is_empty())
        .map(|x| x.to_owned())
        .collect();
    let mut v_len: i8 = (v.len() - 1) as i8;

    debug!("To hyphenate = {:?} {}", v.join(" "), v.join(" ").len());

    while v.join(" ").chars().count() > 40 && v_len >= 0 {
        let hyphenated = pl_pl.hyphenate(&v[v_len as usize]);
        let mut it = hyphenated.iter();
        it.mark_with("");
        let chop: Vec<String> = it.collect();

        v[v_len as usize] = chop[0].clone();

        v_len -= 1;
    }

    let mut len: usize = v.join(" ").chars().count();

    // Leave only first letter if still longer than 40
    if len > 40 {
        for elem in v.iter_mut().rev() {
            let b_len = elem.chars().count();
            *elem = elem.chars().next().unwrap().to_string();

            len -= b_len - 1;

            if len <= 40 {
                break;
            }
        }
    }

    v.join(" ")
}

impl Product {
    // FIXME - check if there is 852 encoder
    fn prepare_name(p_name: &str) -> [u8; 40] {
        let name = hyphenate(p_name);

        // Since there is no 852 encoder first convert to WINDOWS_1250 and then substitue polish
        // characters
        let n_bytes: Vec<u8> = WINDOWS_1250.encode(&name, EncoderTrap::Ignore).unwrap();
        let mut n_bytes = i18n::_win1250_to_cp852(&n_bytes);

        let s_len = n_bytes.len();

        if s_len < 40 {
            n_bytes.extend([32].iter().cycle().take(40 - n_bytes.len()));
            n_bytes[s_len] = 0xFF;
        }

        let _name: [u8; 40] = n_bytes.try_into().unwrap();

        _name
    }

    // TODO: pass row from main, but renamed
    // TODO: move everything to separare methods/function
    // Make some product builder ... with name, price, ptu, quantity others are default
    pub(crate) fn from_row(row: &Row) -> Product {
        let ean = row.ean.parse::<u32>().unwrap_or_default();
        let price = row.price.parse::<f32>().unwrap_or_default();
        let ptu = row.ptu.chars().next().unwrap_or_default();
        let quantity = row.quantity.parse::<i16>().unwrap_or_default();

        Product {
            ean,
            position: 0,
            name: row.name.clone(),
            price,
            ptu,
            precission: b'0',
            flags: "0018".to_owned(), // TODO - option to set flag
            section: 1,
            halo: 0,
            tandem: "".to_owned(),
            unit: 1,
            quantity,
            key_code: 0,
        }
    }

    fn build_name(name: &[u8]) -> String {
        let v: Vec<u8> = i18n::cp852_to_win1250(name);

        //0xFF marks and of the name field.
        let chars = if let Some(idx) = name.iter().position(|x| *x == 0xFF) {
            WINDOWS_1250.decode(&v[..idx], DecoderTrap::Ignore).unwrap()
        } else {
            //Assume 40 characters long name
            WINDOWS_1250
                .decode(&v[..NAME_LEN], DecoderTrap::Strict)
                .unwrap()
        };

        chars
    }

    fn get_numeric<T>(input: &[u8]) -> T
    where
        T: FromStr + Default,
    {
        //Input is cp852 not utf-8, but it works since numeric values fit in the ASCII
        let value = std::str::from_utf8(input).unwrap_or("0").trim();
        value.parse::<T>().unwrap_or_default()
    }
}

pub struct ProductFile {
    reader: BufReader<File>,
}

pub struct RawProductFile {
    reader: BufReader<File>,
}

impl RawProductFile {
    //FIXME: fname -> PathBuf
    pub fn new(fname: String) -> std::io::Result<Self> {
        let file = File::open(fname)?;
        Ok(Self {
            reader: BufReader::<File>::new(file),
        })
    }
}

impl Iterator for ProductFile {
    type Item = Result<Product>; //TODO: io result, or my result

    fn next(&mut self) -> Option<Self::Item> {
        let mut p = [0u8; RECORD_SIZE];

        match self.reader.read_exact(&mut p) {
            Ok(_) => {}
            Err(e) => {
                //check if only 0
                //check for ETB/ETX

                if p[0] == 0 && p[RECORD_SIZE - 1] == 0 {
                    //Correct EOF
                    return None;
                }

                return Some(Err(ProtocolError::IoError(e)));
            }
        }

        // let product: Product = { unsafe { transmute(p) } };

        Product::validate(&p).expect("Valid product buffer");

        let product = Product::from_bytes(&p).unwrap();

        Some(Ok(product))
    }
}

impl Iterator for RawProductFile {
    type Item = Result<Vec<u8>>; //TODO: io result, or my result

    fn next(&mut self) -> Option<Self::Item> {
        let mut p = [0u8; RECORD_SIZE];

        match self.reader.read_exact(&mut p) {
            Ok(_) => {}
            Err(e) => {
                if p[0] == 0 && p[RECORD_SIZE - 1] == 0 {
                    return None;
                }

                return Some(Err(ProtocolError::IoError(e)));
            }
        }

        Product::validate(&p).expect("Valid product buffer");

        let mut out: Vec<u8> = Vec::with_capacity(RECORD_SIZE);

        out.extend(p);
        Some(Ok(out))
    }
}

const NAME_LEN: usize = 40;

impl Validate for Product {
    fn validate(buf: &[u8]) -> Result<()> {
        Product::validate_crc(buf)
    }
}

#[allow(dead_code)]
struct ProductCSV<'a> {
    ean: u32,
    name: &'a str,
    price: f32,
    quantity: i32,
    ptu: &'a str,
}

impl Serialize for Product {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ProductCSV", 5)?;
        state.serialize_field("ean", &self.ean)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("price", &self.price)?;
        state.serialize_field("quantity", &self.quantity)?;
        state.serialize_field("ptu", &self.ptu)?;

        state.end()
    }
}
