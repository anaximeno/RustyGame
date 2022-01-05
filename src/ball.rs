use raylib::prelude::*;
use raylib::misc::AsF32;
use crate::point::Point;


pub struct Ball {
    pub rect: Rectangle,
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub collider: Vector3, 
    pub speed: Vector2,
    pub aceleration: f32,
}


impl Point for Ball {
    fn x(&self) -> f32 { self.rect.x }
    fn y(&self) -> f32 { self.rect.y }
}


impl Ball {
    pub fn from<T: AsF32, U: AsF32>(
        color: Color,
        w: T,
        h: T,
        x: U,
        y: U
    ) -> Ball {
        Ball {
            rect: rrect(x, y, w, h),
            color: color,
            width: w.as_f32(),
            height: h.as_f32(),
            collider: rvec3(0, 0, 0),
            speed: rvec2(0, 0),
            aceleration: 0.0
        }
    }
}