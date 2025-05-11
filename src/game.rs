use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, DrawParam, Rect, Color, Mesh, Text, DrawMode};
use ggez::mint::Point2;

use crate::dinosaur::Dinosaur;
use crate::obstacle::Obstacle;

pub struct Game {
    dinosaur: Dinosaur,
    obstacles: Vec<Obstacle>,
    score: u32,
    game_over: bool,
    ground_y: f32,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Game> {
        let ground_y = 250.0; // Ground level
        Ok(Game {
            dinosaur: Dinosaur::new(50.0, ground_y),
            obstacles: Vec::new(),
            score: 0,
            game_over: false,
            ground_y,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            return Ok(());
        }

        // Update dinosaur
        self.dinosaur.update(ctx)?;

        // Update obstacles
        for obstacle in &mut self.obstacles {
            obstacle.update(ctx)?;
        }

        // Remove obstacles that are off screen
        self.obstacles.retain(|obstacle| obstacle.x > -50.0);

        // Generate new obstacles
        if self.obstacles.is_empty() || 
           self.obstacles.last().unwrap().x < 400.0 {
            self.obstacles.push(Obstacle::new(800.0, self.ground_y));
        }

        // Check collisions
        for obstacle in &self.obstacles {
            if self.check_collision(&self.dinosaur, obstacle) {
                self.game_over = true;
                break;
            }
        }

        // Update score
        self.score += 1;

        Ok(())
    }

    pub fn draw_game(&self, ctx: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> GameResult {
        use ggez::graphics::{DrawParam, Rect, Color, Mesh, Text, DrawMode};
        use ggez::mint::Point2;
        // Draw ground
        let ground = Rect::new(0.0, self.ground_y, 800.0, 2.0);
        let ground_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            ground,
            Color::BLACK,
        )?;
        canvas.draw(&ground_mesh, DrawParam::default());

        // Draw dinosaur
        self.dinosaur.draw(ctx, canvas)?;

        // Draw obstacles
        for obstacle in &self.obstacles {
            obstacle.draw(ctx, canvas)?;
        }

        // Draw score
        let score_text = format!("Score: {}", self.score);
        let score_display = Text::new(score_text);
        canvas.draw(
            &score_display,
            DrawParam::default()
                .dest(Point2 { x: 10.0, y: 10.0 })
                .color(Color::BLACK),
        );

        // Draw game over text
        if self.game_over {
            let game_over_text = Text::new("Game Over! Press R to restart");
            canvas.draw(
                &game_over_text,
                DrawParam::default()
                    .dest(Point2 { x: 300.0, y: 150.0 })
                    .color(Color::RED),
            );
        }

        Ok(())
    }

    pub fn jump(&mut self) {
        if !self.game_over {
            self.dinosaur.jump();
        }
    }

    fn check_collision(&self, dinosaur: &Dinosaur, obstacle: &Obstacle) -> bool {
        // Simple rectangle collision detection
        let dino_rect = Rect::new(
            dinosaur.x,
            dinosaur.y,
            dinosaur.width,
            dinosaur.height,
        );
        let obstacle_rect = Rect::new(
            obstacle.x,
            obstacle.y,
            obstacle.width,
            obstacle.height,
        );

        dino_rect.overlaps(&obstacle_rect)
    }

    pub fn reset(&mut self) {
        self.dinosaur = Dinosaur::new(50.0, self.ground_y);
        self.obstacles.clear();
        self.score = 0;
        self.game_over = false;
    }
} 