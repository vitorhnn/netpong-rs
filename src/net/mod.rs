use protos::ChanMessage;
use protos::chan_message;

use prost::Message;

use std::io::Cursor;

pub struct Channel {
    local_sequence: u32,
    remote_sequence: u32,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            local_sequence: 0,
            remote_sequence: 0
        }
    }

    pub fn make_message(&mut self, message: chan_message::Message) -> ChanMessage {
        let mut msg = ChanMessage::default();
        msg.sequence = self.local_sequence;
        msg.ack = self.remote_sequence;
        msg.message = Some(message);

        self.local_sequence += 1;
        msg
    }

    pub fn decode_message(&mut self, buf: &[u8]) -> Option<ChanMessage> {
        let msg = ChanMessage::decode(&mut Cursor::new(buf)).unwrap();

        self.remote_sequence = msg.sequence;
        Some(msg)
    }
}

