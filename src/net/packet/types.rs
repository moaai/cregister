use serde::Deserialize;

//TODO rename - packet type or not packet ... used during initialization of the StartPacket, so confusing

const DATE_REPORT: char = 'Y';
const PLU_EXT: char = 'I';
const START: char = 'S';
const STATUS: char = 'I';

//Working ones or kind of working (for #00, extended Start packet)
//  'M#00' - Unique number
//  '1#00' - Unique number extended
//  'H#00' - goods department
//  'o#00' - extended report cash register report
//  'U#00' - sell report
//  'W#00' - fiscal report if it is a word !!!
//  'Y/Z#00' - period report !!!!!!!!
//  'y/z #00' - period report - w#0.. etc. extended
//  'O#00' - receipt report
//  'k#00' - new receipt report
//  'A#00, A#08' - system flags (other may also work)
//  'B#00' - header (download header info and error ...)

// FIXME: Move it to different package
#[derive(Deserialize, Debug)]
// #[derive(Debug)]
// pub struct Row<'a> {
//     pub ean: &'a str,
//     pub name: &'a str,
//     pub price: &'a str,
//     pub quantity: &'a str,
//     pub ptu: &'a str,
// }
pub(crate) struct Row {
    pub ean: String,
    pub name: String,
    pub price: String,
    pub quantity: String,
    pub ptu: String,
}

// impl<'de> Deserialize<'de> for Row<'_> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         // let res = deserializer.deserialize_se("Row", &["ean"], RowVisitor)?;

//         // let res= deserializer.deserialize_seq(RowVisitor)?;

//         deserializer.deserialize_string(visitor)

//         // let w = Deserialize::deserialize(deserializer)?;
//         println!("=> {:?}", res);
//         Ok(Row {
//             ean: "ean",
//             name: "name",
//             price: "price",
//             quantity: "q",
//             ptu: "ptu",
//         })
//     }
// }

// struct RowVisitor;

// impl<'de> Visitor<'de> for RowVisitor {
//     type Value = Row<'de>;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("Struct Row")
//     }

//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         Err(serde::de::Error::invalid_type(serde::de::Unexpected::Str(v), &self))
//     }

//     fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         self.visit_str(v)
//     }
// }

//pub(crate) trait Packet {
//    fn get_type() -> PacketType;
//}

#[derive(Debug)]
pub enum PacketType {
    StartPacket,
    ProductExt, //Extended packet for product I#00
    DateReport,
    Status,
}

#[derive(Debug)]
pub enum PacketTag {
    D,
    S
}

#[derive(Debug)]
pub(crate) enum Direction {
    Download,
    Upload,
}

impl From<Direction> for u8 {
    fn from(pt: Direction) -> Self {
        match pt {
            Direction::Download => b'0',
            Direction::Upload => b'1',
        }
    }
}

impl From<PacketTag> for u8 {
    fn from(pt: PacketTag) -> Self {
        match pt {
            PacketTag::D => b'D',
            PacketTag::S => b'S',
        }

    }
}

impl From<&PacketType> for u8 {
    fn from(pt: &PacketType) -> Self {
        match *pt {
            PacketType::ProductExt => PLU_EXT as u8,
            PacketType::DateReport => DATE_REPORT as u8,
            PacketType::StartPacket => START as u8,
            PacketType::Status => STATUS as u8,
        }
    }
}

impl From<&PacketType> for Option<(u8, u8)> {
    fn from(pt: &PacketType) -> Self {
        match *pt {
            PacketType::ProductExt => Some((b'0', b'0')),
            PacketType::DateReport => None,
            PacketType::StartPacket => None, //FIXME - There is some sub type specified
            // PacketType::DateReport => Some(('0' as u8, '0' as u8)),
            PacketType::Status => Some((b'0', b'0')),
        }
    }
}

impl From<&PacketType> for Option<[u8; 3]> {
    fn from(pt: &PacketType) -> Self {
        match *pt {
            PacketType::ProductExt => Some([b'#', b'0', b'0']),
            PacketType::DateReport => None,
            PacketType::StartPacket => None,
            PacketType::Status => None,
        }
    }
}

impl From<char> for PacketType {
    fn from(c: char) -> Self {
        match c {
            PLU_EXT => PacketType::ProductExt,
            _ => PacketType::DateReport, //Fake
        }
    }
}

/*
impl PacketType {
    // pub fn get_type(&self) -> u8 {
    //     match *self {
    //         PacketType::ProductExt => PLU_EXT as u8,
    //         PacketType::DateReport => DATE_REPORT as u8,
    //     }
    // }

    // pub fn get_stype(&self) -> Option<(u8, u8)> {
    //     match *self {
    //         PacketType::ProductExt => Some(('0' as u8, '0' as u8)),
    //         PacketType::DateReport => None,
    //         // PacketType::DateReport => Some(('0' as u8, '0' as u8)),
    //     }
    // }
}
*/

impl From<PacketType> for char {
    fn from(p: PacketType) -> Self {
        match p {
            PacketType::ProductExt => PLU_EXT,
            PacketType::DateReport => DATE_REPORT,
            PacketType::StartPacket => START,
            PacketType::Status => STATUS,
        }
    }
}
