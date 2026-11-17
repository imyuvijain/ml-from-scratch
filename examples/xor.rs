
use nn::{scalar::{Value, nn::Mlp, tape_len, truncate_tape, zero_grads}};

//outputs "+" for 1 and "-" for -1
fn get_sign(n: f32) -> &'static str {
    if n >= 0.0 {"+"} else {"-"}
}

pub fn main() {

    let inputs = [
        [0.0,0.0],
        [0.0,1.0],
        [1.0,0.0],
        [1.0,1.0],
    ];

    let targets = [-1.0, 1.0, 1.0, -1.0];

    let model = Mlp::new(&[2,4,1]);
    let params = model.parameters();

    let n_params = tape_len();

    let learning_rate = 0.05;


    let iterations = 2000;
    for step in 0..=iterations {
        let mut loss = Value::new(0.0);
        
        for (x, t) in inputs.iter().zip(targets.iter()) {
            
            let x_vals: Vec<Value> = x.iter().map(|xi| Value::new(*xi)).collect();
            
            let pred = model.forward(x_vals)[0]; //model is 2,4,1 so 1 output
            let err = pred + Value::new(-*t);

            loss = loss + err*err;

        }
        loss.backwards();

        for p in &params {
            p.update(learning_rate);
        }        

        let new_loss = loss.data();

        zero_grads();
        truncate_tape(n_params);

        if step%100==0 {
            println!("Loss at step {step}: {new_loss}.");

            if step==iterations {
                println!("Test ended with final outputs");
            }

            for input in inputs {
                let input_values: Vec<Value> = input.iter().map(|i| Value::new(*i)).collect();

                let pred = model.forward(input_values)[0].data();
                
                if step == iterations {
                    println!("{}", pred);
                } else {
                    println!("{}", get_sign(pred));
                }
                
            }
        }



    }

}