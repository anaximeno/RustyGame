mod point;
mod player;
mod ball;
mod components;
mod camera;
mod net;

use components::*;
use raylib::prelude::*;


fn main() {
    let p1 = Player::from(Color::BLUE, Side::LEFT, 32, 60, 200, 200);

    println!(
        "Player 1 says hello from {} pixels of distance from the origin!",
        p1.dist()
    );

    println!(
        "p1's height = {}, width = {} and, aceleration = {}",
        p1.height, p1.width, p1.aceleration
    );

    let b = Ball::from(Color::BEIGE, 15, 15, 369, -125);

    println!(
        "\nThere is a ball at the distance of {} pixels from the Player 1!",
        b.dist_from_point(&p1)
    );

    let mut camera = GameCamera::from(rvec2(0, 0), rvec2(200, 345));

    camera.rotate_to(b.theta());
    println!("\nCamera rotation = {}", camera.cam.rotation);

    camera.restore_rotation();
    println!("After restore, camera rotation = {}", camera.cam.rotation);

    let net = Net::from(Color::RAYWHITE, 5, 80, 250, 250);

    println!(
        "\nNet is on ({}, {}) at {} pixels of distance from Player1!",
        net.x(), net.y(), net.dist_from_point(&p1)
    );
}
