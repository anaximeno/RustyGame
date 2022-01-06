use raylib::prelude::*;
use raylib::misc::AsF32;
use crate::point::Point;
use crate::player::Player;


pub struct Ball {
    pub rect: Rectangle,
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub collider: Vector3, 
    pub speed: Vector2,
    pub aceleration: f32,
    pub last_touched_player: Option<Player>,
}


impl Point for Ball {
    fn x(&self) -> f32 { self.rect.x }
    fn y(&self) -> f32 { self.rect.y }
}


impl Ball {
    pub fn from<T1: AsF32, U2: AsF32>(
        color: Color,
        width: T1,
        height: T1,
        x: U2,
        y: U2
    ) -> Self {
        Ball {
            rect: rrect(x, y, width, height),
            color: color,
            width: width.as_f32(),
            height: height.as_f32(),
            collider: rvec3(0, 0, 0),
            speed: rvec2(0, 0),
            aceleration: 0.0,
            last_touched_player: None,
        }
    }
}