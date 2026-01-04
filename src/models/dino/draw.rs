use macroquad::prelude::*;
use crate::models::Dino;
use super::main::{SPRITE_COLS, SPRITE_ROWS};
use super::DeathCause;

impl Dino {
    pub fn draw(&self) {
        if self.is_dead() {
            self._death();
            return;
        }
        if self.is_eating {
            self._eat();
            return;
        }
        if self.is_flying {
            self._fly();
            return;
        }
        if self.is_jumping {
            self._jump();
            return;
        }
        self._run();
    }

    fn _fly(&self) {
        let scale = 2.0; // Adjust this value: 1.0 = normal, 1.2 = 20% larger, 0.8 = 20% smaller
        draw_texture_ex(
            &self.flight_texture,
            self.x() - self.width() * scale / 4.0,
            self.y() - self.height() * scale / 5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.width() * scale, self.height() * scale)),
                ..Default::default()
            },
        );
    }

    fn _death(&self) {
        let texture = match self.death_cause {
            Some(DeathCause::Impact) => &self.death_impact_texture,
            Some(DeathCause::NoHP) => &self.death_no_hp_texture,
            None => &self.death_no_hp_texture, // Default fallback
        };

        let frame_col = self.current_frame % SPRITE_COLS;
        let frame_row = self.current_frame / SPRITE_COLS;

        let frame_width = texture.width() / SPRITE_COLS as f32;
        let frame_height = texture.height() / SPRITE_ROWS as f32;

        let source_rect = Rect::new(
            frame_col as f32 * frame_width,
            frame_row as f32 * frame_height - 35.0,
            frame_width + 5.0,
            frame_height,
        );

        draw_texture_ex(
            texture,
            self.x(),
            self.y(),
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(Vec2::new(self.width(), self.height())),
                ..Default::default()
            },
        );
    }

    fn _eat(&self) {
        let frame_col = self.current_frame % SPRITE_COLS;
        let frame_row = self.current_frame / SPRITE_COLS;

        let frame_width = self.eating_texture.width() / SPRITE_COLS as f32;
        let frame_height = self.eating_texture.height() / SPRITE_ROWS as f32;

        let source_rect = Rect::new(
            frame_col as f32 * frame_width,
            frame_row as f32 * frame_height - 35.0,
            frame_width,
            frame_height,
        );

        draw_texture_ex(
            &self.eating_texture,
            self.x(),
            self.y(),
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(Vec2::new(self.width(), self.height())),
                ..Default::default()
            },
        );
    }

    fn _run(&self) {
        let frame_col = self.current_frame % SPRITE_COLS;
        let frame_row = self.current_frame / SPRITE_COLS;

        let frame_width = self.run_texture.width() / SPRITE_COLS as f32;
        let frame_height = self.run_texture.height() / SPRITE_ROWS as f32;

        let source_rect = Rect::new(
            frame_col as f32 * frame_width,
            frame_row as f32 * frame_height - 35.0,
            frame_width - 5.0,
            frame_height,
        );

        draw_texture_ex(
            if self.is_ducking { &self.duck_texture } else { &self.run_texture },
            self.x(),
            self.y(),
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(Vec2::new(self.width(), self.height())),
                ..Default::default()
            },
        );
        // Circle::new(
        //     self.x() + self.width() / 2.0,
        //     self.y() + self.height() / 1.225,
        //     self.width() / 4.0,
        // )
        // draw debug figure for hitbox as it's circle (not rectangle)
        // draw_circle(self.x() + self.width() / 2.0, self.y() + self.height() / 1.225, self.width() / 4.0, RED);
    }

    fn _jump(&self) {
        let frame_col = self.current_frame % SPRITE_COLS;
        let frame_row = self.current_frame / SPRITE_COLS;

        let frame_width = self.jump_texture.width() / SPRITE_COLS as f32;
        let frame_height = self.jump_texture.height() / SPRITE_ROWS as f32;

        let source_rect = Rect::new(
            frame_col as f32 * frame_width,
            frame_row as f32 * frame_height,
            frame_width,
            frame_height,
        );

        draw_texture_ex(
            &self.jump_texture,
            self.x(),
            self.y(),
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(Vec2::new(self.width(), self.height())),
                ..Default::default()
            },
        );
    }
}
