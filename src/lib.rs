extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate bytes;

pub mod net;
pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/netpong.protos.rs"));
}

