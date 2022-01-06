mod point;
mod player;
mod ball;
mod components;
mod camera;
mod net;

use components::*;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 720;
const WINDOW_HEIGHT: i32 = 480;


fn main() {
    let (mut rl, th) = raylib::init().size(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .title("Rusty Volley").build();
    rl.set_target_fps(60);
    run_test_window(&mut rl, &th);
}


fn run_test_window(rl: &mut RaylibHandle, th: &RaylibThread) {
    let (w, h) = (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let mut player1 = Player::from(Color::BLUE, Side::LEFT, 32, 60);
    let mut player2 = Player::from(Color::RED, Side::RIGHT, 32, 60);
    let mut ball = Ball::from(Color::GOLD, 15, 15);
    let mut camera = GameCamera::from(rvec2(0, 0), rvec2(0, 0));
    let net = Net::from(Color::RAYWHITE, 4, 100, w / 2.0, h - 100.0);

    player1.move_to(0, h - player1.height);
    player2.move_to(w - player2.width, h - player2.height);
    ball.move_to(w / 2.0, h / 2.0);
    ball.walk(50, 0);

    while !rl.window_should_close() {
        handle_keyboard_input(&rl, &mut player1, &mut camera);
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


fn handle_keyboard_input(rl: &RaylibHandle, player: &mut Player, camera: &mut GameCamera) {
    use raylib::consts::KeyboardKey::*;
    const SPEED: f32 = 2.0;
    const CAM_OFF_SPEED: f32 = SPEED / 2.0;
    // TODO: check/improve camera traits
    if rl.is_key_down(KEY_LEFT) {
        player.walk(-SPEED, 0);
        let new_offset = camera.cam.offset.x+CAM_OFF_SPEED;
        camera.change_offset_to(new_offset, 0);
    }
    if rl.is_key_down(KEY_RIGHT) {
        player.walk(SPEED, 0);
        let new_offset = camera.cam.offset.x-CAM_OFF_SPEED;
        camera.change_offset_to(new_offset, 0);
    }
}