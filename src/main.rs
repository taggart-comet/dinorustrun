use macroquad::prelude::*;

mod models;

use models::{Cloud, Dino, Fly, Obstacle, Rock, Cactus, ground_y, dino::DeathCause};

// Game constants (as percentages where applicable)
const GAME_SPEED_PERCENT: f32 = 0.2;       // 40% of screen width per second
const GAME_SPEED_INCREMENT: f32 = 0.015;   // Speed increase per second
const OBSTACLE_SPAWN_TIME: f32 = 1.5;

// Game state enum
enum GameState {
    Ready,
    Playing,
    GameOver,
}

// Main game structure
struct Game {
    dino: Dino,
    obstacles: Vec<Box<dyn Obstacle>>,
    clouds: Vec<Cloud>,
    score: u32,
    high_score: u32,
    game_speed_percent: f32,
    ground_displacement: f32,
    spawn_timer: f32,
    state: GameState,
    // Shared textures for obstacles
    bird_texture: Texture2D,
    rock_texture: Texture2D,
    cactus_texture: Texture2D,
    cactus_hit_texture: Texture2D,
    bird_hit_texture: Texture2D,
}

impl Game {
    async fn new() -> Self {
        // Load cloud texture first so we can pass it to clouds
        let cloud_texture = load_texture("assets/env/clouds.png").await.unwrap();
        cloud_texture.set_filter(FilterMode::Nearest);

        let clouds = (0..5)
            .map(|_| {
                let mut cloud = Cloud::new(cloud_texture.clone());
                cloud.x_percent = rand::gen_range(0.0, 1.0);
                cloud
            })
            .collect();

        // Load shared textures
        let bird_texture = load_texture("assets/bird.png").await.unwrap();
        bird_texture.set_filter(FilterMode::Nearest);

        let rock_texture = load_texture("assets/rock.png").await.unwrap();
        rock_texture.set_filter(FilterMode::Nearest);

        let cactus_texture = load_texture("assets/cactus.png").await.unwrap();
        cactus_texture.set_filter(FilterMode::Nearest);

        let cactus_hit_texture = load_texture("assets/cactus_hit.png").await.unwrap();
        cactus_hit_texture.set_filter(FilterMode::Nearest);

        let bird_hit_texture = load_texture("assets/bird_hit.png").await.unwrap();
        bird_hit_texture.set_filter(FilterMode::Nearest);

        Self {
            dino: Dino::new().await,
            obstacles: Vec::new(),
            clouds,
            score: 0,
            high_score: 0,
            game_speed_percent: GAME_SPEED_PERCENT,
            ground_displacement: 0.0,
            spawn_timer: OBSTACLE_SPAWN_TIME,
            state: GameState::Ready,
            bird_texture,
            rock_texture,
            cactus_texture,
            cactus_hit_texture,
            bird_hit_texture,
        }
    }

    fn reset(&mut self) {
        self.dino.reset();
        self.obstacles.clear();
        self.score = 0;
        self.game_speed_percent = GAME_SPEED_PERCENT;
        self.ground_displacement = 0.0;
        self.spawn_timer = OBSTACLE_SPAWN_TIME;
        self.state = GameState::Playing;
    }

    // Convert percentage speed to pixel speed
    fn game_speed(&self) -> f32 {
        screen_width() * self.game_speed_percent
    }

    async fn spawn_obstacle(&mut self) {
        let obstacle_type = rand::gen_range(0, 10);
        let spawn_x = screen_width() + screen_width() * 0.05;
        let obstacle: Box<dyn Obstacle> = match obstacle_type {
            0..=3 => Box::new(Rock::new(spawn_x, self.rock_texture.clone())),
            4..=6 => Box::new(Cactus::new(spawn_x, self.cactus_texture.clone(), self.cactus_hit_texture.clone())),
            _ => Box::new(Fly::new(spawn_x, self.bird_texture.clone(), self.bird_hit_texture.clone())),
        };
        self.obstacles.push(obstacle);
    }

