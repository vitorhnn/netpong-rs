use protos::ChanMessage;

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

    pub fn make_message(&self, number: u32) -> ChanMessage {
        let mut msg = ChanMessage::default();
        msg.sequence = self.local_sequence;
        msg.ack = self.remote_sequence;
        msg.number = number;

        msg
    }
}

