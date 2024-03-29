extern crate netpong_rs;
extern crate prost;

use std::error::Error;
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
    fn new() -> io::Result<ServerState> {
        let socket = UdpSocket::bind("0.0.0.0:3000")?;
        socket.set_nonblocking(true)?;
        Ok(ServerState {
            game_state: game::GameState::new(),
            socket,
            player_one: None,
            player_two: None,
            spectators: Vec::new(),
        })
    }


    fn handle_packet(&mut self, &addr: &SocketAddr, packet: &[u8]) {
        if let Some(ref mut client) = self.player_one {
            if client.addr == addr {
                if let Some(msg) = client.chan.decode_message(packet) {
                    if let ChanMessage::ClientInput(ci) = msg {
                        self.game_state.paddle1_vel.y = ci.yaxis * -3.0;
                    }
                }
            }
        }

        if let Some(ref mut client) = self.player_two {
            if client.addr == addr {
                if let Some(msg) = client.chan.decode_message(packet) {
                    if let ChanMessage::ClientInput(ci) = msg {
                        self.game_state.paddle2_vel.y = ci.yaxis * -3.0;
                    }
                }
            }
        }

        if self.spectators.iter().any(|x| x.addr == addr) {
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
                        let response = chan.make_message(ChanMessage::ServerConnect(ServerConnect{index: 0}));
                        let mut buf = Vec::with_capacity(response.encoded_len());
                        response.encode(&mut buf).unwrap();
                        self.socket.send_to(&buf, addr).unwrap();
                        self.player_one = Some(Client::new(chan, addr));
                        println!("accepted {} as p1", addr);
                    } else if self.player_two.is_none() {
                        let response = chan.make_message(ChanMessage::ServerConnect(ServerConnect{index: 1}));
                        let mut buf = Vec::with_capacity(response.encoded_len());
                        response.encode(&mut buf).unwrap();
                        self.socket.send_to(&buf, addr).unwrap();
                        self.player_two = Some(Client::new(chan, addr));
                        println!("accepted {} as p2", addr);
                    } else {
                        let response = chan.make_message(ChanMessage::ServerFull(ServerIsFull{}));
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("netpong server starting up");

    let mut state = ServerState::new()?;

    println!("listening on {}", state.socket.local_addr()?);

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

        if let Some(ref mut p1) = state.player_one {
            let msg = p1.chan.make_message(ChanMessage::ServerSendWorld(state.game_state.as_protobuf()));
            let mut buf = Vec::with_capacity(msg.encoded_len());
            msg.encode(&mut buf).unwrap();
            state.socket.send_to(&buf, p1.addr).unwrap();
        }

        if let Some(ref mut p2) = state.player_two {
            let msg = p2.chan.make_message(ChanMessage::ServerSendWorld(state.game_state.as_protobuf()));
            let mut buf = Vec::with_capacity(msg.encoded_len());
            msg.encode(&mut buf).unwrap();
            state.socket.send_to(&buf, p2.addr).unwrap();
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