# Dino Run

A Chrome dino-style endless runner built with Rust and macroquad.

## Controls

| Key | Action |
|-----|--------|
| SPACE / UP | Jump (double jump costs mana) |
| DOWN | Duck |
| F | Fly (hold while jumping, costs mana) |
| E | Eat (catch flies to restore mana) |
| R | Restart (game over screen) |

## Build & Run

```bash
# Run
cargo run --release

# Build only
cargo build --release
```

Or use the Makefile:

```bash
make run
make build
make clean
```

## Requirements

- Rust 1.70+
- macroquad 0.4
