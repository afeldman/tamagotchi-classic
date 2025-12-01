# Tamagochi Classic - Firmware

Platform-agnostic Tamagochi implementation that runs on:

- **Desktop** (for development/testing)
- **ESP32** (embedded target)

## Architecture

```
firmware/
├── core/       # Platform-agnostic game logic (no_std)
├── desktop/    # Desktop CLI for testing
└── esp32/      # ESP32 embedded target
```

## Quick Start - Desktop Testing

```bash
cd desktop
cargo run

# Commands:
# f - feed
# p - play
# m - medicine
# c - clean
# s - status
# q - quit
```

## Building for ESP32

```bash
cd esp32
cargo build --release
```

## Core Logic Features

- ✅ Multiple life stages (Egg → Baby → Child → Teenager → Adult)
- ✅ Three adult forms based on care quality
- ✅ Hunger, happiness, health mechanics
- ✅ Evolution system
- ✅ Death conditions
- ✅ Care quality tracking
- ✅ Event system (evolution, death, birthdays)
- ✅ Full unit test coverage

## Testing

```bash
cd core
cargo test
```

## Development Workflow

1. Develop/test logic in `core/` with unit tests
2. Test interactively with `desktop/` CLI
3. Deploy to `esp32/` hardware

This ensures the same game logic runs everywhere!
