mod header;
mod product;
mod start;
mod status;
mod types;

// pub use super::protocol;
// pub use super::traits;

//TODO is this idiomatic?
pub(crate) use product::{Product, RawProductFile};
pub(crate) use start::{StartPacket, StartPacketBuilder};
pub(crate) use header::{Header, HeaderBuilder};
// pub(crate) use status::Status;
pub(crate) use types::PacketType;
pub(crate) use types::PacketTag;
pub(crate) use types::Direction;
pub(crate) use types::Row;
