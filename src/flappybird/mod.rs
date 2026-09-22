use macroquad::prelude::*;
use ::rand::random_range;
use crate::network::{Network};
use std::cmp::max;

//CONSTANTS
const GRAVITY: f32 = 0.4;
pub const HEIGHT: f32 = 800.0;
pub const WIDTH: f32 = 1200.0;


//STRUCTS
#[derive(Debug)]
struct Bird {
    x: f32,
    y: f32,
    vy: f32,
    radius: f32,
    survival_frames: u32,
    score: u32,
    nn: Network,
    is_alive: bool,
}

struct Pipe {
    height: f32,
    width: f32,
    gap: f32,
    x: f32,
}

//IMPLEMENATIONS
impl Bird {
    fn create() -> Self {
        let x = WIDTH/3.0;
        let y = 100.0;
        let vy = 5.0;
        let radius = 30.0;
        let survival_frames: u32 = 0;
        let score = 0;
        let nn = Network::create(&[5,10,1]);
        let is_alive = true;

        Bird {x, y, vy, radius, survival_frames, score, nn, is_alive}
    }

    fn create_with_network(nn: Network) -> Self {
        Bird {
            x: WIDTH / 3.0,
            y: 100.0,
            vy: 5.0,
            radius: 30.0,
            survival_frames: 0,
            score: 0,
            is_alive: true,
            nn,
        }
    }

    fn draw(&self, index: usize) {

        let n = (index) as u8;

        draw_circle(self.x, self.y, self.radius, Color::from_rgba(n, n, 255, 150));
    }

    fn move_bird(&mut self, dt: f32) {
        self.y += self.vy * dt * 60.0;
        self.vy += GRAVITY*dt*60.0;
        self.survival_frames += 1;
    }

    fn jump(&mut self) {
        self.vy = -8.0;
    }

    fn detect_collision(&self, pipe: &Pipe) -> bool {

        let out_of_bounds = (self.y + self.radius > HEIGHT) || (self.y - self.radius < 0.0);
        let in_pipe_x = (self.x + self.radius > pipe.x) && (self.x - self.radius < pipe.x + pipe.width);

        let gap_top = pipe.height;
        let gap_bottom = pipe.height + pipe.gap;
        let in_pipe_y = (self.y - self.radius < gap_top) || (self.y + self.radius > gap_bottom);

        out_of_bounds || (in_pipe_x && in_pipe_y)

    }

    fn get_inputs(&self, pipe: &Pipe) -> Vec<f32> {
        
        let bird_y = self.y / HEIGHT;
        let bird_vy = (self.vy + 8.0) / 35.0;

        let distance = (pipe.x - self.x).clamp(0.0, WIDTH)/WIDTH;

        let pipe_height = pipe.height/HEIGHT;
        let y_distance = bird_y - pipe_height;

        vec![bird_y, bird_vy, distance, pipe_height, y_distance]
    }


}

impl Pipe {
    fn create(height: f32) -> Self {
        let pipe_width = 100.0;
        Pipe {height: height, width: 80.0, gap: 200.0, x: WIDTH-pipe_width}
    }

    fn draw(&self) {

        draw_rectangle(self.x, 0.0, self.width, self.height, RED);

        let second_height = self.height+self.gap;
        draw_rectangle(self.x, second_height, self.width, HEIGHT-second_height, RED);

    }

    fn move_pipe(&mut self, dt: f32) -> bool{
        let speed: f32 = 10.0*dt*60.0;
        
        let mut passed = false;

        self.x = {
            if self.x <= speed {
                self.height = random_range((self.gap+50.0)..(HEIGHT - self.gap - 100.0));
                
                passed = true;
                WIDTH+speed

            } else {
                self.x - speed
            }
        };

        passed

    }

}



