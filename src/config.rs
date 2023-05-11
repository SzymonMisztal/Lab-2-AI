use crate::evaluation::Evaluation;

pub static PLAYER1_HUMAN: bool = false;
pub static PLAYER2_HUMAN: bool = false;
pub static PLAYER1_DEPTH: usize = 4;
pub static PLAYER2_DEPTH: usize = 2;
pub static WINDOW_SIZE: i32 = 800;
pub static MOVE_INDICATOR: char = '|';
pub static FLIP_INDICATOR: char = '·';
pub static BORDER_POINTS: isize = 3;
pub static CHECKER_POINTS: isize = 2;
pub static PLAYER1_EVAL: Evaluation = Evaluation::Table;
pub static PLAYER2_EVAL: Evaluation = Evaluation::Table;