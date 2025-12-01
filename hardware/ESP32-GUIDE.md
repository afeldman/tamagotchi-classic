# Tamagochi Classic - ESP32 Hardware Guide

## Hardware Requirements

### Components

- **ESP32 DevKit** (any variant)
- **SSD1306 OLED Display** (128x64, I2C)
- **4x Tactile Buttons**
- **4x 10kΩ Resistors** (pull-up, optional - using internal)
- **Breadboard & Jumper Wires**
- **USB Cable** for programming

### Wiring Diagram

```
ESP32 Pin Layout:
┌─────────────────┐
│  ESP32 DevKit   │
│                 │
│ GPIO21 ────────┼─── SDA (OLED Display)
│ GPIO22 ────────┼─── SCL (OLED Display)
│ 3.3V ──────────┼─── VCC (OLED Display)
│ GND ───────────┼─── GND (OLED Display)
│                 │
│ GPIO32 ────────┼─── Button FEED (to GND)
│ GPIO33 ────────┼─── Button PLAY (to GND)
│ GPIO25 ────────┼─── Button MEDICINE (to GND)
│ GPIO26 ────────┼─── Button CLEAN (to GND)
│                 │
└─────────────────┘
```

### Button Wiring

Each button connects the GPIO pin to GND when pressed.
Internal pull-up resistors are enabled in software.

```
GPIO Pin ────┐
             │
         [Button]
             │
            GND
```

## Software Setup

### Prerequisites

1. **Install Rust ESP toolchain:**

```bash
cargo install espup
espup install
source ~/export-esp.sh  # or add to .bashrc/.zshrc
```

2. **Install espflash:**

```bash
cargo install espflash
```

3. **Install ldproxy:**

```bash
cargo install ldproxy
```

### Building

```bash
cd esp32
cargo build --release
```

### Flashing to ESP32

1. **Connect ESP32 via USB**

2. **Flash the firmware:**

```bash
cargo run --release
```

Or manually:

```bash
espflash flash target/xtensa-esp32-espidf/release/tamagochi-esp32 --monitor
```

### Monitoring Serial Output

```bash
espflash monitor
```

## Gameplay

### Controls

- **FEED Button (GPIO32)**: Feed your Tamagochi (reduces hunger)
- **PLAY Button (GPIO33)**: Play with Tamagochi (increases happiness)
- **MEDICINE Button (GPIO25)**: Give medicine (restores health)
- **CLEAN Button (GPIO26)**: Clean Tamagochi (small happiness boost)

### Display Layout

```
┌────────────────────────┐
│                        │
│    [TAMAGOCHI SPRITE]  │ ← Animated character
│                        │
│                        │
│   [WARNING TEXT]       │ ← HUNGRY! / SAD! / SICK!
│                        │
│  ████░░  ████░░  ██░░  │ ← Status bars
│   F       H       M    │   (Food/Happy/Medicine)
└────────────────────────┘
```

### Features

✅ **Continuous Animation**: Sprite animates while you interact
✅ **Real-time Updates**: Status bars update as stats change
✅ **Button Responsiveness**: Immediate feedback on button press
✅ **Visual Warnings**: Screen shows critical needs
✅ **Evolution System**: Character changes form based on care
✅ **Power Efficient**: Uses FreeRTOS delays for low power

## Troubleshooting

### Display not working?

- Check I2C connections (SDA/SCL)
- Verify 3.3V power supply
- Check I2C address (default 0x3C)

### Buttons not responding?

- Verify GPIO pin numbers match code
- Check button wiring (active LOW)
- Test with multimeter for continuity

### Build errors?

- Ensure ESP-IDF toolchain is installed
- Run `source ~/export-esp.sh`
- Update dependencies: `cargo update`

### Serial monitor shows panic?

- Check for hardware connection issues
- Verify pin configurations match your wiring
- Enable verbose logging in code

## Development Tips

### Testing without hardware

Use the desktop version for rapid development:

```bash
cd ../desktop
cargo run
```

### Adjusting game speed

Modify the delay in main loop (currently 100ms):

```rust
FreeRtos::delay_ms(100); // Change this value
```

### Customizing pins

Edit pin numbers in `main.rs`:

```rust
let mut btn_feed = PinDriver::input(peripherals.pins.gpio32).unwrap();
// Change gpio32 to your desired pin
```

## Next Steps

- [ ] Add deep sleep for battery operation
- [ ] Implement RTC for real-time tracking
- [ ] Add sound/buzzer for alerts
- [ ] Battery level indicator
- [ ] Save state to flash memory
- [ ] WiFi sync with mobile app (v2.0)
