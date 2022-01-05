mod point;
mod player;
mod components;
use components::Point;

fn main() {
    let p = Point{x: 1.5, y: 0.5};
    println!("Hello, from the distance {} of the axis (0, 0)!", p.dist());
}
