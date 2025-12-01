//! Tamagochi Classic - Platform-agnostic Core Logic
//! 
//! This module contains the pure game logic that works on any platform:
//! - Desktop (for development/testing)
//! - ESP32 (embedded target)
//! - Future platforms (WASM, mobile, etc.)

#![cfg_attr(not(test), no_std)]

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

/// Main Tamagochi state representing the digital pet
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Tamagochi {
    /// Hunger level (0 = starving, 100 = full)
    pub hunger: u8,
    
    /// Happiness level (0 = very sad, 100 = very happy)
    pub happiness: u8,
    
    /// Health level (0 = dead, 100 = perfect health)
    pub health: u8,
    
    /// Age in game ticks (implementation-dependent time unit)
    pub age: u32,
    
    /// Current life stage
    pub stage: LifeStage,
    
    /// Whether the Tamagochi is alive
    pub alive: bool,
    
    /// Evolution quality score (affects which adult form)
    pub care_quality: u8,
}

/// Life stages of the Tamagochi
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum LifeStage {
    /// Freshly hatched egg
    Egg,
    /// Newborn (0-24 hours)
    Baby,
    /// Child stage (1-3 days)
    Child,
    /// Teenager (3-7 days)
    Teenager,
    /// Adult (7+ days) - final form depends on care
    Adult(AdultForm),
}

/// Different adult forms based on care quality
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum AdultForm {
    /// Poor care (care_quality < 30)
    Neglected,
    /// Average care (care_quality 30-70)
    Normal,
    /// Excellent care (care_quality > 70)
    Premium,
}

/// Events that can occur during gameplay
#[derive(Clone, Copy, Debug)]
pub enum GameEvent {
    /// Tamagochi evolved to a new stage
    Evolution(LifeStage),
    /// Tamagochi died
    Death,
    /// Needs attention (hunger/happiness critical)
    NeedsAttention,
    /// Birthday/age milestone
    Birthday(u32),
}

impl Tamagochi {
    /// Create a new Tamagochi in egg stage
    pub fn new() -> Self {
        Self {
            hunger: 50,
            happiness: 50,
            health: 100,
            age: 0,
            stage: LifeStage::Egg,
            alive: true,
            care_quality: 50,
        }
    }

    /// Feed the Tamagochi
    /// 
    /// Reduces hunger, slightly increases happiness.
    /// Overfeeding can reduce happiness.
    pub fn feed(&mut self) {
        if !self.alive {
            return;
        }

        if self.hunger > 20 {
            // Normal feeding
            self.hunger = self.hunger.saturating_sub(20);
            self.happiness = self.happiness.saturating_add(5).min(100);
            self.update_care_quality(2);
        } else {
            // Overfeeding - slight penalty
            self.hunger = 0;
            self.happiness = self.happiness.saturating_sub(3);
        }
    }

    /// Play with the Tamagochi
    /// 
    /// Increases happiness significantly but also increases hunger.
    pub fn play(&mut self) {
        if !self.alive {
            return;
        }

        self.happiness = self.happiness.saturating_add(15).min(100);
        self.hunger = self.hunger.saturating_add(5).min(100);
        self.update_care_quality(3);
    }

    /// Give medicine to the Tamagochi
    /// 
    /// Restores health but may reduce happiness.
    pub fn medicine(&mut self) {
        if !self.alive {
            return;
        }

        self.health = self.health.saturating_add(30).min(100);
        self.happiness = self.happiness.saturating_sub(5);
        self.update_care_quality(1);
    }

    /// Clean/care for the Tamagochi
    /// 
    /// Small happiness boost, demonstrates good care.
    pub fn clean(&mut self) {
        if !self.alive {
            return;
        }

        self.happiness = self.happiness.saturating_add(10).min(100);
        self.update_care_quality(2);
    }

    /// Update game state (call this periodically, e.g., every second)
    /// 
    /// Returns any events that occurred during this tick.
    pub fn update(&mut self) -> Option<GameEvent> {
        if !self.alive {
            return None;
        }

        self.age = self.age.saturating_add(1);

        // Natural stat degradation
        self.hunger = self.hunger.saturating_add(1);
        self.happiness = self.happiness.saturating_sub(1);

        // Health degradation based on needs
        if self.hunger > 80 {
            self.health = self.health.saturating_sub(2);
            self.update_care_quality(-5);
        }
        if self.happiness < 20 {
            self.health = self.health.saturating_sub(1);
            self.update_care_quality(-3);
        }

        // Check for evolution
        if let Some(new_stage) = self.check_evolution() {
            self.stage = new_stage;
            return Some(GameEvent::Evolution(new_stage));
        }

        // Check for death
        if self.health == 0 {
            self.alive = false;
            return Some(GameEvent::Death);
        }

        // Check if needs attention
        if self.hunger > 80 || self.happiness < 20 || self.health < 30 {
            return Some(GameEvent::NeedsAttention);
        }

        // Birthday milestones (every 24 "hours" = 86400 ticks if tick = 1 second)
        if self.age > 0 && self.age % 86400 == 0 {
            return Some(GameEvent::Birthday(self.age / 86400));
        }

        None
    }

