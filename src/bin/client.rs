extern crate ggez;
extern crate netpong_rs;

use std::io;
use std::net::{UdpSocket, SocketAddr};

use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf;
use ggez::event;
use ggez::event::{EventHandler};
use ggez::timer;
use ggez::graphics;

use netpong_rs::game;
use netpong_rs::net::Channel;
use netpong_rs::protos::chan_message::Message;

struct ClientState {
    game_state: game::GameState,
    chan: Channel,
    socket: UdpSocket,
}

impl ClientState {
    fn new() -> ClientState {
        let socket = UdpSocket::bind("127.0.0.1:3001").unwrap();
        socket.set_nonblocking(true).unwrap();
        ClientState {
            game_state: game::GameState::new(),
            chan: Channel::new(),
            socket,
        }
    }
}

impl EventHandler for ClientState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS_TARGET: u32 = 60;

        let mut buf = [0; 1024];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((bytes, _addr)) => {
                    let filled_buf = &buf[..bytes];
                    if let Some(msg) = self.chan.decode_message(&filled_buf) {
                        if let Some(Message::ServerSendWorld(w)) = msg.message {
                            self.game_state = game::GameState::from_protobuf(&w);
                        }
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => panic!("io error while receiving from socket")
            }
        }

        while timer::check_update_time(ctx, FPS_TARGET) {
            //self.game_state.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(self.game_state.ball_pos.x, self.game_state.ball_pos.y, 50.0, 50.0))?;
        graphics::present(ctx);
        Ok(())
    }
}


fn main() {
    let cb = ContextBuilder::new("netpong", "vitorhnn")
        .window_setup(conf::WindowSetup::default().title("netpong"))
        .window_mode(conf::WindowMode::default().dimensions(300, 300));


    let mut ctx = cb.build().expect("failed to build ggez context");

    let mut state = ClientState::new();

    event::run(&mut ctx, &mut state).expect("ded");
}
