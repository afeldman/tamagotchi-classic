//! Tamagochi Classic - Desktop CLI Version
//! 
//! This is a command-line implementation for testing and development.
//! The core logic is identical to what runs on ESP32.

mod sprites;
use tamagochi_core as core;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use core::{Tamagochi, GameEvent};

fn main() {
    println!("🐣 Tamagochi Classic v1.0.0 - Desktop Edition");
    println!("==============================================\n");

    let mut tama = Tamagochi::new();
    let mut running = true;
    let mut frame: u8 = 0;

    println!("Your Tamagochi is born! 🥚");
    println!("Commands: (f)eed, (p)lay, (m)edicine, (c)lean, (s)tatus, (q)uit\n");
    
    // Show initial sprite
    display_tamagochi(&tama, frame);

    while running && tama.alive {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "f" | "feed" => {
                tama.feed();
                println!("🍔 You fed your Tamagochi!");
                display_tamagochi(&tama, frame);
            }
            "p" | "play" => {
                tama.play();
                println!("🎮 You played with your Tamagochi!");
                display_tamagochi(&tama, frame);
            }
            "m" | "medicine" => {
                tama.medicine();
                println!("💊 You gave medicine to your Tamagochi!");
                display_tamagochi(&tama, frame);
            }
            "c" | "clean" => {
                tama.clean();
                println!("🧹 You cleaned your Tamagochi!");
                display_tamagochi(&tama, frame);
            }
            "s" | "status" => {
                display_status(&tama);
                display_tamagochi(&tama, frame);
            }
            "q" | "quit" => {
                running = false;
                println!("👋 Goodbye!");
            }
            "tick" => {
                // Manual tick for testing
                if let Some(event) = tama.update() {
                    handle_event(event, &tama, frame);
                }
            }
            _ => {
                println!("❓ Unknown command. Try: f, p, m, c, s, q");
            }
        }

        // Simulate time passing (1 second = 1 tick)
        thread::sleep(Duration::from_millis(100));
        frame = frame.wrapping_add(1);
        if let Some(event) = tama.update() {
            handle_event(event, &tama, frame);
        }
    }

    if !tama.alive {
        println!("\n💀 Your Tamagochi has died. RIP.");
        println!("   Age: {} days", tama.age / 86400);
        display_tamagochi(&tama, frame);
    }
}

fn display_tamagochi(tama: &Tamagochi, frame: u8) {
    if !tama.alive {
        println!("{}", sprites::get_death_sprite());
        return;
    }

    let sprite = sprites::get_sprite(tama.stage, frame);
    println!("{}", sprite);

    // Show quick status indicators
    let needs = tama.needs_status();
    let mut indicators = Vec::new();
    
    if needs.hunger_critical {
        indicators.push("🍔 HUNGRY");
    }
    if needs.happiness_critical {
        indicators.push("😢 SAD");
    }
    if needs.health_critical {
        indicators.push("🤒 SICK");
    }
    
    if !indicators.is_empty() {
        println!("⚠️  {}", indicators.join(" | "));
    }
    println!();
}

fn display_status(tama: &Tamagochi) {
    let summary = tama.status_summary();
    
    println!("\n┌─────────────────────────────┐");
    println!("│   TAMAGOCHI STATUS          │");
    println!("├─────────────────────────────┤");
    println!("│ Stage:     {:?}", summary.stage);
    println!("│ Age:       {} days", summary.age_days);
    println!("│ Hunger:    {} {}", summary.hunger_level, bar(summary.hunger_level));
    println!("│ Happiness: {} {}", summary.happiness_level, bar(summary.happiness_level));
    println!("│ Health:    {} {}", summary.health_level, bar(summary.health_level));
    println!("│ Care:      {} {}", summary.care_rating, bar(summary.care_rating));
    println!("└─────────────────────────────┘\n");

    let needs = tama.needs_status();
    if needs.hunger_critical {
        println!("⚠️  HUNGRY! Feed your Tamagochi!");
    }
    if needs.happiness_critical {
        println!("⚠️  SAD! Play with your Tamagochi!");
    }
    if needs.health_critical {
        println!("⚠️  SICK! Give medicine!");
    }
}

fn handle_event(event: GameEvent, tama: &Tamagochi, frame: u8) {
    match event {
        GameEvent::Evolution(stage) => {
            println!("\n✨ EVOLUTION! Your Tamagochi evolved to: {:?}", stage);
            display_tamagochi(tama, frame);
        }
        GameEvent::Death => {
            println!("\n💀 OH NO! Your Tamagochi died!");
            display_tamagochi(tama, frame);
        }
        GameEvent::NeedsAttention => {
            println!("\n🔔 Your Tamagochi needs attention!");
        }
        GameEvent::Birthday(days) => {
            println!("\n🎂 Birthday! Your Tamagochi is {} days old!", days);
            display_tamagochi(tama, frame);
        }
    }
}

fn bar(value: u8) -> String {
    let filled = (value as usize) / 10;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
