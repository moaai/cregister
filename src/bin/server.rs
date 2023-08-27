use cregister::device::CashRegister;

//array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}

//use bincode;
//use serde::{Deserialize, Serialize};

//use log::{debug, error, log_enabled, Level};

/*
fn myserialize<S>(x: &[u8; 40], s : S) -> Result<S::Ok, S::Error>
    where S: Serializer {

        s.serialize_bytes(x)
}
*/

// fn write<P: AsRef<Path>>(path: P, p: Product) -> Result<()> {
//     let mut file = File::create(path)?;

//     let bytes: [u8; size_of::<Product>()] = unsafe { transmute(p) };

//     file.write_all(&bytes)?;

//     Ok(())
// }

fn main() {
    env_logger::init();

    CashRegister::start();
}
