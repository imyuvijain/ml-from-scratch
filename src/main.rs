mod network;
// mod blackjack;
use macroquad::prelude::*;
mod flappybird;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        window_width: flappybird::WIDTH as i32,
        window_height: flappybird::HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    flappybird::run().await;

}


// fn main() {
//     blackjack::run();
// }