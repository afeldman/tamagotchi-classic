//! Tamagochi Classic - ESP32 Implementation
//! 
//! Hardware:
//! - ESP32 DevKit
//! - SSD1306 128x64 OLED (I2C)
//! - 4 Buttons on GPIO pins
//!
//! Pin Configuration:
//! - I2C SDA: GPIO21
//! - I2C SCL: GPIO22
//! - Button FEED: GPIO32
//! - Button PLAY: GPIO33
//! - Button MEDICINE: GPIO25
//! - Button CLEAN: GPIO26

use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{PinDriver, Pull},
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};

use ssd1306::{
    prelude::*,
    I2CDisplayInterface, Ssd1306,
};

use tamagochi_core::{Tamagochi, GameEvent};

mod sprites;

fn main() {
    // Initialize ESP-IDF
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("🐣 Tamagochi Classic v1.0.0 - ESP32 Edition");

    let peripherals = Peripherals::take().unwrap();

    // Initialize I2C for OLED display
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();

    // Initialize OLED display
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    display.clear_buffer();
    display.flush().unwrap();

    log::info!("Display initialized");

    // Initialize buttons with internal pull-up resistors
    let mut btn_feed = PinDriver::input(peripherals.pins.gpio32).unwrap();
    btn_feed.set_pull(Pull::Up).unwrap();
    
    let mut btn_play = PinDriver::input(peripherals.pins.gpio33).unwrap();
    btn_play.set_pull(Pull::Up).unwrap();
    
    let mut btn_medicine = PinDriver::input(peripherals.pins.gpio25).unwrap();
    btn_medicine.set_pull(Pull::Up).unwrap();
    
    let mut btn_clean = PinDriver::input(peripherals.pins.gpio26).unwrap();
    btn_clean.set_pull(Pull::Up).unwrap();

    log::info!("Buttons initialized");

    // Initialize Tamagochi game state
    let mut tama = Tamagochi::new();
    let mut frame: u8 = 0;
    let mut last_button_state = [false; 4];

    // Show welcome screen
    draw_welcome(&mut display);
    FreeRtos::delay_ms(2000);

    log::info!("Starting main game loop");

    // Main game loop
    loop {
        // Read button states (active LOW with pull-up)
        let btn_states = [
            btn_feed.is_low(),
            btn_play.is_low(),
            btn_medicine.is_low(),
            btn_clean.is_low(),
        ];

        // Handle button presses (detect rising edge)
        for (i, (&current, &last)) in btn_states.iter().zip(last_button_state.iter()).enumerate() {
            if current && !last {
                // Button was just pressed
                match i {
                    0 => {
                        log::info!("Button: FEED");
                        tama.feed();
                        show_action_feedback(&mut display, "FEED!");
                    }
                    1 => {
                        log::info!("Button: PLAY");
                        tama.play();
                        show_action_feedback(&mut display, "PLAY!");
                    }
                    2 => {
                        log::info!("Button: MEDICINE");
                        tama.medicine();
                        show_action_feedback(&mut display, "MEDICINE!");
                    }
                    3 => {
                        log::info!("Button: CLEAN");
                        tama.clean();
                        show_action_feedback(&mut display, "CLEAN!");
                    }
                    _ => {}
                }
            }
        }
        last_button_state = btn_states;

        // Update game state
        if let Some(event) = tama.update() {
            handle_event(&mut display, event);
        }

        // Render current state to display
        draw_game_state(&mut display, &tama, frame);

        // Increment animation frame
        frame = frame.wrapping_add(1);

        // Small delay for animation timing and debouncing
        FreeRtos::delay_ms(100);
    }
}

/// Draw welcome screen
fn draw_welcome(display: &mut impl DrawTarget<Color = BinaryColor>) {
    display.clear_buffer();
    
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    
    Text::new("TAMAGOCHI", Point::new(20, 20), text_style)
        .draw(display)
        .ok();
    Text::new("CLASSIC v1.0", Point::new(15, 35), text_style)
        .draw(display)
        .ok();
    Text::new("Press any button", Point::new(5, 55), text_style)
        .draw(display)
        .ok();
    
    display.flush().ok();
}