pub async fn run() {

    let bot_mode = true;

    if bot_mode {
        let count = 1000;

        let mut birds: Vec<Bird> = (0..count).map(|_| Bird::create()).collect::<Vec<_>>();

        let mut alive_count = count;

        let mut evolution_count = 0;

        let mut score = 0;

        'outer: loop {

            let mut pipe = Pipe::create(HEIGHT/2.0);
            
            

            loop {
                // clear_background(SKYBLUE);
                clear_background(BLACK);
                draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 40.0, WHITE);
                draw_text(&format!("Birds: {} / {}", alive_count, count), 10.0, 80.0, 40.0, WHITE);
                draw_text(&format!("Evolution {}", evolution_count), 10.0, 120.0, 40.0, WHITE);
                draw_text(&format!("Score {}", score), 10.0, 160.0, 40.0, WHITE);

                if alive_count <= 0 {
                    score = 0;
                    alive_count = count;
                    evolution_count += 1;

                    birds.sort_by(|a, b| b.survival_frames.cmp(&a.survival_frames));

                    let elite = &birds[..20];

                    // rank-based weights: best bird gets 20, next gets 19, ..., last gets 1
                    let total_weight: usize = (1..=20).sum(); // 210

                    let mut new_networks: Vec<Network> = Vec::new();

                    for (i, bird) in elite.iter().enumerate() {
                        let rank_weight = 20 - i; // 20 for best, 1 for worst
                        let offspring = (rank_weight * count) / total_weight;

                        new_networks.push(bird.nn.clone()); // keep 1 original
                        for _ in 1..offspring {
                            let mut child = bird.nn.clone();
                            child.mutate();
                            new_networks.push(child);
                        }
                    }

                    // fill remaining slots (rounding loss) with mutated copies of the best bird
                    while new_networks.len() < count {
                        let mut child = elite[0].nn.clone();
                        child.mutate();
                        new_networks.push(child);
                    }

                    birds = new_networks
                        .into_iter()
                        .map(|nn| Bird::create_with_network(nn))
                        .collect();

                    break;

                }

                pipe.draw();
                let dt = get_frame_time();
                let pipe_passed = pipe.move_pipe(dt);

                if pipe_passed {
                    score += 1;
                }

                for (index, bird) in birds.iter_mut().enumerate() {
                    
                    if !bird.is_alive {
                        continue;
                    }
        
                    {
                        bird.draw(index);
            
                        
            
                        bird.move_bird(dt);
            
                        if pipe_passed { // returns whether the pipe passed
                            bird.score = score;
                        };
                    }
            
                    //reset
                    if bird.detect_collision(&pipe) {
                        alive_count -= 1;
                        bird.is_alive = false;
                    }
            
                    let inputs = bird.get_inputs(&pipe);
            
                    let nn_output = bird.nn.calculate(&inputs)[0];
            
                    if nn_output > 0.0 {
                        bird.jump();
                    };
        
                }
        
                if is_key_pressed(KeyCode::Enter) {
                    alive_count = 0;
                }
        
                if is_key_pressed(KeyCode::Escape) {
                    
                    birds.sort_by(|a, b| b.survival_frames.cmp(&a.survival_frames));
                    
                    for bird in &birds[0..2] {
                        dbg!(&bird.nn);
                    }

                    break 'outer;
                }
        
                next_frame().await
            }



        }// dbg!(birds);
    }    
    else {    let mut high_score = 0;
        
        'outer: loop {
        
            let mut score = 0;
        
            let mut pipe = Pipe::create(HEIGHT/2.0);
            
            let mut bird = Bird::create();
        
            loop {
                clear_background(SKYBLUE);
                draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 40.0, WHITE);
                draw_text(&format!("Score: {} Max Score: {}", score, high_score), 10.0, 50.0, 40.0, WHITE);
                
                
                pipe.draw();
                let dt = get_frame_time();
                if pipe.move_pipe(dt) {
                    score += 1;
                    high_score = max(score, high_score);
                }
            
                bird.draw(0);
                bird.move_bird(dt);
        
                if bird.detect_collision(&pipe) {
                    break;
                }
        
                if is_key_pressed(KeyCode::Space) {
                        bird.jump();
                    };
        
                if is_key_pressed(KeyCode::Escape) {
        
                    break 'outer;
                }
        
                next_frame().await
            }
        
        }
    }
}



