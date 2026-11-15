use macroquad::prelude::*;
use nn::scalar::{Value, tape_len, truncate_tape, zero_grads};

pub const HEIGHT: f32 = 800.0;
pub const WIDTH: f32 = 1200.0;
pub const X_SCALE: f32 = 100.0;
pub const Y_SCALE: f32 = 100.0;

const X_NORM: f32 = 5.0;
const LEARNING_RATE: f32 = 0.05;
const STEPS_PER_FRAME: usize = 10;

// STRUCTS

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn draw(&self) {
        plot(self.x, self.y, BLACK)
    }
}

struct Curve {
    coeffs: Vec<Value>,
    n_params: usize,
    thickness: f32,
}

impl Curve {
    fn create(coeffs: Vec<Value>, n_params: usize) -> Self {

        Curve {
            coeffs,
            n_params,
            thickness: 3.0,
        }
    }

    fn f(&self, x: f32) -> Value {
        let u = x / X_NORM;
        let data = self.coeffs
            .iter()
            .enumerate()
            .map(|(i, c)| c.data() * u.powi(i as i32))
            .sum();
        Value::new(data)
    }

    fn draw(&self) {
        let mut prev: Option<(f32, f32)> = None;
        for sx in 0..(WIDTH as i32) {
            let x = (sx as f32 - WIDTH / 2.0) / X_SCALE;
            let (px, py) = to_screen(x, self.f(x).data());
            if let Some((lx, ly)) = prev {
                draw_line(lx, ly, px, py, self.thickness, RED);
            }
            prev = Some((px, py));
        }
    }

    fn update(&mut self, target: &[Point]) -> f32 {
        let mut loss = Value::new(0.0);

        // Calculate error
        for point in target {
            let u = point.x / X_NORM;
            let mut prediction = Value::new(0.0);
            for (i, c) in self.coeffs.iter().enumerate() {
                prediction = prediction + *c * (u.powi(i as i32));
            }
            let error = prediction + -point.y;
            loss = loss + error.pow(2.0) * 0.5;
        }

        // Calculate gradients
        loss.backwards();

        // Update values
        for c in self.coeffs.iter_mut() {
            c.set_data(c.data() - (c.grad() * LEARNING_RATE));
        }

        // Reset
        let loss_value = loss.data();
        zero_grads();
        truncate_tape(self.n_params as usize);

        loss_value * 1.0/((target.len().max(1)) as f32)
    }
}

// FUNCTIONS

fn to_screen(x: f32, y: f32) -> (f32, f32) {
    (x * X_SCALE + WIDTH / 2.0, HEIGHT / 2.0 - y * Y_SCALE)
}

fn plot(x: f32, y: f32, color: Color) {
    let (sx, sy) = to_screen(x, y);
    draw_circle(sx, sy, 4.0, color);
}

pub async fn run() {
    // f(x) = 0.025x^5 - 0.521x^3 + 1.936x
    let sample_points: Vec<Point> = vec![
        Point {
            x: -4.275,
            y: -3.268,
        },
        Point {
            x: -3.825,
            y: 1.282,
        },
        Point {
            x: -3.375,
            y: 2.548,
        },
        Point {
            x: -2.925,
            y: 2.023,
        },
        Point {
            x: -2.475,
            y: 0.785,
        },
        Point {
            x: -2.025,
            y: -0.445,
        },
        Point {
            x: -1.575,
            y: -1.256,
        },
        Point {
            x: -1.125,
            y: -1.481,
        },
        Point {
            x: -0.675,
            y: -1.150,
        },
        Point {
            x: -0.225,
            y: -0.430,
        },
        Point { x: 0.225, y: 0.430 },
        Point { x: 0.675, y: 1.150 },
        Point { x: 1.125, y: 1.481 },
        Point { x: 1.575, y: 1.256 },
        Point { x: 2.025, y: 0.445 },
        Point {
            x: 2.475,
            y: -0.785,
        },
        Point {
            x: 2.925,
            y: -2.023,
        },
        Point {
            x: 3.375,
            y: -2.548,
        },
        Point {
            x: 3.825,
            y: -1.282,
        },
        Point { x: 4.275, y: 3.268 },
    ];

    let coeffs: Vec<Value> = (0..6).map(|_| Value::new(0.0)).collect();
    let n_params = tape_len();
    let mut model_curve = Curve::create(coeffs, n_params);
    let mut epoch: usize = 0;
    let mut loss = 0.0;

    loop {
        clear_background(WHITE);

        for _ in 0..STEPS_PER_FRAME {
            loss = model_curve.update(&sample_points);
            epoch += 1;
        }

        model_curve.draw();
        for point in &sample_points {
            point.draw();
        }

        draw_text(
            &format!("epoch {epoch}   loss {loss:.6}"),
            10.0,
            30.0,
            30.0,
            BLACK,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Back Prop".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    run().await;
}
