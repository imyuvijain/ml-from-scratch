use macroquad::prelude::*;
use nn::games::pong;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_string(),
        window_width: pong::WIDTH as i32,
        window_height: pong::HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    pong::run().await;
}
