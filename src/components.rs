pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub fn dist(&self) -> f32 {
        self.dist_from(0.0, 0.0)
    }

    pub fn dist_from(&self, x: f32, y: f32) -> f32 {
        ((x - self.x).powf(2.0) + (y - self.y).powf(2.0)).sqrt()
    }

    pub fn dist_from_point(&self, point: &Point<f32>) -> f32 {
        self.dist_from(point.x, point.y)
    }

    pub fn theta(&self) -> f32 {
        (self.y / self.x).atan()
    }
}


pub struct Player;
pub struct Ball;
pub struct Ground;
pub struct Net;
pub struct Placar;
pub struct Game;