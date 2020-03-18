//! Game models for the game 2048
//! 
//! game_2048_model provide multiple game engine models for [the game 2048](https://en.wikipedia.org/wiki/2048_(video_game))
//! 
//! # Quick Start
//! 
//! To get started, here is an example of how to setup a new game and run the first turn.
//! 
//! ```
//! use rand::thread_rng;
//! use game_2048_model::prelude::*;
//!
//! // Inital game setup with two numbers spawned
//! let mut game = Matrix::new();
//! let mut rng = thread_rng();
//! game.random(&mut rng);
//! game.random(&mut rng);
//! 
//! // The player choose a direction and the game spawns a new number
//! game.slide(Directions::Up);
//! game.random(&mut rng);
//! ```
//! 
//! You can also load an existing game.
//! 
//! ```
//! use rand::prelude::*;
//! use game_2048_model::prelude::*;
//!
//! // Load an existing game
//! let stored_game = [
//!     [0, 0, 0, 0],
//!     [4, 0, 0, 2],
//!     [2, 8, 4, 0],
//!     [32, 8, 16, 4]
//! ];
//! let mut game = Matrix::from(stored_game);
//! 
//! // The player choose a direction and the game spawns a new number
//! game.slide(Directions::Down);
//! ```

pub mod models;
mod base;

pub use base::*;

pub mod prelude;