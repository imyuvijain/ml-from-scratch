use rand::random_range;
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::network::Network;
use rayon::prelude::*;

const SAVE_PATH: &str = "models.json";


//STRUCTS

struct Hand {
    cards: Vec<u32>,
    cards_list: [u32; 13]
}

impl Hand {
    fn create() -> Self {
        let cards_list: [u32; 13] = [11, 10, 10, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2];
        Hand { cards: vec![], cards_list }
    }

    fn new_card(&mut self) -> u32 {
        let index: usize = random_range(0..self.cards_list.len());
        let new: u32 = self.cards_list[index];
        self.cards.push(new);
        new
    }

    fn total(&self) -> (u32, bool) {
        let mut a_count = 0;
        let mut is_soft: bool = false;
        let mut sum = 0;

        for card in &self.cards {
            if *card == 11 {
                a_count += 1;
            } else {
                sum += card;
            }
        }

        for _ in 0..a_count {
            if (sum + 11) > 21 {
                sum += 1;
            } else {
                sum += 11;
                is_soft = true;
            }
        }

        (sum, is_soft)
    }

    fn display(&self, name: &str) {
        print!("{name}'s Hand: ");
        for card in &self.cards {
            print!("{card} ");
        }
        println!("Value: {:?}\n", self.total().0);
    }
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    generations: u32,
    models: Vec<Game>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Game {
    nn: Network,
    loss_rate: u32,
    iterations: u32,
}

impl Game {

    fn create() -> Self {
        let nn = Network::create(&[4, 8, 8, 1]);
        let iterations = 5000;
        Game { nn, loss_rate: iterations, iterations }
    }

    fn play(&self, console_output: bool) -> bool {
        let mut player = Hand::create();
        let mut dealer = Hand::create();

        dealer.new_card();

        if console_output {dealer.display("Dealer")}

        loop {

            if player_should_hit(&player, &dealer, &self.nn) {

                player.new_card();

                if console_output {player.display("Player")};

                if player.total().0 >= 21 {
                    break;
                }
            } else {
                break;
            }
        }
        
        if console_output {println!("===")}

        loop {
            if dealer.total().0 < 17 {
                dealer.new_card();
            } else {
                break;
            }
            if console_output {dealer.display("Dealer")}
        }

        let dealer_value = dealer.total().0;
        let player_value = player.total().0;

        if player_value > 21 || (player_value < dealer_value && dealer_value <= 21) {
            
            if console_output {println!("Player has lost")}
            false
        } else {
            if console_output {println!("Player has won!")}
            true
        }
    }

    fn run_simulation(&mut self) -> u32 {
        let mut losses = 0;
        
        for _ in 0..self.iterations {
            let game_not_lost = self.play(false);

            if !game_not_lost {losses+=1};

        }

        self.loss_rate = losses;
        losses
    }
}

macro_rules! input {
    ($prompt:expr) => {{
        print!("{}", $prompt);
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }};
    () => {{
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }};
}

fn player_should_hit(player: &Hand, dealer: &Hand, network: &Network) -> bool {
    let (player_total, is_soft_dealer) = player.total();
    let (dealer_total, is_soft) = dealer.total();


    let scaled_player_total = (player_total as f32) / 21.0;
    let is_soft_int = is_soft as u32 as f32;
    let scaled_dealer_total = (dealer_total as f32) / 11.0;
    let is_soft_dealer_int = is_soft_dealer as u32 as f32;

    let output = network.calculate(&vec![scaled_player_total, is_soft_int, scaled_dealer_total, is_soft_dealer_int]);

    output[0] > 0.0
}

pub fn run() {
    let count = 1000;
    let top_threshold = 10;

    let mut save_data: SaveData = if Path::new(SAVE_PATH).exists() {
        let json = fs::read_to_string(SAVE_PATH).unwrap();
        serde_json::from_str(&json).unwrap()
    } else {
        SaveData { generations: 0, models: (0..count).map(|_| Game::create()).collect() }
    };

    loop {
        let inp = input!("Input: ");

        match inp.as_str() {
            //QUIT
            "q" => {
                let json = serde_json::to_string_pretty(&save_data).unwrap();
                fs::write(SAVE_PATH, json).unwrap();
                println!("Saved {} models to {} ({} total generations)", save_data.models.len(), SAVE_PATH, save_data.generations);
                break;
            }
            //LEARNING MODE
            "l" => {
                let model_inp = input!("Enter number of simulations: ");

                let simulations: u32 = match model_inp.as_str() {
                    "1" => 1, "5" => 5, "10" => 10, "50" => 50,
                    "100" => 100, "500" => 500, "1000" => 1000,
                    _ => 1
                };

                for i in 0..simulations {

                    save_data.generations += 1;

                    println!("\n{i}th gen\n");

                    save_data.models.par_iter_mut().for_each(|model| {
                        model.run_simulation();
                    });

                    save_data.models.sort_by(|a, b| a.loss_rate.cmp(&b.loss_rate));
                    let best: Vec<_> = save_data.models[0..top_threshold].to_vec();

                    for model in &best {
                        println!("Loss rate: {}%", (100.0*model.loss_rate as f32/model.iterations as f32));
                    }

                    save_data.models = (0..count - top_threshold).map(|i| {
                        let mut m = best[i % top_threshold].clone();
                        m.nn._mutate();
                        m
                    }).collect();

                    save_data.models.extend(best);
                }
            }
            // TESTING MODE
            "t" => {
                let model_inp = input!("Enter model letter: ");

                let index: usize = match model_inp.as_str() {
                    "Z" => save_data.models.len() - 1,
                    _ => 0,
                };

                save_data.models[index].play(true);
            }
            _ => {}
        }
    }
}