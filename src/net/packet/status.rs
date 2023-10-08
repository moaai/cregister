use super::{PacketTag, PacketType};
use crate::net::error::Result;
use crate::net::traits::{Packet, Validate};

#[derive(Debug)]
pub struct Status;

impl Packet for Status {
    type Output = Status;
    const P_SIZE: usize = 144;
    fn to_bytes(&self, _buf: &mut impl std::io::Write) -> Result<usize> {
        Ok(0)
    }

    fn from_bytes(_buf: &[u8]) -> Result<Status> {
        Ok(Status)
    }
    fn get_type() -> PacketType {
        PacketType::Status
    }
    fn get_tag(&self) -> PacketTag {
        todo!()
    }
}

impl Validate for Status {
    fn validate(_buf: &[u8]) -> Result<()> {
        Ok(())
    }
}
