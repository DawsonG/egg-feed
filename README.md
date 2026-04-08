# Feed the Egg

A simple, fun clicker game built with [Bevy](https://bevyengine.org/). Click and drag eggs to feed the other egg! Based on the game from Tim Robinson's "I Think You Should Leave".

## How to Play

1. Click on the **egg bowl** on the right side of the screen to grab an egg
2. Drag the egg to **Eggy's mouth** (the egg on the left)
3. Release the mouse to feed the egg
4. Watch your score increase with each successful feed
5. See how many eggs you can feed!

## Controls

- **Left Click** on the egg bowl to grab an egg
- **Drag** to move the egg with your cursor
- **Release** to drop the egg into Eggy's mouth

The egg's mouth opens when you hover over it to show it's ready to eat!

## Building & Running

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Cargo (comes with Rust)

### Run the Game

```bash
cargo run --release
```

### Web Build

To build for the web:

```bash
cargo build --release --profile wasm-release --target wasm32-unknown-unknown
```

## About

This is a minimal game project demonstrating:
- 2D sprite rendering with Bevy
- Mouse input handling and pointer picking
- Simple game state management
- Pixel art graphics