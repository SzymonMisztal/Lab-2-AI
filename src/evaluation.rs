use std::borrow::Borrow;
use crate::board::Board;
use crate::config;
use crate::config::{BORDER_POINTS, PLAYER1_EVAL, PLAYER2_EVAL};
use crate::game::Game;

pub enum Evaluation {
    Table,
    CheckersDiffPos,
    CheckersDiffPosMoves
}

impl Evaluation {
    pub fn score(
        board: &[[char; 8]; 8],
        player: char,
        eval: &Evaluation
    ) -> isize {
        return match eval {
            Evaluation::Table => Self::score_table(board, player),
            Evaluation::CheckersDiffPos => Self::score_checkers_diff_pos(board, player, 0),
            Evaluation::CheckersDiffPosMoves => {
                let mut vec: Vec<(usize, usize)> = Vec::new();
                Game::possible_moves(board, player, &mut vec);
                Self::score_checkers_diff_pos(
                    board,
                    player,
                    vec.len() as isize
                )
            }
        }
    }

    fn score_table(
        board: &[[char; 8]; 8],
        player: char
    ) -> isize {

        let enemy = Game::swap_player(player);
        let mut score: isize = 0;

        let weights = [
            [120, -20,  20,   5,   5,  20, -20, 120],
            [-20, -40,  -5,  -5,  -5,  -5, -40, -20],
            [ 20,  -5,  15,   3,   3,  15,  -5,  20],
            [  5,  -5,   3,   3,   3,   3,  -5,   5],
            [  5,  -5,   3,   3,   3,   3,  -5,   5],
            [ 20,  -5,  15,   3,   3,  15,  -5,  20],
            [-20, -40,  -5,  -5,  -5,  -5, -40, -20],
            [120, -20,  20,   5,   5,  20, -20, 120]
        ];

        for (y, row) in board.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                if *element == player { score += weights[y][x] }
                else if *element == enemy { score -= weights[y][x] }
            }
        }

        //Board::print_board2(board);
        //println!("{}", score);

        score
    }
    fn score_checkers_diff_pos(
        board: &[[char; 8]; 8],
        player: char,
        mut score: isize
    ) -> isize {

        let enemy = Game::swap_player(player);

        for row in board.iter() {
            for element in row.iter() {
                if *element == player { score += config::CHECKER_POINTS }
                else if *element == enemy { score -= config::CHECKER_POINTS }
            }
        }


        for x in 0..8 {
            if board[0][x] == player { score += BORDER_POINTS }
        }
        for x in 0..8 {
            if board[7][x] == player { score += BORDER_POINTS }
        }
        for y in 0..8 {
            if board[y][0] == player { score += BORDER_POINTS }
        }
        for y in 0..8 {
            if board[y][7] == player { score += BORDER_POINTS }
        }

        return score
    }
}