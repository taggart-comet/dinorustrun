use macroquad::prelude::*;
use super::obstacle::Obstacle;
use super::dino::{ground_y, Effect};

pub struct Rock {
    x_percent: f32,
    size_percent: f32,
    texture: Texture2D,
}

impl Rock {
    pub async fn load_texture() -> Texture2D {
        let texture = load_texture("assets/rock.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        texture
    }

    pub fn new(x: f32, texture: Texture2D) -> Self {
        Self {
            x_percent: x / screen_width(),
            size_percent: 0.125,
            texture,
        }
    }

    fn x(&self) -> f32 {
        screen_width() * self.x_percent
    }

    fn y(&self) -> f32 {
        ground_y() - self.height() + 6.0  // Adjust offset to sit on ground
    }

    fn width(&self) -> f32 {
        screen_height() * self.size_percent
    }

    fn height(&self) -> f32 {
        screen_height() * self.size_percent
    }
}

impl Obstacle for Rock {
    fn update(&mut self, speed: f32, dt: f32) {
        let speed_percent = speed / screen_width();
        self.x_percent -= speed_percent * dt;
    }

    fn draw(&self) {
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
        // draw_rectangle(self.x()+self.width()/4.0, self.y(), self.width()/2.0, self.height()*2.0, RED)
    }

    fn get_hitbox(&self) -> Rect {
        Rect::new(self.x()+self.width()/4.0, self.y(), self.width()/2.0, self.height()*2.0)
    }

    fn is_off_screen(&self) -> bool {
        self.x() + self.width() < 0.0
    }

    fn get_collision_effect(&self, _dino: &crate::models::Dino) -> Effect {
        Effect::Kill
    }
}
