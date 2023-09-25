//! This crate is Rust implementation of the Tango cash register protocol for Novitus Next cash
//! register.
//!
//! It should also handle other devices that implements protocol described here:
//! https://novitus.pl/file/get/771 .
//!
//!
pub mod client;
pub mod device;
pub(crate) mod i18n;
pub mod net;
pub mod tools;
pub mod cli;

// use net::Protocol;

mod private {
    pub trait Sealed {}

    // impl Sealed for crate::net::packet::StartPacket {}
    // impl Sealed for crate::net::packet::Header {}
    // impl Sealed for crate::net::packet::Product {}
}
