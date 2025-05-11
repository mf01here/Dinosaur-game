use ggez::{Context, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawParam, Mesh, Text, DrawMode};
use ggez::input::keyboard::{KeyCode, KeyInput};

mod game;
mod dinosaur;
mod obstacle;

use game::Game;

fn main() -> GameResult {
    // Create a new game context
    let cb = ggez::ContextBuilder::new("dinosaur-game", "your-name")
        .window_setup(WindowSetup::default().title("Dinosaur Game"))
        .window_mode(WindowMode::default().dimensions(800.0, 300.0));

    let (mut ctx, event_loop) = cb.build()?;
    
    // Create and run the game
    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}

// Game state implementation
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update game state
        self.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a canvas for drawing
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw game elements
        self.draw_game(ctx, &mut canvas)?;
        // Present the frame
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        // Handle key presses
        println!("Key pressed: {:?}", input.keycode);
        if input.keycode == Some(KeyCode::Space) {
            println!("Space pressed - attempting to jump");
            self.jump();
        }
        if input.keycode == Some(KeyCode::R) {
            println!("R pressed - resetting game");
            self.reset();
        }
        Ok(())
    }
} 