# finch

A machine learning library I'm writing from scratch in Rust. No `tch`, no `candle`, no `ndarray` — just `Vec<f32>` and whatever math I can work out myself.

It isn't meant to be a crate anyone depends on. I'm using it to learn Rust and machine learning at the same time, which means writing the networks, the training loops, and the games they play. **All of it by hand, with no AI involved.** More on that below.

## What it does right now

It plays Flappy Bird. 1000 birds spawn with random weights and all play the same pipe at once, and the ones that live longest get to have children.

Generation 0. These are random networks, so most of the population is dead before the first pipe even shows up:

![Generation 0: 94 of 1000 birds alive, score 0](assets/generation-0.png)

Generation 1, after one round of selection and mutation:

![Generation 1: 4 birds alive, score 20](assets/generation-1.png)

Score 20 after a single generation. No training data, no labels, no gradients. Just killing off the birds that crashed early.

## No AI wrote any of this code

I type all of it myself. No Copilot, no Cursor, no ChatGPT, no Claude.

This is the whole reason the project exists. If I let a model write backprop for me I'd end up with code that works and no idea why, and then I'd have learned nothing. I'd rather write a slow, wrong version, watch it break, and work out what I got wrong. That part is the point. The finished code is mostly a byproduct.

So when something in here looks naive, it's because I haven't learned that bit yet. The bugs are mine and so are the fixes.

## How the evolution works

Each bird has a `5 → 10 → 1` network. The inputs are its height, its velocity, how far it is from the pipe horizontally, the height of the pipe's gap, and the vertical distance between the bird and that gap. If the output comes out above 0, it flaps.

When the last bird dies I sort the population by how many frames each one survived and take the top 20. Those become the parents. How many children each one gets depends on its rank, so the best bird gets about 20/210 of the next generation and the 20th gets about 1/210. Every parent keeps one unmutated clone of itself in the next generation and the rest of its children get mutated. Mutation is per weight: an 80% chance to nudge it by up to ±0.05, or ±0.1 for biases.

`Enter` kills the current generation early. `Escape` dumps the two best networks and quits.

If you'd rather play it yourself with the spacebar, set `bot_mode` to `false` in [`src/flappybird/mod.rs`](src/flappybird/mod.rs).

The network itself ([`src/network.rs`](src/network.rs)) is about 90 lines. Dense layers, random init between -0.5 and 0.5, a forward pass, `tanh` on the output, and the mutation function. It derives `Serialize`/`Deserialize` so populations can be saved.

## What isn't finished

Most of it. This is early.

**Pong** ([`src/pong/mod.rs`](src/pong/mod.rs)) has a ball and a paddle that bounces around, but it isn't wired into `main.rs` and nothing is learning to play it yet.

**Backprop** ([`src/backprop/mod.rs`](src/backprop/mod.rs)) is meant to fit a polynomial to a target curve using gradient descent, as a visual warm-up before I try it on the actual network. Right now `update()` is a comment saying `//NEED TO WORK HERE`. It doesn't compile and it's commented out of `main.rs`.

**`models.json`** holds saved populations from older runs, 10 generations and 191 generations. The code that saved them isn't in the tree anymore.

## What I want to add

Roughly in this order:

- **Backpropagation.** Real gradient descent, so this isn't purely evolutionary. The curve demo first, then the network.
- Activations as an enum (ReLU, sigmoid, tanh) instead of `tanh` hardcoded, and better weight init. `rand_distr` is already in `Cargo.toml` for this.
- Loss functions, then an optimizer of some kind. SGD, then momentum, then Adam if I get that far.
- **Reinforcement learning.** Q-learning, then DQN, then policy gradients. This is the big one and probably where most of the time goes.
- Headless training, so it isn't stuck at 60fps because everything runs inside the render loop. `rayon` for evaluating birds in parallel.
- Saving and loading models again, properly this time.
- More games. Finish Pong, then snake, and maybe chess. Chess is a much bigger problem than the others and I might be getting ahead of myself there.

## Running it

```bash
cargo run --release
```

Use `--release`. 1000 birds per frame in a debug build is painful.

Rust 2024 edition, with [macroquad](https://macroquad.rs/) doing the rendering.

## Notes to self

Fitness is survival frames, not score. It works, but a bird that hovers in a safe spot scores the same as one that actually gets through pipes.

`calculate()` only applies `tanh` at the very end, so there's no nonlinearity between the hidden layers and they collapse into one linear map. It still plays Flappy Bird fine, since the problem is close to linearly separable, but the hidden layer isn't earning its place. Fix this first.

Diversity in the population dies off fast. Worth trying crossover, or mutation rates that adapt over time.
