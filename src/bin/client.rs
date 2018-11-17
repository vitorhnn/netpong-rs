extern crate ggez;
extern crate netpong_rs;
extern crate structopt;
extern crate prost;

use std::io;
use std::net::{UdpSocket, SocketAddr};

use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf;
use ggez::event;
use ggez::event::{EventHandler, Keycode, Mod};
use ggez::timer;
use ggez::graphics;

use structopt::StructOpt;

use prost::Message;

use netpong_rs::game;
use netpong_rs::net::Channel;
use netpong_rs::protos::chan_message::Message as ChanMessage;
use netpong_rs::protos::{ClientConnect, ClientInput};


#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short="a", long="addr")]
    addr: SocketAddr,
    #[structopt(short="s", long="spec")]
    spectate: bool,
}

struct ClientState {
    game_state: game::GameState,
    chan: Channel,
    remote_addr: SocketAddr,
    socket: UdpSocket,
    player: Option<u32>,
}

impl ClientState {
    fn new(remote_addr: SocketAddr) -> ClientState {
        let socket = UdpSocket::bind("127.0.0.1:3001").unwrap();
        ClientState {
            game_state: game::GameState::new(),
            chan: Channel::new(),
            player: None,
            socket,
            remote_addr,
        }
    }
}

impl EventHandler for ClientState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS_TARGET: u32 = 60;

        loop {
            let mut buf = [0; 1024];
            match self.socket.recv_from(&mut buf) {
                Ok((bytes, addr)) => {
                    if addr == self.remote_addr {
                        let filled_buf = &buf[..bytes];
                        if let Some(msg) = self.chan.decode_message(&filled_buf) {
                            if let ChanMessage::ServerSendWorld(w) = msg {
                                self.game_state = game::GameState::from_protobuf(&w);
                            }
                        }
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => panic!("io error while receiving from socket")
            }
        }

        while timer::check_update_time(ctx, FPS_TARGET) {
            println!("{:?}", self.game_state.paddle2_pos);
            self.game_state.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(self.game_state.ball_pos.x, self.game_state.ball_pos.y, 20.0, 20.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(self.game_state.paddle1_pos.x, self.game_state.paddle1_pos.y, 30.0, 90.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(self.game_state.paddle2_pos.x, self.game_state.paddle2_pos.y, 30.0, 90.0))?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let input = match self.player {
            Some(0) => &mut self.game_state.p1_input,
            Some(1) => &mut self.game_state.p2_input,
            _ => panic!("wut"),
        };

        match keycode {
            Keycode::Up => input.yaxis = 1.0,
            Keycode::Down => input.yaxis = -1.0,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let input = match self.player {
            Some(0) => &mut self.game_state.p1_input,
            Some(1) => &mut self.game_state.p2_input,
            _ => panic!("wut"),
        };

        match keycode {
            Keycode::Up | Keycode::Down => input.yaxis = 0.0,
            _ => (),
        }
    }
}


fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    let cb = ContextBuilder::new("netpong", "vitorhnn")
        .window_setup(conf::WindowSetup::default().title("netpong"))
        .window_mode(conf::WindowMode::default().dimensions(900, 700));


    let mut ctx = cb.build().expect("failed to build ggez context");

    let mut state = ClientState::new(opt.addr);

    let connect_msg = state.chan.make_message(ChanMessage::ClientConnect(ClientConnect { spectating: opt.spectate }));
    let mut buf = Vec::with_capacity(connect_msg.encoded_len());
    connect_msg.encode(&mut buf).unwrap();
    state.socket.send_to(&buf, opt.addr).unwrap();

    println!("sent clientconnect. awaiting response");
    {
        let mut buf = [0; 1024];
        match state.socket.recv_from(&mut buf) {
            Ok((bytes, addr)) => {
                if addr == opt.addr {
                    let filled_buf = &buf[..bytes];
                    if let Some(msg) = state.chan.decode_message(&filled_buf) {
                        match msg {
                            ChanMessage::ServerConnect(sc) => {
                                state.player = Some(sc.index);
                            },
                            ChanMessage::ServerFull(_) => {
                                println!("server full");
                            },
                            ChanMessage::ServerSendWorld(_) => {
                                if !opt.spectate {
                                    panic!("received ServerSendWorld before ServerConnect (dropped?)");
                                }
                            },
                            _ => panic!("received bogus packet (proper response dropped?)"),
                        }
                    }
                }
            },
            Err(_) => panic!("io error while reading from socket")
        }
    }

    state.socket.set_nonblocking(true).unwrap();
    event::run(&mut ctx, &mut state).expect("ded");
}
