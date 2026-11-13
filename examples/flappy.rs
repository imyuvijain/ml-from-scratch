use macroquad::prelude::*;
use nn::games::flappy;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        window_width: flappy::WIDTH as i32,
        window_height: flappy::HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    flappy::run().await;
}
