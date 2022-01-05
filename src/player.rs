use raylib::misc::AsF32;
use raylib::prelude::*;
use crate::point::Point;


pub enum Side {
    LEFT,
    RIGHT
}


pub struct Player {
    pub rect: Rectangle,
    pub color: Color,
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub collider: Vector3, 
    pub speed: Vector2,
    pub aceleration: f32,
}


impl Point for Player {
    fn x(&self) -> f32 { self.rect.x }
    fn y(&self) -> f32 { self.rect.y }
}


impl Player {
    pub fn from<T: AsF32, U: AsF32>(color: Color, side: Side, w: T, h: T, x: U, y: U) -> Player {
        Player {
            rect: rrect(x, y, w, h),
            color: color,
            side: side,
            width: w.as_f32(),
            height: h.as_f32(),
            collider: rvec3(0, 0, 0),
            speed: rvec2(0, 0),
            aceleration: 0.0
        }
    }
}

//  on Player and added new function create_player