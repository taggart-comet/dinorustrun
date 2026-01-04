mod main;
mod draw;
mod stats;
mod effects;

use macroquad::miniquad::FilterMode;
use macroquad::prelude::{load_texture, Texture2D};
pub use main::ground_y;
pub use effects::Effect;

#[derive(Clone, Copy, PartialEq)]
pub enum DeathCause {
    Impact,
    NoHP,
}

pub struct Dino {
    // Position as percentage (0.0 - 1.0)
    pub x_percent: f32,
    pub y_percent: f32,
    // Size as percentage of screen height
    pub size_percent: f32,
    pub(crate) velocity_y: f32,
    pub(crate) is_jumping: bool,
    pub(crate) is_flying: bool,
    pub(crate) fly_direction: f32,
    pub(crate) is_ducking: bool,
    pub(crate) is_eating: bool,
    pub(crate) has_eaten: bool,
    pub(crate) current_frame: usize,
    pub(crate) animation_timer: f32,
    // Textures owned by Dino
    pub(crate) run_texture: Texture2D,
    pub(crate) jump_texture: Texture2D,
    pub(crate) flight_texture: Texture2D,
    pub(crate) duck_texture: Texture2D,
    pub(crate) eating_texture: Texture2D,
    pub(crate) death_impact_texture: Texture2D,
    pub(crate) death_no_hp_texture: Texture2D,
    // Stats
    pub(crate) health: f32,  // 0.0 - 1.0
    pub(crate) mana: f32,    // 0.0 - 1.0
    pub(crate) death_cause: Option<DeathCause>,
    pub(crate) can_double_jump: bool,
}

impl Dino {
    pub async fn new() -> Self {
        let size_percent = 0.25;
        let ground = 0.85;

        let run_texture = load_texture("assets/dino/run.png").await.unwrap();
        run_texture.set_filter(FilterMode::Nearest);

        let jump_texture = load_texture("assets/dino/jump.png").await.unwrap();
        jump_texture.set_filter(FilterMode::Nearest);

        let flight_texture = load_texture("assets/dino/flight_one.png").await.unwrap();
        flight_texture.set_filter(FilterMode::Nearest);

        let duck_texture = load_texture("assets/dino/low_run.png").await.unwrap();
        duck_texture.set_filter(FilterMode::Nearest);

        let eating_texture = load_texture("assets/dino/eating.png").await.unwrap();
        eating_texture.set_filter(FilterMode::Nearest);

        let death_impact_texture = load_texture("assets/dino/death_impact.png").await.unwrap();
        death_impact_texture.set_filter(FilterMode::Nearest);

        let death_no_hp_texture = load_texture("assets/dino/death_no_hp.png").await.unwrap();
        death_no_hp_texture.set_filter(FilterMode::Nearest);

        Self {
            x_percent: 0.15,
            y_percent: ground - size_percent,
            size_percent,
            velocity_y: 0.0,
            is_jumping: false,
            is_flying: false,
            fly_direction: 0.0,
            is_ducking: false,
            is_eating: false,
            has_eaten: false,
            current_frame: 0,
            animation_timer: 0.0,
            run_texture,
            jump_texture,
            flight_texture,
            duck_texture,
            eating_texture,
            death_impact_texture,
            death_no_hp_texture,
            health: 1.0,
            mana: 1.0,
            death_cause: None,
            can_double_jump: false,
        }
    }
}