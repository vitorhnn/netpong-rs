extern crate ggez;
extern crate netpong_rs;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf;
use ggez::event;
use ggez::event::{EventHandler};
use ggez::timer;
use ggez::graphics;

use netpong_rs::game;
use netpong_rs::protos::chan_message::Message;
use netpong_rs::protos::ServerSendChallenge;

struct ClientState {
    game_state: game::GameState
}

impl ClientState {
    fn new() -> ClientState {
        ClientState {
            game_state: game::GameState::new()
        }
    }
}

impl EventHandler for ClientState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS_TARGET: u32 = 60;

        while timer::check_update_time(ctx, FPS_TARGET) {
            self.game_state.update();
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
    let m = netpong_rs::net::Channel::new().make_message(Message::ServerChallenge(ServerSendChallenge{ challenge: 42 }));

    println!("{:?}", m);

    let cb = ContextBuilder::new("netpong", "vitorhnn")
        .window_setup(conf::WindowSetup::default().title("netpong"))
        .window_mode(conf::WindowMode::default().dimensions(640, 480));


    let mut ctx = cb.build().expect("failed to build ggez context");

    let mut state = ClientState::new();

    event::run(&mut ctx, &mut state).expect("ded");
}
