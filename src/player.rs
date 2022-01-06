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
    pub aceleration: Vector2,
}


impl Point for Player {
    fn x(&self) -> f32 { self.rect.x }
    fn y(&self) -> f32 { self.rect.y }
}


impl MovablePoint for Player {
    fn x_ref(&mut self) -> &mut f32 { &mut self.rect.x }
    fn y_ref(&mut self) -> &mut f32 { &mut self.rect.y }
    fn speed_ref(&mut self) -> &mut Vector2 { &mut self.speed }
    fn aceleration_ref(&mut self) -> &mut Vector2 { &mut self.aceleration }
}


impl Player {
    pub fn from<T1: AsF32, T2: AsF32>(color: Color, side: Side, width: T1, height: T2) -> Self {
        Player {
            rect: rrect(0, 0, width, height),
            color: color,
            side: side,
            width: width.as_f32(),
            height: height.as_f32(),
            collider: rvec3(0, 0, 0),
            speed: rvec2(0, 0),
            aceleration: rvec2(0, 0),
        }
    }
}
