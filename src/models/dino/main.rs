use macroquad::prelude::*;
use crate::models::Dino;

// Ground at 85% of screen height
pub fn ground_y() -> f32 {
    screen_height() * 0.85
}

// Physics scaled to screen height
fn gravity() -> f32 {
    screen_height() * 3.0
}

// Sprite sheet configuration (2x2 grid = 4 frames)
pub(crate) const SPRITE_COLS: usize = 2;
pub(crate) const SPRITE_ROWS: usize = 2;
pub(crate) const FRAME_COUNT: usize = 4;
const ANIMATION_SPEED: f32 = 0.25;
const EATING_ANIMATION_SPEED: f32 = 0.1;
const DOUBLE_JUMP_MANA_COST: f32 = 0.3;


impl Dino {
    pub fn reset(&mut self) {
        let ground = 0.85;
        self.x_percent = 0.15;
        self.y_percent = ground - self.size_percent;
        self.velocity_y = 0.0;
        self.is_jumping = false;
        self.is_flying = false;
        self.is_ducking = false;
        self.is_eating = false;
        self.has_eaten = false;
        self.current_frame = 0;
        self.animation_timer = 0.0;
        self.health = 1.0;
        self.mana = 1.0;
        self.death_cause = None;
        self.can_double_jump = false;
    }

    pub fn x(&self) -> f32 {
        screen_width() * self.x_percent
    }

    pub fn y(&self) -> f32 {
        screen_height() * self.y_percent
    }

    pub fn width(&self) -> f32 {
        screen_height() * self.size_percent
    }

    pub fn height(&self) -> f32 {
        screen_height() * self.size_percent
    }

    pub fn jump(&mut self) {
        if !self.is_jumping {
            self.velocity_y = screen_height() * -1.05;
            self.is_jumping = true;
            self.can_double_jump = true;
            self.current_frame = 0;
            self.animation_timer = 0.0;
        } else if self.can_double_jump && self.mana >= DOUBLE_JUMP_MANA_COST {
            self.velocity_y = screen_height() * -0.8;
            self.mana -= DOUBLE_JUMP_MANA_COST;
            self.can_double_jump = false;
        }
    }

    pub fn duck(&mut self, ducking: bool) {
        self.is_ducking = ducking;
    }

    pub fn fly(&mut self, flying: bool, vertical_direction: f32) {
        if self.is_jumping && flying && self.mana > 0.0 {
            self.is_flying = true;
            self.fly_direction = vertical_direction;
        } else {
            self.is_flying = false;
            self.fly_direction = 0.0;
        }
    }

    pub fn eat(&mut self) {
        if !self.is_eating && !self.is_dead() {
            self.is_eating = true;
            self.has_eaten = false;
            self.current_frame = 0;
            self.animation_timer = 0.0;
        }
    }

    pub fn is_eating(&self) -> bool {
        self.is_eating
    }

    pub fn update(&mut self, dt: f32) {
        let ground = 0.85;
        let current_height = if self.is_ducking {
            self.size_percent * 0.5
        } else {
            self.size_percent
        };
        let ground_level = ground - current_height;

        if self.is_dead() {
            // Update death animation, but stop at last frame
            self.animation_timer += dt;
            if self.animation_timer >= ANIMATION_SPEED {
                self.animation_timer = 0.0;
                if self.current_frame < FRAME_COUNT - 1 {
                    self.current_frame += 1;
                }
            }

            // Fall to the ground if dead while jumping
            if self.is_jumping {
                self.velocity_y += gravity() * dt;
                self.y_percent += (self.velocity_y * dt) / screen_height();

                // Land when falling and reached ground
                if self.velocity_y > 0.0 && self.y_percent >= ground_level {
                    self.y_percent = ground_level;
                    self.velocity_y = 0.0;
                    self.is_jumping = false;
                    self.is_flying = false;
                }
            }
            return;
        }

        if self.is_flying {
            if self.mana > 0.0 {
                self.mana -= 0.1 * dt; // 10% per second
                
                // Smooth vertical movement
                let target_velocity = self.fly_direction * screen_height() * 0.6;
                let lerp_factor = 10.0; // Adjust for smoothness
                self.velocity_y += (target_velocity - self.velocity_y) * lerp_factor * dt;
                self.y_percent += (self.velocity_y * dt) / screen_height();

                // Keep dino within reasonable vertical bounds
                let min_y = 0.05; // 5% from top
                let max_y = ground - current_height;
                if self.y_percent < min_y {
                    self.y_percent = min_y;
                    self.velocity_y = 0.0;
                } else if self.y_percent > max_y {
                    self.y_percent = max_y;
                    self.velocity_y = 0.0;
                }

                if self.mana < 0.0 {
                    self.mana = 0.0;
                    self.is_flying = false;
                }
            } else {
                self.is_flying = false;
            }
        }

        if self.is_eating {
            self.animation_timer += dt;
            if self.animation_timer >= EATING_ANIMATION_SPEED {
                self.animation_timer = 0.0;
                self.current_frame += 1;
                
                let max_frame = if self.has_eaten {
                    FRAME_COUNT - 1
                } else {
                    1
                };

                if self.current_frame > max_frame {
                    self.is_eating = false;
                    self.current_frame = 0;
                }
            }
        }

        if self.is_jumping {
            if !self.is_flying {
                self.velocity_y += gravity() * dt;
                self.y_percent += (self.velocity_y * dt) / screen_height();
            }

            // Land when falling and reached ground
            if self.velocity_y > 0.0 && self.y_percent >= ground_level {
                self.y_percent = ground_level;
                self.velocity_y = 0.0;
                self.is_jumping = false;
                self.is_flying = false;
                self.current_frame = 0;
            }
        }

        // Update animation (both running and jumping)
        if !self.is_eating {
            self.animation_timer += dt;
            if self.animation_timer >= ANIMATION_SPEED {
                self.animation_timer = 0.0;
                self.current_frame = (self.current_frame + 1) % FRAME_COUNT;
            }
        }
    }

    pub fn get_hitbox(&self) -> Circle {
        if self.is_jumping {
            Circle::new(
                self.x() + self.width() / 2.0,
                self.y() + self.height() / 2.0,
                self.width() / 4.0,
            )
        } else {
            Circle::new(
                self.x() + self.width() / 2.0,
                self.y() + self.height() / 1.225,
                self.width() / 4.0,
            )
        }
    }
}