// [src/flappybird/mod.rs:263:21] &bird.nn = Network {
//     layers: [
//         Layer {
//             weights: [
//                 [
//                     0.037371714,
//                     -0.34199256,
//                     -0.06287138,
//                     -0.1566994,
//                 ],
//                 [
//                     -0.2167017,
//                     0.008045696,
//                     0.43266773,
//                     0.121567786,
//                 ],
//                 [
//                     0.5261048,
//                     0.28316972,
//                     -0.29443458,
//                     0.008204989,
//                 ],
//                 [
//                     0.35710564,
//                     0.5237644,
//                     -0.2277393,
//                     -0.3162954,
//                 ],
//                 [
//                     0.5180902,
//                     -0.60808146,
//                     -0.32233688,
//                     -0.12611303,
//                 ],
//                 [
//                     -0.20878361,
//                     -0.26190263,
//                     -0.10807819,
//                     -0.06343655,
//                 ],
//                 [
//                     -0.16596648,
//                     -0.3141103,
//                     0.17618003,
//                     -0.15351179,
//                 ],
//                 [
//                     -0.1597423,
//                     0.66714424,
//                     0.110396296,
//                     0.099380136,
//                 ],
//                 [
//                     0.13115798,
//                     0.20318983,
//                     0.2551105,
//                     -0.20198177,
//                 ],
//                 [
//                     -0.004138928,
//                     0.58170813,
//                     0.23694146,
//                     0.22874637,
//                 ],
//                 [
//                     0.6580808,
//                     0.15783088,
//                     0.25210276,
//                     0.03163381,
//                 ],
//                 [
//                     -0.5071465,
//                     0.07801345,
//                     0.47321668,
//                     0.49827135,
//                 ],
//                 [
//                     -0.2930274,
//                     0.5041174,
//                     -0.2767416,
//                     0.2945428,
//                 ],
//                 [
//                     0.16486342,
//                     -0.24964927,
//                     -0.15528053,
//                     0.11700468,
//                 ],
//                 [
//                     0.20548691,
//                     0.0878008,
//                     -0.113927536,
//                     -0.07115465,
//                 ],
//                 [
//                     0.13970152,
//                     -0.1963773,
//                     -0.5526079,
//                     -0.0812255,
//                 ],
//             ],
//             biases: [
//                 0.09184291,
//                 -0.4145124,
//                 0.01841069,
//                 -0.0561792,
//                 -0.3946785,
//                 -0.09286977,
//                 0.1357233,
//                 -0.61959285,
//                 -0.25209948,
//                 0.4745141,
//                 -0.6087182,
//                 0.19295016,
//                 -0.5812674,
//                 -0.1809206,
//                 -0.3655286,
//                 -0.28414193,
//             ],
//         },
//         Layer {
//             weights: [
//                 [
//                     0.4937936,
//                     -0.24959137,
//                     -0.35429385,
//                     -0.28716794,
//                     -0.008118458,
//                     -0.012603583,
//                     0.38733956,
//                     0.28545454,
//                     -0.101188965,
//                     0.25943708,
//                     0.3007258,
//                     0.11607986,
//                     -0.003480794,
//                     -0.26221493,
//                     0.24448234,
//                     0.088973306,
//                 ],
//                 [
//                     0.39722562,
//                     0.3373331,
//                     -0.3817391,
//                     -0.5990455,
//                     0.33332798,
//                     -0.22490402,
//                     -0.47878173,
//                     0.30319113,
//                     -0.033943523,
//                     -0.35821256,
//                     0.32064074,
//                     0.55398536,
//                     -0.3004925,
//                     0.40708864,
//                     -0.4544071,
//                     0.28292337,
//                 ],
//                 [
//                     -0.19960837,
//                     -0.09003111,
//                     -0.3509532,
//                     0.16988148,
//                     0.29615614,
//                     -0.08302736,
//                     0.17377976,
//                     -0.6093982,
//                     -0.09779893,
//                     0.15909454,
//                     -0.040276892,
//                     0.4249,
//                     -0.2822238,
//                     0.54400843,
//                     0.2746352,
//                     0.34028226,
//                 ],
//                 [
//                     0.21229626,
//                     -0.5992457,
//                     -0.5360826,
//                     0.2992794,
//                     0.46853465,
//                     -0.26734674,
//                     -0.28972852,
//                     -0.387314,
//                     0.16650595,
//                     0.22930317,
//                     0.27648175,
//                     -0.6502726,
//                     -0.5308949,
//                     0.202938,
//                     0.061963424,
//                     -0.143369,
//                 ],
//                 [
//                     -0.37479287,
//                     0.072364256,
//                     0.25478065,
//                     -0.29822677,
//                     0.4591054,
//                     -0.1870695,
//                     -0.5142673,
//                     -0.27384812,
//                     0.5792756,
//                     0.45848772,
//                     -0.49633405,
//                     -0.32270882,
//                     0.23378132,
//                     0.19251853,
//                     0.45489326,
//                     0.38836288,
//                 ],
//                 [
//                     -0.13716304,
//                     -0.21357122,
//                     0.059296947,
//                     0.028094858,
//                     0.037087746,
//                     0.4610248,
//                     -0.25278372,
//                     0.2237593,
//                     -0.21358535,
//                     -0.2770477,
//                     -0.50081974,
//                     0.10327878,
//                     0.32078505,
//                     -0.25620827,
//                     0.47265816,
//                     -0.1258745,
//                 ],
//                 [
//                     -0.25013044,
//                     0.29117993,
//                     0.07155347,
//                     0.47378176,
//                     0.45662388,
//                     0.036734357,
//                     0.06787391,
//                     -0.23790166,
//                     -0.17578948,
//                     0.47464776,
//                     -0.30149388,
//                     0.07270704,
//                     0.406201,
//                     0.30129564,
//                     0.08193852,
//                     0.2606883,
//                 ],
//                 [
//                     0.47099265,
//                     0.0064150877,
//                     0.23982356,
//                     -0.04535761,
//                     0.25699097,
//                     -0.15244544,
//                     0.4036657,
//                     -0.31521243,
//                     0.03946044,
//                     0.307488,
//                     0.22141394,
//                     0.29432917,
//                     0.60589874,
//                     -0.29110032,
//                     0.024806052,
//                     -0.4678156,
//                 ],
//             ],
//             biases: [
//                 -0.38317576,
//                 -0.06484091,
//                 -0.19639571,
//                 -0.7901745,
//                 -0.4129293,
//                 0.37545702,
//                 0.61328924,
//                 -0.21840233,
//             ],
//         },
//         Layer {
//             weights: [
//                 [
//                     0.06390774,
//                     -0.43901092,
//                     -0.051067427,
//                     0.4020104,
//                     0.16026853,
//                     -0.19113584,
//                     -0.35577935,
//                     -0.18271562,
//                 ],
//             ],
//             biases: [
//                 0.048330367,
//             ],
//         },
//     ],
// }


