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

    /// Called when dino collides with this obstacle
    fn on_hit(&mut self) {}

    /// Returns true if obstacle should be removed (e.g., after hit animation)
    fn should_remove(&self) -> bool { false }
}
