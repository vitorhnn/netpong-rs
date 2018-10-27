extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate bytes;

extern crate ggez;

pub mod net;
pub mod game;

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/netpong.protos.rs"));
}

