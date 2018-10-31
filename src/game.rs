use ggez::nalgebra;

use protos::ServerSendWorld;

type Point2f = nalgebra::Point2<f32>;
type Vec2f = nalgebra::Vector2<f32>;

pub const GAME_WIDTH: f32 = 300.0;
pub const GAME_HEIGHT: f32 = 300.0;

pub const BALL_WIDTH: f32 = 50.0;
pub const BALL_HEIGHT: f32 = 50.0;

pub struct GameState {
    pub ball_pos: Point2f,
    ball_vel: Vec2f,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            ball_pos: Point2f::new(0.0, 0.0),
            ball_vel: Vec2f::new(1.0, 1.0)
        }
    }

    pub fn update(&mut self) {
        self.ball_pos += self.ball_vel;

        if self.ball_pos.x + BALL_WIDTH > GAME_WIDTH {
            self.ball_vel.x *= -1.0;
        }

        if self.ball_pos.x < 0.0 {
            self.ball_vel.x *= -1.0;
        }

        if self.ball_pos.y + BALL_HEIGHT > GAME_HEIGHT {
            self.ball_vel.y *= -1.0;
        }

        if self.ball_pos.y < 0.0 {
            self.ball_vel.y *= -1.0;
        }
    }

    pub fn as_protobuf(&self) -> ServerSendWorld {
        ServerSendWorld {
            pos_x: self.ball_pos.x,
            pos_y: self.ball_pos.y,
            vel_x: self.ball_vel.x,
            vel_y: self.ball_vel.y,
            p1_x: 0.0, // STUB
            p2_x: 0.0 // STUB
        }
    }

    pub fn from_protobuf(proto: &ServerSendWorld) -> GameState {
        GameState {
            ball_pos: Point2f::new(proto.pos_x, proto.pos_y),
            ball_vel: Vec2f::new(proto.vel_x, proto.vel_y),
        }
    }
}
