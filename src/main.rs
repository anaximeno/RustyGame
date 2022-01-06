mod point;
mod player;
mod ball;
mod components;
mod camera;
mod net;

use components::*;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 720;
const WINDOW_HEIGHT: i32 = 640;


fn main() {
    let (mut rl, th) = raylib::init().size(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .title("Rusty Volley").build();
    rl.set_target_fps(60);
    run_test_window(&mut rl, &th);
}


fn run_test_window(rl: &mut RaylibHandle, th: &RaylibThread) {
    let (w, h) = (WINDOW_WIDTH, WINDOW_HEIGHT);
    let player1 = Player::from(Color::BLUE, Side::LEFT, 32, 60, 0, h - 42);
    let player2 = Player::from(Color::RED, Side::RIGHT, 32, 60, w - 32, h - 42);
    let ball = Ball::from(Color::GOLD, 15, 15, w / 2, h / 2);
    let net = Net::from(Color::RAYWHITE, 4, 100, w / 2, h - 100);
    let camera = GameCamera::from(rvec2(0, 0), rvec2(0, 0));
    while !rl.window_should_close() {
        let mut dw = rl.begin_drawing(&th);
        dw.clear_background(Color::DARKGRAY);
        {
            let mut cw = dw.begin_mode2D(camera.cam);
            cw.draw_rectangle_rec(&player1.rect, player1.color);
            cw.draw_rectangle_rec(&player2.rect, player2.color);
            cw.draw_rectangle_rec(&net.rect, net.color);
            cw.draw_rectangle_rec(&ball.rect, ball.color);
        }
    }
}
