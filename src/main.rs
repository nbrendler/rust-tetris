use std::env;
use std::path;

#[macro_use]
extern crate gfx;
extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod assets;
mod constants;
mod piece;
mod position;
mod state;
mod types;

use crate::state::{SceneManager};

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        println!("backup");
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("tetris", "me")
        .window_setup(conf::WindowSetup::default().title("Tetris"))
        .window_mode(conf::WindowMode::default().dimensions(640., 480.).resizable(true))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut SceneManager::new(ctx)?;
    event::run(ctx, events_loop, game)
}
