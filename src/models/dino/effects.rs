use super::{Dino, DeathCause};

/// Effects that can be applied to the Dino
pub enum Effect {
    Damage(f32),
    Kill,
    Eaten,
}

impl Dino {
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
