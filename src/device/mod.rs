//! Bare bone implementation of the novitus next cash register.
//!
//! The current implementation only supports upload and download extended product(s).
//!
//! The request type is not detected. For dir==1 the request is saved to output.bin and
//! for dir==0 products are sent to the client from the products.bin file.
//!
mod cash_register;

pub use cash_register::CashRegister;
