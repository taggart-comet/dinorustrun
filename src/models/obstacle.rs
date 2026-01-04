use super::Dino;
use macroquad::prelude::Rect;
use super::dino::Effect;

/// Trait for all obstacles in the game (rocks, birds, etc.)
pub trait Obstacle {
    fn update(&mut self, speed: f32, dt: f32);
    fn draw(&self);
    fn get_hitbox(&self) -> Rect;
    fn is_off_screen(&self) -> bool;
    fn get_collision_effect(&self, dino: &Dino) -> Effect;

    fn on_hit(&mut self) {}
    fn should_remove(&self) -> bool { false }
}
