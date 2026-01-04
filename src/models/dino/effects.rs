use super::{Dino, DeathCause};

/// Effects that can be applied to the Dino
pub enum Effect {
    Damage(f32),      // Reduce health by amount
    Heal(f32),        // Restore health by amount
    DrainMana(f32),   // Reduce mana by amount
    RestoreMana(f32), // Restore mana by amount
    Kill,             // Instant death - zero health and mana
    Eaten,            // Recover 0.1 health and 0.1 mana
}

impl Dino {
    /// Apply an effect to the dino
    pub fn apply_effect(&mut self, effect: Effect) {
        match effect {
            Effect::Damage(amount) => {
                self.health = (self.health - amount).max(0.0);
                if self.is_dead() && self.death_cause.is_none() {
                    self.death_cause = Some(DeathCause::NoHP);
                    self.current_frame = 0;
                    self.animation_timer = 0.0;
                    self.is_ducking = false;
                }
            }
            Effect::Heal(amount) => {
                self.health = (self.health + amount).min(1.0);
            }
            Effect::DrainMana(amount) => {
                self.mana = (self.mana - amount).max(0.0);
            }
            Effect::RestoreMana(amount) => {
                self.mana = (self.mana + amount).min(1.0);
            }
            Effect::Kill => {
                self.health = 0.0;
                self.mana = 0.0;
                if self.death_cause.is_none() {
                    self.death_cause = Some(DeathCause::Impact);
                    self.current_frame = 0;
                    self.animation_timer = 0.0;
                    self.is_ducking = false;
                }
            }
            Effect::Eaten => {
                self.health = (self.health + 0.2).min(1.0);
                self.mana = (self.mana + 0.3).min(1.0);
                self.has_eaten = true;
                // If we hit something while in the first half of eating animation, 
                // skip to the second half
                if self.is_eating && self.current_frame < 2 {
                    self.current_frame = 2;
                    self.animation_timer = 0.0;
                }
            }
        }
    }

    /// Check if dino is dead
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Check if death animation is finished
    pub fn death_animation_finished(&self) -> bool {
        self.is_dead() && self.current_frame >= super::main::FRAME_COUNT - 1 && !self.is_jumping
    }
}
