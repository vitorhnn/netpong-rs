use ggez::nalgebra;

use crate::protos::ServerSendWorld;

type Point2f = nalgebra::Point2<f32>;
type Vec2f = nalgebra::Vector2<f32>;

pub const GAME_WIDTH: f32 = 900.0;
pub const GAME_HEIGHT: f32 = 700.0;

pub const BALL_WIDTH: f32 = 20.0;
pub const BALL_HEIGHT: f32 = 20.0;

pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32= 90.0;

pub struct GameState {
    pub p1_score: u32,
    pub p2_score: u32,
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
            ball_vel: Vec2f::new(2.0, 2.0),
            paddle1_pos: Point2f::new(10.0, 250.0),
            paddle1_vel: Vec2f::new(0.0, 0.0),
            paddle2_pos: Point2f::new(860.0, 250.0),
            paddle2_vel: Vec2f::new(0.0, 0.0),
            p1_score: 0,
            p2_score: 0,
        }
    }

    fn ball_collide(&self, min_point: Point2f, max_point: Point2f) -> bool {
        if self.ball_pos.y + BALL_HEIGHT < min_point.y {
            return false;
        }

        if self.ball_pos.y > max_point.y {
            return false;
        }

        if self.ball_pos.x + BALL_WIDTH < min_point.x {
            return false;
        }

        if self.ball_pos.x > max_point.x {
            return false;
        }

        true
    }

    pub fn update(&mut self) {
        self.ball_pos += self.ball_vel;
        self.paddle1_pos += self.paddle1_vel;
        self.paddle2_pos += self.paddle2_vel;


        let p1_min = self.paddle1_pos;
        let p1_max = self.paddle1_pos + Vec2f::new(PADDLE_WIDTH, PADDLE_HEIGHT);

        if self.ball_collide(p1_min, p1_max) {
            self.ball_vel.x *= -1.0;
        }

        let p2_min = self.paddle2_pos;
        let p2_max = self.paddle2_pos + Vec2f::new(0.0, PADDLE_HEIGHT);

        if self.ball_collide(p2_min, p2_max) {
            self.ball_vel.x *= -1.0;
        }

        if self.ball_pos.x + BALL_WIDTH > GAME_WIDTH {
            self.p1_score += 1;
            self.ball_pos = Point2f::new(440.0, 340.0);
            self.ball_vel = Vec2f::new(-2.0, -2.0);
        }

        if self.ball_pos.x < 0.0 {
            self.p2_score += 1;
            self.ball_pos = Point2f::new(440.0, 340.0);
            self.ball_vel = Vec2f::new(2.0, 2.0);
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
            p1_score: self.p1_score,
            p2_score: self.p2_score,
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
            p1_score: proto.p1_score,
            p2_score: proto.p2_score,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

