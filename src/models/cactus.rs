use macroquad::prelude::*;
use super::obstacle::Obstacle;
use super::dino::{ground_y, Effect};

// Hit animation sprite sheet (2x2 grid = 4 frames)
const SPRITE_COLS: usize = 2;
const SPRITE_ROWS: usize = 2;
const FRAME_COUNT: usize = 4;
const HIT_FRAME_SPEED: f32 = 0.15;

pub struct Cactus {
    x_percent: f32,
    size_percent: f32,
    texture: Texture2D,
    hit_texture: Texture2D,
    is_hit: bool,
    hit_frame: usize,
    hit_timer: f32,
}

impl Cactus {
    pub fn new(x: f32, texture: Texture2D, hit_texture: Texture2D) -> Self {
        Self {
            x_percent: x / screen_width(),
            size_percent: 0.15,
            texture,
            hit_texture,
            is_hit: false,
            hit_frame: 0,
            hit_timer: 0.0,
        }
    }

    fn x(&self) -> f32 {
        screen_width() * self.x_percent
    }

    fn y(&self) -> f32 {
        ground_y() - self.height() + 13.0  // Adjust offset to sit on ground
    }

    fn width(&self) -> f32 {
        let aspect_ratio = self.texture.width() / self.texture.height();
        self.height() * aspect_ratio
    }

    fn height(&self) -> f32 {
        screen_height() * self.size_percent
    }
}

impl Obstacle for Cactus {
    fn update(&mut self, speed: f32, dt: f32) {
        let speed_percent = speed / screen_width();
        self.x_percent -= speed_percent * dt;

        // Progress hit animation
        if self.is_hit {
            self.hit_timer += dt;
            if self.hit_timer >= HIT_FRAME_SPEED {
                self.hit_timer = 0.0;
                self.hit_frame += 1;
            }
        }
    }

    fn draw(&self) {
        if self.is_hit {
            // Draw hit animation from sprite sheet
            let frame_col = self.hit_frame % SPRITE_COLS;
            let frame_row = self.hit_frame / SPRITE_COLS;

            let frame_width = self.hit_texture.width() / SPRITE_COLS as f32;
            let frame_height = self.hit_texture.height() / SPRITE_ROWS as f32;

            let source_rect = Rect::new(
                frame_col as f32 * frame_width,
                frame_row as f32 * frame_height,
                frame_width,
                frame_height,
            );

            draw_texture_ex(
                &self.hit_texture,
                self.x(),
                self.y(),
                WHITE,
                DrawTextureParams {
                    source: Some(source_rect),
                    dest_size: Some(Vec2::new(self.width(), self.height())),
                    ..Default::default()
                },
            );
        } else {
            // Draw normal cactus
            draw_texture_ex(
                &self.texture,
                self.x(),
                self.y(),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.width(), self.height())),
                    ..Default::default()
                },
            );
        }
    }

    fn get_hitbox(&self) -> Rect {
        // No hitbox when already hit (prevent double-hit)
        if self.is_hit {
            Rect::new(0.0, 0.0, 0.0, 0.0)
        } else {
            Rect::new(self.x()+self.width()/3.5, self.y(), self.width()/2.5, self.height() - self.height()*0.1)
        }
    }

    fn is_off_screen(&self) -> bool {
        self.x() + self.width() < 0.0
    }

    fn get_collision_effect(&self, dino: &crate::models::Dino) -> Effect {
        Effect::Damage(0.15)
    }

    fn on_hit(&mut self) {
        self.is_hit = true;
        self.hit_timer = 0.0;
    }

    fn should_remove(&self) -> bool {
        self.is_hit && self.hit_frame >= FRAME_COUNT
    }
}
