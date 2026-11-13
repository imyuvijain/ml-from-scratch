use macroquad::prelude::*;

pub const HEIGHT: f32 = 800.0;
pub const WIDTH: f32 = 1200.0;

struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    vx: f32,
    vy: f32,
}

impl Ball {
    fn create() -> Ball {
        Ball {x: 100.0, y: 100.0, radius: 10.0, vx: 10.0, vy: 10.0}
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, WHITE);
    }

    fn move_ball(&mut self) {

        if (self.x+self.vx) > WIDTH-self.radius || (self.x+self.vx) <= self.radius {
            self.vx = -self.vx;
        }
        if (self.y+self.vy) > HEIGHT-self.radius || (self.y+self.vy) <= self.radius {
            self.vy = -self.vy;
        }

        self.x += self.vx;
        self.y += self.vy;

        

    }

}


struct Paddle {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    vy: f32,
    ay: f32, //acceleration
}

impl Paddle {
    fn create() -> Self {
        Paddle {width: 20.0, height: 100.0, x: WIDTH-50.0, y: HEIGHT/2.0, vy: 0.0, ay: 0.3}
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }

    fn move_paddle(&mut self) {
        self.vy += self.ay;
        self.y += self.vy;

        if self.y <= 0.0 {
            self.y = 0.0;           // clamp to boundary
            self.vy = self.vy.abs(); // force moving downward
            self.ay = self.ay.abs();
            
        } else if self.y >= HEIGHT - self.height {
            
            self.y = HEIGHT - self.height; // clamp
            self.vy = -self.vy.abs();      // force moving upward
            self.ay = -self.ay.abs();
        }
    }

    fn toggle(&mut self) {
        self.vy = 0.0;
        self.ay = -self.ay;
    }

}

pub async fn run() {

    let mut ball = Ball::create();

    let mut paddle = Paddle::create();

    loop {
        clear_background(BLACK);

        ball.draw();    
        ball.move_ball();

        paddle.draw();
        paddle.move_paddle();

        if is_key_pressed(KeyCode::Enter) {
            paddle.toggle();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }


}