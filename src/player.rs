use raylib::prelude::*;
use crate::point::Point;

pub enum Side {
    LEFT,
    RIGHT
}


pub struct Player {
    pos: Point<f32>,
    rect: Rectangle,
    color: Color,
    side: Side,
    width: i32,
    height: i32,
    collider: Vector3, 
    speed: Vector2,
    aceleration: f32,
}