    /// Check if Tamagochi should evolve to next stage
    fn check_evolution(&mut self) -> Option<LifeStage> {
        match self.stage {
            LifeStage::Egg if self.age >= 3600 => {
                // Egg hatches after 1 hour (3600 ticks)
                Some(LifeStage::Baby)
            }
            LifeStage::Baby if self.age >= 86400 => {
                // Baby becomes child after 24 hours
                Some(LifeStage::Child)
            }
            LifeStage::Child if self.age >= 259200 => {
                // Child becomes teenager after 3 days
                Some(LifeStage::Teenager)
            }
            LifeStage::Teenager if self.age >= 604800 => {
                // Teenager becomes adult after 7 days
                let form = match self.care_quality {
                    0..=30 => AdultForm::Neglected,
                    31..=70 => AdultForm::Normal,
                    _ => AdultForm::Premium,
                };
                Some(LifeStage::Adult(form))
            }
            _ => None,
        }
    }

    /// Update care quality score
    fn update_care_quality(&mut self, delta: i16) {
        let new_quality = (self.care_quality as i16) + delta;
        self.care_quality = new_quality.clamp(0, 100) as u8;
    }

    /// Get current needs status
    pub fn needs_status(&self) -> NeedsStatus {
        NeedsStatus {
            hunger_critical: self.hunger > 80,
            happiness_critical: self.happiness < 20,
            health_critical: self.health < 30,
        }
    }

    /// Get a summary of the Tamagochi's current state
    pub fn status_summary(&self) -> StatusSummary {
        StatusSummary {
            alive: self.alive,
            stage: self.stage,
            age_days: self.age / 86400,
            hunger_level: self.hunger,
            happiness_level: self.happiness,
            health_level: self.health,
            care_rating: self.care_quality,
        }
    }
}

impl Default for Tamagochi {
    fn default() -> Self {
        Self::new()
    }
}

/// Status of critical needs
#[derive(Clone, Copy, Debug)]
pub struct NeedsStatus {
    pub hunger_critical: bool,
    pub happiness_critical: bool,
    pub health_critical: bool,
}

/// High-level status summary
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct StatusSummary {
    pub alive: bool,
    pub stage: LifeStage,
    pub age_days: u32,
    pub hunger_level: u8,
    pub happiness_level: u8,
    pub health_level: u8,
    pub care_rating: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tamagochi() {
        let tama = Tamagochi::new();
        assert_eq!(tama.stage, LifeStage::Egg);
        assert!(tama.alive);
        assert_eq!(tama.age, 0);
    }

    #[test]
    fn test_feeding() {
        let mut tama = Tamagochi::new();
        let initial_hunger = tama.hunger;
        tama.feed();
        assert!(tama.hunger < initial_hunger);
    }

    #[test]
    fn test_evolution() {
        let mut tama = Tamagochi::new();
        tama.age = 3600; // 1 hour
        let event = tama.update();
        assert!(matches!(event, Some(GameEvent::Evolution(LifeStage::Baby))));
        assert_eq!(tama.stage, LifeStage::Baby);
    }

    #[test]
    fn test_death_from_health() {
        let mut tama = Tamagochi::new();
        tama.health = 1;
        tama.hunger = 90; // Will reduce health by 2
        let event = tama.update();
        assert!(matches!(event, Some(GameEvent::Death)));
        assert!(!tama.alive);
    }

    #[test]
    fn test_care_quality_affects_adult_form() {
        let mut tama = Tamagochi::new();
        tama.age = 604799; // Just before adult evolution
        tama.stage = LifeStage::Teenager; // Set to teenager stage
        tama.care_quality = 80; // High care
        let event = tama.update(); // Trigger evolution
        assert!(matches!(event, Some(GameEvent::Evolution(LifeStage::Adult(AdultForm::Premium)))));
        assert!(matches!(tama.stage, LifeStage::Adult(AdultForm::Premium)));
    }
}
