# nn

A machine learning library I'm writing from scratch in Rust — no `tch`, no `candle`, no `ndarray`. Just `Vec<f32>` and the math.

The point isn't to ship a crate. The point is to learn Rust and machine learning at the same time, by building both ends myself: the networks, the training loops, and the little games they learn to play.

> **Status: very early.** The neuroevolution side works and is fun to watch. Almost everything else is scaffolding, half-finished, or still an idea. Expect churn.

## What works right now

**Flappy Bird + neuroevolution.** 1000 birds spawn with random networks, all play simultaneously against the same pipe, and the ones that survive longest get to reproduce. No gradients involved — just mutation and selection.

- Each bird gets a `5 → 10 → 1` network. Inputs: its height, its velocity, horizontal distance to the pipe, the pipe's gap height, and the vertical offset between the two. Output > 0 means flap.
- When the last bird dies, the population is sorted by survival frames and the top 20 become parents. Offspring are allocated by rank — the best bird gets ~20/210 of the next generation, the 20th gets ~1/210 — and every child except one clone per parent gets mutated.
- Mutation is per-weight: 80% chance to jiggle by ±0.05 (±0.1 for biases).
- Press `Enter` to force-kill the generation, `Escape` to dump the two best networks and quit.

Flip `bot_mode` to `false` in [`src/flappybird/mod.rs`](src/flappybird/mod.rs) if you just want to play it yourself with the spacebar.

**The network core** ([`src/network.rs`](src/network.rs)) is about 90 lines: dense layers, uniform random init in `[-0.5, 0.5]`, a forward pass, `tanh` on the output, and a mutation operator. Everything is `Serialize`/`Deserialize` so populations can be checkpointed.

## What's half-built

- **Pong** ([`src/pong/mod.rs`](src/pong/mod.rs)) — ball physics and a bouncing paddle exist, but the module isn't even wired into `main.rs` yet and nothing is learning to play it.
- **Backprop** ([`src/backprop/mod.rs`](src/backprop/mod.rs)) — the idea is to fit a polynomial curve to a target curve with gradient descent as a visual warm-up before doing it properly on the network. The `update()` function is currently a `//NEED TO WORK HERE` comment. It doesn't compile; it's commented out of `main.rs`.
- **`models.json`** — saved populations (10 and 191 generations) from earlier runs. The save/load code that produced them isn't in the current tree.
- **Blackjack** — referenced in `main.rs`, doesn't exist anymore.

## Roadmap

Roughly in the order I want to tackle it:

- [ ] **Backpropagation** — real gradient descent, so the library isn't purely evolutionary. Start with the curve-fitting demo, then generalize to `Network`.
- [ ] Activation functions as a proper enum (ReLU, sigmoid, tanh) instead of a hardcoded `tanh`, and better weight init (Xavier/He — `rand_distr` is already pulled in for it).
- [ ] Loss functions + an optimizer abstraction (SGD, then momentum, then Adam).
- [ ] **Reinforcement learning** — Q-learning first, then DQN, then policy gradients. This is the big one.
- [ ] Headless training (decouple the sim from the render loop so it isn't capped at 60fps) and parallel evaluation with `rayon`.
- [ ] Model save/load back in, properly this time.
- [ ] **More games.** Pong finished, snake, maybe **chess** — though chess is a different beast entirely and I might be getting ahead of myself.

## Running it

```bash
cargo run --release
```

`--release` matters — 1000 birds per frame in a debug build is miserable.

Built on Rust 2024 edition with [macroquad](https://macroquad.rs/) for rendering.

## Notes to self

- The fitness function is survival frames, not score. It works, but it rewards a bird that hovers safely as much as one that threads pipes.
- `Network::calculate` applies `tanh` only at the very end — there's no nonlinearity between hidden layers, so the hidden layers currently collapse into a single linear map. Fixing this is the first thing on the list.
- Population diversity dies fast. Worth trying crossover, or adaptive mutation rates, or both.
