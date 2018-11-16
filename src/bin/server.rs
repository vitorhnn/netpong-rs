extern crate netpong_rs;
extern crate prost;

use std::net::{SocketAddr, UdpSocket};
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::io;

use netpong_rs::game;
use netpong_rs::net::Channel;
use netpong_rs::protos::ServerIsFull;
use netpong_rs::protos::ServerConnect;
use netpong_rs::protos::chan_message::Message as ChanMessage;

use prost::Message;

const UPDATE_FREQUENCY: Duration = Duration::from_millis(1000 / 60);

struct Client {
    chan: Channel,
    addr: SocketAddr
}

impl Client {
    fn new(chan: Channel, addr: SocketAddr) -> Client {
        Client {
            chan,
            addr,
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

    fn handle_packet(&mut self, &addr: &SocketAddr, packet: &[u8]) {
        println!("received packet");
        if let Some(ref client) = self.player_one {
            if client.addr == addr {
                return;
            }
        }

        if let Some(ref client) = self.player_two {
            if client.addr == addr {
                return;
            }
        }

        if let Some(_) = self.spectators.iter().find(|&x| x.addr == addr) {
            return;
        }

        // unconnected client
        let mut chan = Channel::new();
        if let Some(msg) = chan.decode_message(packet) {
            if let ChanMessage::ClientConnect(cc) = msg {
                if cc.spectating {
                    self.spectators.push(Client::new(chan, addr));
                    println!("accepted {} as a spectator", addr);
                } else {
                    if self.player_one.is_none() {
                        let mut response = chan.make_message(ChanMessage::ServerConnect(ServerConnect{index: 0}));
                        let mut buf = Vec::with_capacity(response.encoded_len());
                        response.encode(&mut buf).unwrap();
                        self.socket.send_to(&buf, addr).unwrap();
                        self.player_one = Some(Client::new(chan, addr));
                        println!("accepted {} as p1", addr);
                    } else if self.player_two.is_none() {
                        let mut response = chan.make_message(ChanMessage::ServerConnect(ServerConnect{index: 1}));
                        let mut buf = Vec::with_capacity(response.encoded_len());
                        response.encode(&mut buf).unwrap();
                        self.socket.send_to(&buf, addr).unwrap();
                        self.player_two = Some(Client::new(chan, addr));
                        println!("accepted {} as p2", addr);
                    } else {
                        let mut response = chan.make_message(ChanMessage::ServerFull(ServerIsFull{}));
                        let mut buf = Vec::with_capacity(response.encoded_len());
                        response.encode(&mut buf).unwrap();
                        self.socket.send_to(&buf, addr).unwrap();
                        println!("{} tried to play but no slots remain", addr);
                    }
                }
            }
        }
    }
}

fn main() {
    println!("netpong server starting up or something");

    let mut state = ServerState::new();

    let mut previous = Instant::now();
    let mut lag = Duration::new(0, 0);

    'running: loop {
        let current = Instant::now();
        let elapsed = current - previous;
        previous = current;
        lag += elapsed;

        let mut buf = [0; 1024];

        loop {
            match state.socket.recv_from(&mut buf) {
                Ok((received, addr)) => {
                    let filled_buf = &buf[..received];
                    state.handle_packet(&addr, filled_buf);
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => panic!("io error while reading from socket")
            }
        }

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