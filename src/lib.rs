//! This crate is a Rust implementation of the Tango cash register protocol for Novitus Next cash
//! register device.
//!
//! It should also handle other devices implementing the protocol described here:
//! https://novitus.pl/file/get/771 .
//!
//! ```
//! use std::net::{SocketAddr, IpAddr, Ipv4Addr};
//!
//! let client = client::Connect(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001)).unwrap();  
//! ```
pub mod cli;
pub mod client;
pub mod device;

pub(crate) mod i18n;
pub(crate) mod net;
pub(crate) mod tools;
