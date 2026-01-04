use macroquad::prelude::*;

// Sprite sheet is 2x2 grid
const SPRITE_COLS: usize = 2;
const SPRITE_ROWS: usize = 2;

pub struct Cloud {
    // Position as percentage
    pub x_percent: f32,
    y_percent: f32,
    // Speed as percentage of screen width per second
    speed_percent: f32,
    // Size as percentage of screen height
    size_percent: f32,
    // Which sprite from the 2x2 sheet (0-3)
    sprite_index: usize,
    // Texture reference
    texture: Texture2D,
}

impl Cloud {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            x_percent: 1.0 + rand::gen_range(0.0, 0.2),
            y_percent: rand::gen_range(0.05, 0.3),
            speed_percent: rand::gen_range(0.05, 0.06),
            size_percent: rand::gen_range(0.3, 0.4),
            sprite_index: rand::gen_range(0, 4),
            texture,
        }
    }

    fn x(&self) -> f32 {
        screen_width() * self.x_percent
    }

    fn y(&self) -> f32 {
        // Offset to compensate for empty space in sprite above the cloud graphic
        // The cloud graphic sits in the lower portion of each sprite cell
        screen_height() * self.y_percent - self.size() * 0.8
    }

    fn size(&self) -> f32 {
        screen_height() * self.size_percent
    }

    pub fn update(&mut self, dt: f32) {
        self.x_percent -= self.speed_percent * dt;

        // Reset when off screen (left side)
        if self.x_percent < -0.5 {
            self.x_percent = 1.0 + rand::gen_range(0.0, 0.2);
            self.y_percent = rand::gen_range(0.05, 0.3);
            self.size_percent = rand::gen_range(0.3, 0.4);
            self.sprite_index = rand::gen_range(0, 4);
        }
    }

    pub fn draw(&self) {
        // Calculate source rectangle from sprite sheet
        let frame_width = self.texture.width() / SPRITE_COLS as f32;
        let frame_height = self.texture.height() / SPRITE_ROWS as f32;
        let frame_col = self.sprite_index % SPRITE_COLS;
        let frame_row = self.sprite_index / SPRITE_COLS;

        let source = Rect::new(
            frame_col as f32 * frame_width,
            frame_row as f32 * frame_height,
            frame_width,
            frame_height,
        );

        draw_texture_ex(
            &self.texture,
            self.x(),
            self.y(),
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(Vec2::new(self.size() * 2.0, self.size() * 2.0)),
                ..Default::default()
            },
        );
    }
}
