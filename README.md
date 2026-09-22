# nn

![Rust](https://img.shields.io/badge/Rust-2024_edition-CE422B?logo=rust&logoColor=white)
![Handwritten](https://img.shields.io/badge/code-100%25_handwritten-2ea44f)
![Status](https://img.shields.io/badge/status-early_WIP-DAA520)

A machine learning library I'm writing from scratch in Rust — no `tch`, no `candle`, no `ndarray`. Just `Vec<f32>` and the math.

The point isn't to ship a crate. The point is to learn Rust and machine learning at the same time, by building both ends myself: the networks, the training loops, and the little games they learn to play.

---

## ✍️ Every line of code here is handwritten

**No AI wrote any of the code in this repository.** No Copilot, no Cursor, no ChatGPT, no Claude. Not the neural network, not the forward pass, not the evolution loop, not the games. Every line of Rust in this repo was typed out by me, by hand, character by character.

That isn't a side note — **it's the entire point of the project.** If I autocompleted my way through a backprop implementation I'd end up with working code and no understanding, which is the exact opposite of what I'm here for. I'd rather write a slow, wrong, ugly version myself and then figure out *why* it's wrong.

So when you read something in here that looks naive, it's because I haven't learned that part yet — not because a model generated it. The bugs are mine. So are the fixes.

---

## It works

**Generation 0** — 1000 birds spawn with completely random networks. Most of the population is dead before the first pipe even arrives.

![Generation 0 — 1000 random networks, 94 birds still alive, score 0](assets/generation-0.png)

**Generation 1** — after a single round of selection and mutation, the survivors are threading pipes.

![Generation 1 — 4 birds alive, score 20](assets/generation-1.png)

That's *one* generation. No gradients, no training data, no labels — just "the birds that lasted longest get to have children."

## How it actually works

**Flappy Bird + neuroevolution.** 1000 birds play the same pipe simultaneously, and the ones that survive longest reproduce.

- Each bird gets a `5 → 10 → 1` network. Inputs: its height, its velocity, horizontal distance to the pipe, the pipe's gap height, and the vertical offset between the two. Output > 0 means flap.
- When the last bird dies, the population is sorted by survival frames and the top 20 become parents. Offspring are allocated by rank — the best bird gets ~20/210 of the next generation, the 20th gets ~1/210 — and every child except one clone per parent gets mutated.
- Mutation is per-weight: 80% chance to jiggle by ±0.05 (±0.1 for biases).
- Press `Enter` to force-kill the generation, `Escape` to dump the two best networks and quit.

Flip `bot_mode` to `false` in [`src/flappybird/mod.rs`](src/flappybird/mod.rs) if you just want to play it yourself with the spacebar.

**The network core** ([`src/network.rs`](src/network.rs)) is about 90 lines: dense layers, uniform random init in `[-0.5, 0.5]`, a forward pass, `tanh` on the output, and a mutation operator. Everything is `Serialize`/`Deserialize` so populations can be checkpointed.

## What's half-built

> **Status: very early.** The neuroevolution side works and is fun to watch. Most of the rest is scaffolding, half-finished, or still an idea. Expect churn.

- **Pong** ([`src/pong/mod.rs`](src/pong/mod.rs)) — ball physics and a bouncing paddle exist, but the module isn't wired into `main.rs` yet and nothing is learning to play it.
- **Backprop** ([`src/backprop/mod.rs`](src/backprop/mod.rs)) — the idea is to fit a polynomial curve to a target curve with gradient descent as a visual warm-up before doing it properly on the network. The `update()` function is currently a `//NEED TO WORK HERE` comment. It doesn't compile; it's commented out of `main.rs`.
- **`models.json`** — saved populations (10 and 191 generations) from earlier runs. The save/load code that produced them isn't in the current tree.

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
- `Network::calculate` applies `tanh` only at the very end — there's no nonlinearity *between* hidden layers, so they currently collapse into a single linear map. It still plays flappy bird fine (the problem is close to linearly separable), but the hidden layer isn't buying anything. Fixing this is first on the list.
- Population diversity dies fast. Worth trying crossover, or adaptive mutation rates, or both.
