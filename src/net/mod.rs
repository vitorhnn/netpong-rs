use protos::ChanMessage;
use protos::chan_message::Message;
use protos::ClientAskChallenge;

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

    pub fn make_message(&self, message: Message) -> ChanMessage {
        let mut msg = ChanMessage::default();
        msg.sequence = self.local_sequence;
        msg.ack = self.remote_sequence;
        msg.message = Some(message);

        msg
    }
}