/// Draw the main game state
fn draw_game_state(
    display: &mut impl DrawTarget<Color = BinaryColor>,
    tama: &Tamagochi,
    frame: u8,
) {
    display.clear_buffer();

    // Draw sprite in center-top area
    sprites::draw_sprite(display, tama.stage, frame, Point::new(40, 5));

    // Draw status bars at bottom
    draw_status_bars(display, tama);

    // Draw warning indicators if needed
    draw_warnings(display, tama);

    display.flush().ok();
}

/// Draw status bars for hunger, happiness, health
fn draw_status_bars(display: &mut impl DrawTarget<Color = BinaryColor>, tama: &Tamagochi) {
    let bar_width = 40;
    let bar_height = 4;
    let start_y = 52;
    
    // Hunger bar (0 = empty, 100 = full, so invert for display)
    let hunger_fill = ((100 - tama.hunger) as i32 * bar_width) / 100;
    draw_bar(display, Point::new(0, start_y), bar_width, bar_height, hunger_fill);
    
    // Happiness bar
    let happy_fill = (tama.happiness as i32 * bar_width) / 100;
    draw_bar(display, Point::new(44, start_y), bar_width, bar_height, happy_fill);
    
    // Health bar
    let health_fill = (tama.health as i32 * bar_width) / 100;
    draw_bar(display, Point::new(88, start_y), bar_width, bar_height, health_fill);
    
    // Labels
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("F", Point::new(0, 63), text_style).draw(display).ok();
    Text::new("H", Point::new(44, 63), text_style).draw(display).ok();
    Text::new("M", Point::new(88, 63), text_style).draw(display).ok();
}

/// Draw a single status bar
fn draw_bar(
    display: &mut impl DrawTarget<Color = BinaryColor>,
    position: Point,
    width: i32,
    height: i32,
    fill: i32,
) {
    // Draw outline
    Rectangle::new(position, Size::new(width as u32, height as u32))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    
    // Draw filled portion
    if fill > 0 {
        Rectangle::new(
            position + Point::new(1, 1),
            Size::new((fill - 2).max(0) as u32, (height - 2) as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)
        .ok();
    }
}

/// Draw warning indicators
fn draw_warnings(display: &mut impl DrawTarget<Color = BinaryColor>, tama: &Tamagochi) {
    let needs = tama.needs_status();
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    
    let mut y = 42;
    if needs.hunger_critical {
        Text::new("HUNGRY!", Point::new(2, y), text_style).draw(display).ok();
    }
    if needs.happiness_critical {
        Text::new("SAD!", Point::new(50, y), text_style).draw(display).ok();
    }
    if needs.health_critical {
        Text::new("SICK!", Point::new(90, y), text_style).draw(display).ok();
    }
}

/// Show brief action feedback
fn show_action_feedback(display: &mut impl DrawTarget<Color = BinaryColor>, text: &str) {
    display.clear_buffer();
    
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new(text, Point::new(40, 32), text_style)
        .draw(display)
        .ok();
    
    display.flush().ok();
    FreeRtos::delay_ms(300);
}

/// Handle game events
fn handle_event(display: &mut impl DrawTarget<Color = BinaryColor>, event: GameEvent) {
    match event {
        GameEvent::Evolution(stage) => {
            log::info!("Evolution: {:?}", stage);
            display.clear_buffer();
            let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
            Text::new("EVOLUTION!", Point::new(20, 32), text_style)
                .draw(display)
                .ok();
            display.flush().ok();
            FreeRtos::delay_ms(2000);
        }
        GameEvent::Death => {
            log::info!("Tamagochi died");
            display.clear_buffer();
            let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
            Text::new("R.I.P.", Point::new(45, 32), text_style)
                .draw(display)
                .ok();
            display.flush().ok();
            // Stay on death screen
            loop {
                FreeRtos::delay_ms(1000);
            }
        }
        GameEvent::NeedsAttention => {
            log::info!("Needs attention");
            // Visual indicator already shown by draw_warnings
        }
        GameEvent::Birthday(days) => {
            log::info!("Birthday: {} days", days);
            display.clear_buffer();
            let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
            Text::new("BIRTHDAY!", Point::new(25, 32), text_style)
                .draw(display)
                .ok();
            display.flush().ok();
            FreeRtos::delay_ms(2000);
        }
    }
}
