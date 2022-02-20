use raylib::prelude::*;
use raylib::misc::AsF32;

pub use crate::point::*;
pub use crate::player::*;
pub use crate::ball::*;
pub use crate::camera::*;
pub use crate::net::*;

// pub struct Ground;
// pub struct Placar;
// pub struct Game;


struct Players(Player, Player);

struct Components {
    players: Players,
    ball: Ball,
    camera: GameCamera,
    net: Net
}

pub struct GameWindow {
    pub name: String,
    pub width: i32,
    pub height: i32,
    fps: u32,
    rl: RaylibHandle,
    thd: RaylibThread,
    components: Components
}


impl Players {
    fn set_speed_to<T1: AsF32, T2: AsF32>(&mut self, x: T1, y: T2) {
        self.0.set_speed_to(x, y);
        self.1.set_speed_to(x, y);
    }
}


impl GameWindow {
    pub fn from(name: &str, width: i32, height: i32, fps: u32) -> Self {
        let (rl, thd) = raylib::init()
            .size(width, height)
            .title(name)
            .build();

        let player1 = Player::from(
            Color::BLUE,
            Side::LEFT,
            32_i32,
            60_i32
        );

        let player2 = Player::from(
            Color::RED,
            Side::RIGHT,
            32_i32,
            60_i32
        );

        let ball = Ball::from(
            Color::GOLD,
            15_i32,
            15_i32
        );

        let net = Net::from(
            Color::RAYWHITE,
            4_i32,
            100_i32,
            width.as_f32() / 2.0,
            height.as_f32() - 100.0
        );
            
        let players = Players(player1, player2);
        let camera = GameCamera::new();
        let name = String::from(name);


        GameWindow {
            name:       name,
            width:      width,
            height:     height,
            fps:        fps,
            rl:         rl,
            thd:        thd,
            components: Components {
                players:    players,
                ball:       ball,
                camera:     camera,
                net:        net
            }
        }
    }

    pub fn run_test_window(&mut self, background_color: Color) {
        self.rl.set_target_fps(self.fps);
        let (width, height) = (self.width.as_f32(), self.height.as_f32());

        self.components.players.set_speed_to(2, 2);

        self.components.players.0.move_to(
            0_i32, height - self.components.players.0.height
        );

        self.components.players.1.move_to(
            width  - self.components.players.1.width,
            height - self.components.players.1.height
        );

        self.components.ball.move_to(width / 2.0, height / 2.0);
        self.components.ball.walk(50_i32, 0_i32);
    
        while !self.rl.window_should_close() {
            self.handle_keyboard_input();

            let mut drawer = self.rl.begin_drawing(&self.thd);
            drawer.clear_background(background_color);

            { let mut mode = drawer.begin_mode2D(self.components.camera.cam);
                mode.draw_rectangle_rec(
                    &self.components.players.0.rect, self.components.players.0.color
                );
                mode.draw_rectangle_rec(
                    &self.components.players.1.rect, self.components.players.1.color
                );
                mode.draw_rectangle_rec(
                    &self.components.net.rect,       self.components.net.color
                );
                mode.draw_rectangle_rec(
                    &self.components.ball.rect,      self.components.ball.color
                );
            }
        }
    }


    fn handle_keyboard_input(&mut self) {
        use raylib::consts::KeyboardKey::*;

        let player = &mut self.components.players.0;
        let camera = &mut self.components.camera;

        if self.rl.is_key_down(KEY_LEFT) {
            player.walk(-player.speed.x, 0);
            camera.increase_offset_by(player.speed.x / 2.0, 0);
        }

        if self.rl.is_key_down(KEY_RIGHT) {
            player.walk(player.speed.x, 0);
            camera.increase_offset_by(-player.speed.x / 2.0, 0);
        }

        if self.rl.is_key_down(KEY_UP) {
            player.walk(0, -player.speed.y);
            camera.increase_offset_by(0, player.speed.y / 2.0);
        }

        if self.rl.is_key_down(KEY_DOWN) {
            player.walk(0, player.speed.y);
            camera.increase_offset_by(0, -player.speed.y / 2.0);
        }
    }
}

