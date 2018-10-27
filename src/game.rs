use ggez::nalgebra;

type Point2f = nalgebra::Point2<f32>;
type Vec2f = nalgebra::Vector2<f32>;

const GAME_WIDTH: f32 = 300.0;
const GAME_HEIGHT: f32 = 300.0;

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
    }
}
