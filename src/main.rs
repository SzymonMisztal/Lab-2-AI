use crate::board::{Board};
use crate::game::Game;
use crate::minmax::Minmax;

mod board;
mod game;
mod user_input;
mod minmax;

fn main() {
    println!("Hello, world!");

    Game::run(true, false)
}