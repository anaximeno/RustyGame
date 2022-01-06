use raylib::prelude::*;
use raylib::misc::AsF32;
use crate::point::*;


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


impl MovablePoint for Player {
    fn x_ref(&mut self) -> &mut f32 { &mut self.rect.x }
    fn y_ref(&mut self) -> &mut f32 { &mut self.rect.y }
}


impl Player {
    pub fn from<T1: AsF32, T2: AsF32>(
        color: Color,
        side: Side,
        width: T1,
        height: T1,
        x: T2,
        y: T2
    ) -> Self {
        Player {
            rect: rrect(x, y, width, height),
            color: color,
            side: side,
            width: width.as_f32(),
            height: height.as_f32(),
            collider: rvec3(0, 0, 0),
            speed: rvec2(0, 0),
            aceleration: 0.0
        }
    }
}
