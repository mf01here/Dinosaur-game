use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, DrawParam, Rect, Color, Mesh, DrawMode};
use ggez::mint::Point2;

pub struct Dinosaur {
    pub x: f32,
    pub y: f32,
    pub velocity_y: f32,
    pub width: f32,
    pub height: f32,
    pub is_jumping: bool,
    ground_y: f32,
}

impl Dinosaur {
    const JUMP_VELOCITY: f32 = -15.0;
    const GRAVITY: f32 = 0.8;
    const WIDTH: f32 = 40.0;
    const HEIGHT: f32 = 60.0;

    pub fn new(x: f32, ground_y: f32) -> Self {
        Dinosaur {
            x,
            y: ground_y - Self::HEIGHT,
            velocity_y: 0.0,
            width: Self::WIDTH,
            height: Self::HEIGHT,
            is_jumping: false,
            ground_y,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.is_jumping {
            // Apply gravity
            self.velocity_y += Self::GRAVITY;
            self.y += self.velocity_y;
            println!("Dinosaur position: y={}, velocity={}", self.y, self.velocity_y);

            // Check if landed
            if self.y >= self.ground_y - self.height {
                self.y = self.ground_y - self.height;
                self.velocity_y = 0.0;
                self.is_jumping = false;
                println!("Dinosaur landed at y={}", self.y);
            }
        }
        Ok(())
    }

    pub fn draw(&self, ctx: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> GameResult {
        use ggez::graphics::{DrawParam, Rect, Color, Mesh, DrawMode};
        // Draw dinosaur as a simple rectangle for now
        let dino_rect = Rect::new(self.x, self.y, self.width, self.height);
        let dino_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            dino_rect,
            Color::BLACK,
        )?;
        canvas.draw(&dino_mesh, DrawParam::default());
        Ok(())
    }

    pub fn jump(&mut self) {
        println!("Jump called - is_jumping: {}", self.is_jumping);
        if !self.is_jumping {
            self.velocity_y = Self::JUMP_VELOCITY;
            self.is_jumping = true;
            println!("Jump initiated with velocity: {}", self.velocity_y);
        }
    }
} 