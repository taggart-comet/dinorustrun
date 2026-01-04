use macroquad::prelude::*;
use super::Dino;
use super::ground_y;

const BAR_WIDTH_PERCENT: f32 = 0.25;
const BAR_HEIGHT_PERCENT: f32 = 0.02;
const BAR_SPACING_PERCENT: f32 = 0.025;

impl Dino {
    pub fn draw_stats(&self) {
        let bar_width = screen_width() * BAR_WIDTH_PERCENT;
        let bar_height = screen_height() * BAR_HEIGHT_PERCENT;
        let spacing = screen_height() * BAR_SPACING_PERCENT;
        let margin = screen_width() * 0.02;
        let start_y = ground_y() + spacing*2f32;

        // Health bar
        self.draw_bar(
            margin,
            start_y,
            bar_width,
            bar_height,
            self.health,
            RED,
            "HP",
        );

        // Mana bar
        self.draw_bar(
            margin,
            start_y + bar_height + spacing,
            bar_width,
            bar_height,
            self.mana,
            BLUE,
            "MP",
        );
    }

    fn draw_bar(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        percentage: f32,
        color: Color,
        label: &str,
    ) {
        // Background (empty bar)
        draw_rectangle(x, y, width, height, LIGHTGRAY);

        // Filled portion
        draw_rectangle(x, y, width * percentage.clamp(0.0, 1.0), height, color);

        // Border
        draw_rectangle_lines(x, y, width, height, 2.0, DARKGRAY);

        // Label
        let font_size = height * 1.2;
        draw_text(label, x + width + 5.0, y + height * 0.8, font_size, DARKGRAY);
    }
}
