mod point;
mod player;
mod ball;
mod components;
mod camera;
mod net;

use components::GameWindow;

fn main() {
    use raylib::prelude::Color;
    let mut game = GameWindow::from("Rusty Block Volley", 720, 480, 60);
    game.run_test_window(Color::GRAY);
}
