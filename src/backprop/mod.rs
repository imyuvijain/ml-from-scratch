use macroquad::prelude::*;

pub const HEIGHT: f32 = 800.0;
pub const WIDTH: f32 = 1200.0;
pub const X_SCALE: f32 = 200.0;
pub const Y_SCALE: f32 = 5.0;

//2.2, -3.0, -7.4, 3.5, -5.4

struct Curve {
    coeffs: Vec<f32>,
    thickness: f32,
}

impl Curve {
    fn create(coeffs: Vec<f32>, color: Color) -> Self {
        Curve { coeffs: coeffs, thickness: 5.0 }
    }

    fn draw(&self) {
        for x in 0..(WIDTH as i32) {
            let y = self.f(((x as f32) - WIDTH / 2.0) / X_SCALE);
            draw_rectangle(x as f32, y, 1.0, 1.0, RED);
        }
    }

    fn f(&self, x: f32) -> f32 {
        let mut value: f32 = 0.0;

        for (i, coeff) in self.coeffs.iter().enumerate() {
            value += coeff * x.powi(i as i32);
        }
        HEIGHT / 2.0 - value * Y_SCALE
    }

    fn update(&self, target: Vec<f32>) {
        let mut loss = 0.0;
        for (i, coeff) in self.coeffs.iter().enumerate() {
            //NEED TO WORK HERE
            //NEED TO ADD A FEW SAMPLES (points) FROM THE CURVE AND USE THAT AS AN INPUT INSTEAD
            
        }
    }
}

pub async fn run() {
    println!("Running");

    let target_curve = Curve::create(vec![-37.0, -44.0, -35.5, -4.5, 12.0], RED);
    let model_curve = Curve::create(vec![0.0, 0.0, 0.0, 0.0, 0.0],BLUE);

    loop {
        clear_background(BLACK);

        target_curve.draw();
        model_curve.draw();

        model_curve.update();


        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}