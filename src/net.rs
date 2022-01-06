use raylib::prelude::*;
use raylib::misc::AsF32;
use crate::point::Point;


pub struct Net {
    pub rect: Rectangle,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub collider: Vector3,
}


impl Point for Net {
    fn x(&self) -> f32 { self.rect.x }
    fn y(&self) -> f32 { self.rect.y }
}


impl Net {
    pub fn from<T1: AsF32, U2: AsF32>(
        color: Color,
        width: T1,
        height: T1,
        x: U2,
        y: U2
    ) -> Self {
        Net {
            rect: rrect(x, y, width, height),
            collider: rvec3(0, 0, 0),
            color: color,
            width: width.as_f32(),
            height: height.as_f32(),
        }
    }
}
