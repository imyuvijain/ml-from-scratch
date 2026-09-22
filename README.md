# Machine Learning from Scratch

I'm writing a small machine learning library in Rust without using any ML crates. No `tch`, no `candle`, no `ndarray`. Everything is built on `Vec<f32>` and math I work out myself.

I don't plan on publishing this anywhere. It exists so I can learn Rust and machine learning at the same time, which means I write the networks, the training loops and the games they play. I write all of it by hand and I don't use AI for any of it. There's more about that further down.

## What it does right now

It plays Flappy Bird. A thousand birds start with random weights and they all play the same pipe at the same time. The ones that survive longest get to have children.

This is generation 0. The networks are random so most of the birds are dead before the first pipe even arrives.

![Generation 0, 94 of 1000 birds alive, score 0](assets/generation-0.png)

And this is generation 1, after one round of selection and mutation.

![Generation 1, 4 birds alive, score 20](assets/generation-1.png)

A score of 20 after a single generation. There's no training data in this and no labels and no gradients. The only thing that happens is the birds that crashed early stop having children.

## I don't use AI to write this code

I type all of it myself. No Copilot, no Cursor, no ChatGPT, no Claude.

That's most of the reason I started the project in the first place. If I got a model to write backprop for me I'd end up with working code sitting in front of me that I didn't understand, which defeats the whole purpose of doing it. I'd rather write a slow broken version, watch it fail and spend an afternoon working out what I did wrong. The code at the end of that matters a lot less to me than the part where I figure it out.

So when something in here looks naive it's usually because I haven't got to that part yet. The bugs are mine and so are the fixes.

## How the evolution works

Every bird has a `5 → 10 → 1` network. It takes in the bird's height, its velocity, how far away the pipe is, the height of the gap in the pipe, and the vertical distance between the bird and that gap. If the output comes out above 0 the bird flaps.

Once the last bird dies I sort them by how many frames they survived and keep the top 20 as parents. How many children a parent gets depends on where it ranked, so the best bird produces around 20/210 of the next generation and the worst of the 20 produces around 1/210. Each parent passes on one exact copy of itself and the rest of its children get mutated. Mutation walks through every weight with an 80% chance of nudging it by up to 0.05 in either direction, and 0.1 for the biases.

Pressing Enter kills the generation early. Escape prints the two best networks and quits.

There's a `bot_mode` flag in [`src/flappybird/mod.rs`](src/flappybird/mod.rs). Set it to false and you can play the game yourself with the spacebar.

The network code is in [`src/network.rs`](src/network.rs) and it's about 90 lines. Dense layers, random starting weights between -0.5 and 0.5, a forward pass, tanh on the output, and the mutation function. It derives Serialize and Deserialize so I can save populations.

## What isn't done

Most of it, honestly.

Pong has a ball and a paddle moving around but it isn't hooked up to `main.rs` yet and nothing is learning to play it.

Backprop is supposed to fit a polynomial to a target curve using gradient descent, so I can watch it working before I try the same thing on the network itself. At the moment `update()` is an empty function with a comment in it that says NEED TO WORK HERE. It doesn't compile and it's commented out of `main.rs`.

`models.json` has saved populations in it from older runs, one with 10 generations and one with 191. The code that wrote those files isn't in the repo anymore.

## What I want to add

Backpropagation is the next big one. Real gradient descent, so this isn't only evolution. I'll get the curve fitting demo working first and then move it onto the network.

After that, activation functions as an enum so I can choose between ReLU and sigmoid and tanh instead of having tanh hardcoded everywhere, and better starting weights. `rand_distr` is already in `Cargo.toml` for that.

Then loss functions and some kind of optimizer. SGD first, then momentum, then Adam if I get that far.

Reinforcement learning is the part I'm most interested in. Q-learning, then DQN, then policy gradients. I expect most of my time to end up going here.

At some point I need to get the training out of the render loop so it isn't capped at 60fps, and use rayon to evaluate birds in parallel.

Saving and loading models again, done properly this time.

More games after that. Pong needs finishing, then snake. I've been thinking about chess but it's a much harder problem than any of these and I'm probably getting ahead of myself there.

## Running it

```bash
cargo run --release
```

Use release mode. A thousand birds per frame in a debug build is painful.

It's Rust 2024 edition and macroquad does the rendering.

## Notes to self

Fitness is based on how many frames a bird survived rather than its score. It works, but a bird that sits still in a safe spot gets rewarded the same as one that's getting through pipes.

`calculate()` only runs tanh at the very end, so there's nothing nonlinear between the hidden layers and they collapse into a single linear map. It still plays Flappy Bird well enough because the problem is close to linearly separable, but the hidden layer isn't doing anything for me. First thing to fix.

The population loses its diversity quickly. I should try crossover, or mutation rates that change as it goes along.
