extern crate ggez;
extern crate netpong_rs;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf;
use ggez::event;
use ggez::event::{EventHandler};
use ggez::timer;

struct ClientState {
}

impl EventHandler for ClientState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS_TARGET: u32 = 60;

        while timer::check_update_time(ctx, FPS_TARGET) {
            // update state
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}


fn main() {
    let m = netpong_rs::net::Channel::new().make_message(5);

    println!("{:?}", m);

    let cb = ContextBuilder::new("netpong", "vitorhnn")
        .window_setup(conf::WindowSetup::default().title("netpong"))
        .window_mode(conf::WindowMode::default().dimensions(640, 480));


    let mut ctx = cb.build().expect("failed to build ggez context");

    let mut state = ClientState{};

    event::run(&mut ctx, &mut state);
}
