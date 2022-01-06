use raylib::prelude::*;
use raylib::misc::AsF32;
use crate::player::Player;
use crate::point::*;


pub struct Ball {
    pub rect: Rectangle,
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub collider: Vector3, 
    pub speed: Vector2,
    pub aceleration: Vector2,
    pub last_touched_player: Option<Player>,
}


impl Point for Ball {
    fn x(&self) -> f32 { self.rect.x }
    fn y(&self) -> f32 { self.rect.y }
}


impl MovablePoint for Ball {
    fn x_ref(&mut self) -> &mut f32 { &mut self.rect.x }
    fn y_ref(&mut self) -> &mut f32 { &mut self.rect.y }
    fn speed_ref(&mut self) -> &mut Vector2 { &mut self.speed }
    fn aceleration_ref(&mut self) -> &mut Vector2 { &mut self.aceleration }
}


impl Ball {
    pub fn from<T1: AsF32, T2: AsF32>(color: Color, width: T1, height: T2) -> Self {
        Ball {
            rect: rrect(0, 0, width, height),
            color: color,
            width: width.as_f32(),
            height: height.as_f32(),
            collider: rvec3(0, 0, 0),
            speed: rvec2(0, 0),
            aceleration: rvec2(0, 0),
            last_touched_player: None,
        }
    }
}