use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, DrawParam, Rect, Color, Mesh, DrawMode};

pub struct Obstacle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    speed: f32,
}

impl Obstacle {
    const SPEED: f32 = 5.0;
    const WIDTH: f32 = 20.0;
    const HEIGHT: f32 = 40.0;

    pub fn new(x: f32, ground_y: f32) -> Self {
        Obstacle {
            x,
            y: ground_y - Self::HEIGHT,
            width: Self::WIDTH,
            height: Self::HEIGHT,
            speed: Self::SPEED,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Move obstacle to the left
        self.x -= self.speed;
        Ok(())
    }

    pub fn draw(&self, ctx: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> GameResult {
        use ggez::graphics::{DrawParam, Rect, Color, Mesh, DrawMode};
        // Draw obstacle as a simple rectangle for now
        let obstacle_rect = Rect::new(self.x, self.y, self.width, self.height);
        let obstacle_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            obstacle_rect,
            Color::GREEN,
        )?;
        canvas.draw(&obstacle_mesh, DrawParam::default());
        Ok(())
    }
} 