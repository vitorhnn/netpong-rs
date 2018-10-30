extern crate netpong_rs;
extern crate prost;

use std::net::{SocketAddr, UdpSocket};
use std::time::{Instant, Duration};
use std::thread::sleep;

use netpong_rs::game;
use netpong_rs::net::Channel;
use netpong_rs::protos::chan_message::Message as ChanMessage;

use prost::Message;

const UPDATE_FREQUENCY: Duration = Duration::from_millis(1000 / 60);

struct Client {
    chan: Channel,
    addr: SocketAddr
}

impl Client {
    fn new(addr: SocketAddr) -> Client {
        Client {
            chan: Channel::new(),
            addr
        }
    }
}

struct ServerState {
    game_state: game::GameState,
    socket: UdpSocket,
    player_one: Option<Client>,
    player_two: Option<Client>,
    spectators: Vec<Client>,
}

impl ServerState {
    fn new() -> ServerState {
        let socket = UdpSocket::bind("127.0.0.1:3000").unwrap();
        socket.set_nonblocking(true).unwrap();
        ServerState {
            game_state: game::GameState::new(),
            socket,
            player_one: None,
            player_two: None,
            spectators: Vec::new(),
        }
    }

}

fn main() {
    println!("netpong server starting up or something");

    let mut state = ServerState::new();

    let mut previous = Instant::now();
    let mut lag = Duration::new(0, 0);

    let local_client = Client::new("127.0.0.1:3001".parse().unwrap());

    state.spectators.push(local_client);

    'running: loop {
        let current = Instant::now();
        let elapsed = current - previous;
        previous = current;
        lag += elapsed;

        while lag > UPDATE_FREQUENCY {
            state.game_state.update();
            lag -= UPDATE_FREQUENCY;
        }

        for spectator in &mut state.spectators {
            let msg = spectator.chan.make_message(ChanMessage::ServerSendWorld(state.game_state.as_protobuf()));
            let mut buf = Vec::with_capacity(msg.encoded_len());
            msg.encode(&mut buf).unwrap();
            state.socket.send_to(&buf, spectator.addr).unwrap();
        }

        sleep(UPDATE_FREQUENCY - lag);
    }
}