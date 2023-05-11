use std::time::Instant;
use macroquad::miniquad::CursorIcon::Default;
use crate::board::{Board};
use crate::game::Game;
use crate::minmax::Minmax;
use crate::config::WINDOW_SIZE;
use crate::evaluation::Evaluation;

mod board;
mod game;
mod user_input;
mod minmax;
mod config;
mod evaluation;

fn main() {

    let start_time = Instant::now();

    Game::run(true, false);

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("Execution time: {:?}", duration);


}