    async fn update(&mut self) {
        let dt = get_frame_time();

        match self.state {
            GameState::Ready => {
                for cloud in &mut self.clouds {
                    cloud.update(dt);
                }

                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) {
                    self.state = GameState::Playing;
                    self.dino.jump();
                }
            }
            GameState::Playing => {
                // Handle input
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) {
                    self.dino.jump();
                }
                self.dino.duck(is_key_down(KeyCode::Down));
                if is_key_pressed(KeyCode::E) {
                    self.dino.eat();
                }
                
                let mut fly_dir = 0.0;
                if is_key_down(KeyCode::Up) {
                    fly_dir -= 1.0;
                }
                if is_key_down(KeyCode::Down) {
                    fly_dir += 1.0;
                }
                self.dino.fly(is_key_down(KeyCode::F), fly_dir);

                // Update dino
                self.dino.update(dt);

                // Determine if world should update
                let should_update_world = if !self.dino.is_dead() {
                    true
                } else {
                    // Dino is dead, check cause
                    match self.dino.death_cause {
                        Some(DeathCause::NoHP) => !self.dino.death_animation_finished(),
                        _ => false, // Impact or unknown - stop immediately
                    }
                };

                if should_update_world {
                    // Update obstacles
                    let speed = self.game_speed();
                    for obstacle in &mut self.obstacles {
                        obstacle.update(speed, dt);
                    }

                    // Update ground displacement
                    self.ground_displacement += speed * dt;
                    if self.ground_displacement >= screen_width() {
                        self.ground_displacement -= screen_width();
                    }

                    // Remove off-screen obstacles and finished hit animations
                    self.obstacles.retain(|o| !o.is_off_screen() && !o.should_remove());

                    // Spawn new obstacles (only if dino is actually alive, not just in death animation)
                    if !self.dino.is_dead() {
                        self.spawn_timer -= dt;
                        if self.spawn_timer <= 0.0 {
                            self.spawn_obstacle().await;
                            let min_time = (OBSTACLE_SPAWN_TIME - self.game_speed_percent).max(0.5);
                            self.spawn_timer = rand::gen_range(min_time, min_time + 1.0);
                        }

                        // Check collisions and apply effects
                        let dino_hitbox = self.dino.get_hitbox();
                        for obstacle in &mut self.obstacles {
                            if dino_hitbox.overlaps_rect(&obstacle.get_hitbox()) {
                                let effect = obstacle.get_collision_effect(&self.dino);
                                self.dino.apply_effect(effect);
                                obstacle.on_hit();
                            }
                        }

                        // Update score and speed
                        self.score += 1;
                        self.game_speed_percent += GAME_SPEED_INCREMENT * dt;
                    }
                }

                // Always update clouds
                for cloud in &mut self.clouds {
                    cloud.update(dt);
                }

                // Check if dino is dead and animation finished
                if self.dino.is_dead() && self.dino.death_animation_finished() {
                    self.state = GameState::GameOver;
                    if self.score > self.high_score {
                        self.high_score = self.score;
                    }
                }
            }
            GameState::GameOver => {
                for cloud in &mut self.clouds {
                    cloud.update(dt);
                }

                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::R) {
                    self.reset();
                }
            }
        }
    }

    fn draw(&self) {
        clear_background(SKYBLUE);

        // Draw clouds
        for cloud in &self.clouds {
            cloud.draw();
        }

        // Draw ground (at 85% of screen height)
        let ground = ground_y();
        draw_rectangle(0.0, ground, screen_width(), screen_height() - ground, BEIGE);
        draw_line(0.0, ground, screen_width(), ground, 2.0, DARKBROWN);

        // Draw ground texture (scaled)
        let pebble_size = screen_height() * 0.006;
        let spacing = screen_width() * 0.025;
        for i in 0..((screen_width() / spacing) as i32 + 1) {
            let x = (i as f32 * spacing + self.ground_displacement) % screen_width();
            if rand::gen_range(0, 5) == 0 {
                draw_rectangle(screen_width() - x, ground + pebble_size, pebble_size, pebble_size, GRAY);
            }
        }

        // Draw obstacles
        for obstacle in &self.obstacles {
            obstacle.draw();
        }

        // Draw dino
        self.dino.draw();

        // Draw dino stats below ground
        self.dino.draw_stats();

        // Draw score (scaled font size)
        let font_large = screen_height() * 0.05;
        let font_small = screen_height() * 0.035;
        let margin = screen_width() * 0.02;

        draw_text(&format!("Score: {}", self.score), margin, font_large * 1.2, font_large, DARKGRAY);
        draw_text(
            &format!("High Score: {}", self.high_score),
            margin,
            font_large * 1.2 + font_small * 1.2,
            font_small,
            GRAY,
        );

        // Draw state-specific UI
        match self.state {
            GameState::Ready => {
                let font_title = screen_height() * 0.06;
                let font_sub = screen_height() * 0.03;
                let center_x = screen_width() / 2.0;
                let start_y = screen_height() * 0.4;

                let text = "Press SPACE or UP to start!";
                let text_width = measure_text(text, None, font_title as u16, 1.0).width;
                draw_text(
                    text,
                    center_x - text_width / 2.0,
                    start_y,
                    font_title,
                    DARKGRAY,
                );

                let controls = [
                    "SPACE / UP - Jump (double jump costs mana)",
                    "DOWN - Duck",
                    "F - Fly (hold while jumping, costs mana)",
                    "E - Eat (catch flies to restore mana)",
                ];
                for (i, line) in controls.iter().enumerate() {
                    let line_width = measure_text(line, None, font_sub as u16, 1.0).width;
                    draw_text(
                        line,
                        center_x - line_width / 2.0,
                        start_y + font_title + (i as f32 + 1.0) * font_sub * 1.3,
                        font_sub,
                        GRAY,
                    );
                }
            }
            GameState::GameOver => {
                let font_title = screen_height() * 0.1;
                let font_sub = screen_height() * 0.04;
                let font_controls = screen_height() * 0.025;
                let center_x = screen_width() / 2.0;

                let text = "GAME OVER";
                let text_width = measure_text(text, None, font_title as u16, 1.0).width;
                draw_text(
                    text,
                    center_x - text_width / 2.0,
                    screen_height() * 0.35,
                    font_title,
                    RED,
                );

                let restart = "Press SPACE or R to restart";
                let restart_width = measure_text(restart, None, font_sub as u16, 1.0).width;
                draw_text(
                    restart,
                    center_x - restart_width / 2.0,
                    screen_height() * 0.45,
                    font_sub,
                    DARKGRAY,
                );

                let controls = [
                    "SPACE / UP - Jump (double jump costs mana)",
                    "DOWN - Duck",
                    "F - Fly (hold while jumping, costs mana)",
                    "E - Eat (catch flies to restore mana)",
                ];
                for (i, line) in controls.iter().enumerate() {
                    let line_width = measure_text(line, None, font_controls as u16, 1.0).width;
                    draw_text(
                        line,
                        center_x - line_width / 2.0,
                        screen_height() * 0.55 + (i as f32) * font_controls * 1.3,
                        font_controls,
                        GRAY,
                    );
                }
            }
            GameState::Playing => {}
        }
    }
}

#[macroquad::main("Dino Run")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.update().await;
        game.draw();
        next_frame().await;
    }
}
