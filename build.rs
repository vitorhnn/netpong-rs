extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/chan_message.proto"], &["src/"]).expect("proto compiled failed!");
}