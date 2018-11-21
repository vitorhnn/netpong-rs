use ggez::nalgebra;

use protos::ServerSendWorld;

type Point2f = nalgebra::Point2<f32>;
type Vec2f = nalgebra::Vector2<f32>;

pub const GAME_WIDTH: f32 = 900.0;
pub const GAME_HEIGHT: f32 = 700.0;

pub const BALL_WIDTH: f32 = 20.0;
pub const BALL_HEIGHT: f32 = 20.0;

pub struct GameState {
    pub ball_pos: Point2f,
    ball_vel: Vec2f,
    pub paddle1_pos: Point2f,
    pub paddle1_vel: Vec2f,
    pub paddle2_pos: Point2f,
    pub paddle2_vel: Vec2f,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            ball_pos: Point2f::new(440.0, 340.0),
            ball_vel: Vec2f::new(1.0, 1.0),
            paddle1_pos: Point2f::new(10.0, 250.0),
            paddle1_vel: Vec2f::new(0.0, 0.0),
            paddle2_pos: Point2f::new(860.0, 250.0),
            paddle2_vel: Vec2f::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.ball_pos += self.ball_vel;
        self.paddle1_pos += self.paddle1_vel;
        self.paddle2_pos += self.paddle2_vel;

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
            p1_y: self.paddle1_pos.y,
            p1_dy: self.paddle1_vel.y,
            p2_y: self.paddle2_pos.y,
            p2_dy: self.paddle2_vel.y,
        }
    }

    pub fn from_protobuf(proto: &ServerSendWorld) -> GameState {
        GameState {
            ball_pos: Point2f::new(proto.pos_x, proto.pos_y),
            ball_vel: Vec2f::new(proto.vel_x, proto.vel_y),
            paddle1_pos: Point2f::new(10.0, proto.p1_y),
            paddle1_vel: Vec2f::new(0.0, proto.p1_dy),
            paddle2_pos: Point2f::new(860.0, proto.p2_y),
            paddle2_vel: Vec2f::new(0.0, proto.p2_dy),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

