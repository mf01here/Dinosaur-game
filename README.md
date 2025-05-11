A simple clone of the Chrome offline dinosaur (T‑Rex) game, written in Rust using the ggez game engine.


**Overview**
This project recreates the classic “dinosaur” (T‑Rex) game (which appears when Chrome is offline) in Rust. The dinosaur (drawn as a black rectangle) jumps (using the space key) over obstacles (green rectangles) that move from right to left. The score increases as you survive, and the game ends (with a “Game Over” message) if the dinosaur collides with an obstacle. You can restart the game (after a game over) by pressing the “R” key.


**Features**
• A dinosaur (rendered as a black rectangle) that jumps (using the space key) over obstacles.
• Obstacles (green rectangles) that move from the right side of the screen toward the dinosaur.
• A score counter (incremented as you survive) and a “Game Over” message (displayed in red) when you collide with an obstacle.
• A restart mechanism (press “R” to reset the game) so you can play again.


**Requirements**
• Rust (and Cargo) installed on your machine.
• The ggez game engine (version 0.9) (see Cargo.toml).


**How to Run**
Clone the repository (or download the source) and then run:
  cargo run
in your terminal (from the project’s root directory).


**Controls**
• Space – Jump (press to make the dinosaur jump).
• R – Restart (press “R” after a game over to reset the game).


**Project Structure**
• src/main.rs – Entry point and event handling (keyboard input, game loop).
• src/game.rs – Defines the Game struct (update, draw, collision detection, and reset logic).
• src/dinosaur.rs – Contains the Dinosaur struct (jump, update, and draw methods).
• src/obstacle.rs – Defines the Obstacle struct (update and draw methods).
