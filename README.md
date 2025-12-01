# 🐣 Tamagochi Classic v1.0.0

> The pure 90s experience - open source hardware Tamagochi

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-ESP32-blue.svg)](https://www.espressif.com/en/products/socs/esp32)

## 🎯 What is this?

A faithful recreation of the classic Tamagochi virtual pet using modern open-source hardware. Build your own physical Tamagochi with:

- ✅ **Pure retro gameplay** - No blockchain, no complexity, just classic fun
- ✅ **Open source hardware** - Complete schematics and BOM
- ✅ **Affordable** - ~€25 total cost
- ✅ **DIY-friendly** - 2-3 hour build time for beginners
- ✅ **Platform-agnostic core** - Same logic runs on desktop and ESP32

## 🚀 Quick Start

### Option 1: Desktop Testing (No Hardware)

```bash
cd firmware
task desktop:run
```

Play with ASCII graphics in your terminal!

### Option 2: Build Real Hardware

See [hardware/BOM.md](hardware/BOM.md) for complete parts list and [hardware/ESP32-GUIDE.md](hardware/ESP32-GUIDE.md) for assembly instructions.

```bash
# One-time setup
task esp32:setup
source ~/export-esp.sh

# Build and flash
task esp32:flash
```

## 📦 Hardware Requirements

| Component             | Price    | Link  |
| --------------------- | -------- | ----- |
| ESP32 DevKit          | €5.00    | [Buy] |
| SSD1306 OLED (128x64) | €8.00    | [Buy] |
| 500mAh LiPo Battery   | €3.50    | [Buy] |
| 4x Tactile Buttons    | €1.00    | [Buy] |
| PCB & Connectors      | €7.50    | [Buy] |
| **Total**             | **~€25** |       |

## 🎮 Gameplay

### Controls

- **FEED** (GPIO32) - Feed your Tamagochi
- **PLAY** (GPIO33) - Play and increase happiness
- **MEDICINE** (GPIO25) - Restore health
- **CLEAN** (GPIO26) - Clean and care

### Features

- 🥚 **5 Life Stages**: Egg → Baby → Child → Teenager → Adult
- 🌟 **3 Adult Forms**: Neglected / Normal / Premium (based on care)
- 📊 **3 Stats**: Hunger, Happiness, Health
- ⚡ **Real-time Evolution**: Character grows based on your care
- 💀 **Consequences**: Neglect can lead to death
- 🎂 **Milestones**: Birthday celebrations every day

## 🛠️ Development

### Project Structure

```
classic/
├── firmware/           # Rust codebase
│   ├── core/          # Platform-agnostic game logic
│   ├── desktop/       # CLI version for testing
│   └── esp32/         # Hardware implementation
├── hardware/          # Electronics & assembly
│   ├── BOM.md         # Bill of materials
│   ├── assembly-guide.md
│   └── ESP32-GUIDE.md
├── docs/              # Documentation
├── 3d-printing/       # STL files for case
└── Taskfile.yml       # Build automation

```

### Available Tasks

```bash
task --list-all        # Show all available tasks

# Development
task desktop:run       # Test on computer
task esp32:flash       # Flash to hardware
task dev:desktop       # Quick dev cycle
task dev:esp32         # Quick ESP32 cycle

# Testing & Quality
task core:test         # Run unit tests
task test:all          # All tests
task clippy            # Linter
task fmt               # Format code

# Building
task build:all         # Build everything
task release:desktop   # Desktop release
task release:esp32     # ESP32 release
```

### Testing the Core Logic

```bash
cd firmware/core
cargo test
```

All game logic is thoroughly tested with unit tests.

### Desktop Development

The desktop version is perfect for rapid prototyping:

```bash
task desktop:run
```

Commands: `f` (feed), `p` (play), `m` (medicine), `c` (clean), `s` (status), `q` (quit)

## 📖 Documentation

- [Firmware README](firmware/README.md) - Technical architecture
- [ESP32 Hardware Guide](hardware/ESP32-GUIDE.md) - Complete wiring & setup
- [Assembly Guide](hardware/assembly-guide.md) - Step-by-step build
- [Getting Started](docs/getting-started.md) - Quick start guide

## 🧪 Technical Details

### Architecture

- **Language**: Rust (no_std for embedded)
- **Microcontroller**: ESP32 (xtensa architecture)
- **Display**: SSD1306 OLED via I2C
- **Graphics**: embedded-graphics library
- **Build System**: Cargo + Taskfile

### Why Rust?

- Memory safety without garbage collection
- Zero-cost abstractions
- Excellent embedded support
- Same code runs on desktop and ESP32

## 🤝 Contributing

We welcome contributions! This is the open-source version - help make it better:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Make your changes
4. Run tests (`task test:all`)
5. Commit (`git commit -m 'Add amazing feature'`)
6. Push (`git push origin feature/amazing`)
7. Open a Pull Request

### Development Setup

```bash
# Clone the repo
git clone https://github.com/afeldman/tamagotchi-classic.git
cd tamagotchi-classic

# Test on desktop first
cd firmware
cargo run -p tamagochi-desktop

# For ESP32 development
task esp32:setup
source ~/export-esp.sh
```

## 📜 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

### What this means:

- ✅ Use it commercially
- ✅ Modify it freely
- ✅ Distribute it
- ✅ Use it privately
- ⚠️ No warranty provided

## 🎯 Roadmap

### v1.0.0 (Current)

- [x] Core game logic
- [x] Desktop CLI version
- [x] ESP32 implementation
- [x] ASCII/Pixel art graphics
- [x] Full documentation
- [ ] 3D printable case
- [ ] Battery optimization
- [ ] Community testing

### v1.1.0 (Planned)

- [ ] Sound effects (buzzer)
- [ ] Save state to flash
- [ ] Multiple save slots
- [ ] Breeding mechanics
- [ ] Mini-games

### v2.0.0 (Future - Separate Proprietary Product)

- Blockchain integration
- NFT features
- Mobile app
- Play-to-Earn

## 🌟 Features in Detail

### Life Stages

| Stage       | Duration | Description                          |
| ----------- | -------- | ------------------------------------ |
| 🥚 Egg      | 1 hour   | Freshly born, needs basic care       |
| 👶 Baby     | 24 hours | Newborn, requires frequent attention |
| 🧒 Child    | 3 days   | Growing, developing personality      |
| 🧑 Teenager | 7 days   | Active, needs lots of play           |
| 👤 Adult    | Forever  | Final form based on care quality     |

### Adult Forms

Your care quality (0-100) determines the final adult form:

- **Neglected** (0-30): Poor care, sad appearance
- **Normal** (31-70): Average care, standard form
- **Premium** (71-100): Excellent care, happy with sparkles ✨

### Stats Explained

- **Hunger** (0-100): 0 = starving, 100 = full

  - Increases by 1 per second
  - Feed to reduce by 20
  - Critical at 80+ (health degrades)

- **Happiness** (0-100): 0 = very sad, 100 = very happy

  - Decreases by 1 per second
  - Play to increase by 15
  - Critical at <20 (health degrades)

- **Health** (0-100): 0 = death, 100 = perfect
  - Degrades when hungry or sad
  - Medicine restores 30 points
  - Death occurs at 0

## 🐛 Troubleshooting

### Desktop version won't compile?

```bash
# Update Rust
rustup update
cd firmware
cargo clean
cargo build
```

### ESP32 flash fails?

```bash
# Check toolchain
source ~/export-esp.sh
espflash board-info

# Try with explicit port
espflash flash --port /dev/ttyUSB0 target/release/tamagochi-esp32
```

### Display shows nothing?

- Check I2C wiring (SDA=GPIO21, SCL=GPIO22)
- Verify 3.3V power supply
- Test I2C address (usually 0x3C)

### Buttons not working?

- Buttons should connect GPIO to GND
- Internal pull-ups are enabled
- Check GPIO pin numbers in code

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/afeldman/tamagotchi-classic/issues)
- **Discussions**: [GitHub Discussions](https://github.com/afeldman/tamagotchi-classic/discussions)
- **Documentation**: Check the `docs/` folder

## 🙏 Acknowledgments

- Inspired by the original Bandai Tamagochi (1996)
- Built with ❤️ using Rust and ESP-IDF
- Thanks to the embedded-rust community

## ⚡ Performance

- **Frame Rate**: ~10 FPS (100ms per frame)
- **Battery Life**: ~7 days with 500mAh LiPo
- **Memory**: <50KB RAM, ~200KB Flash
- **Bootup Time**: ~2 seconds

## 🔒 Security Note

This is a standalone device with no network connectivity in v1.0. Your Tamagochi data stays on your device.

---

**Made with 🐣 by the Tamagochi Classic community**

_Remember: A Tamagochi is for life, not just for Christmas!_ 🎄
