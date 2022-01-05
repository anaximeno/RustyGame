mod point;
mod player;
mod ball;
mod components;

use components::*;
use raylib::prelude::*;


fn main() {
    let p1 = Player::from(Color::BLUE, Side::LEFT, 32, 60, 200, 200);
    println!("Hello from p1 at {} pixels of distance from the origin!", p1.dist());
    println!("p1's height = {}, width = {} and, aceleration = {}", p1.height, p1.width, p1.aceleration);
}
