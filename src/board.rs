use std::fmt;
use crate::config::{FLIP_INDICATOR, MOVE_INDICATOR};


pub struct Board {}

impl Board {
    pub fn reset_board(board: &mut [[char; 8]; 8]) {
        for y in board.iter_mut() {
            for x in y.iter_mut() {
                *x = ' ';
            }
        }
        board[3][3] = 'W';
        board[4][4] = 'W';
        board[3][4] = 'B';
        board[4][3] = 'B';
    }

    pub fn create_board() -> [[char; 8]; 8] {
        let mut board = [[' '; 8]; 8];
        board[3][3] = 'W';
        board[4][4] = 'W';
        board[3][4] = 'B';
        board[4][3] = 'B';
        return board;
    }

    pub fn print_board(board: &[[char; 8]; 8], changes: &Vec<(usize, usize)>, last_move: (usize, usize)) {
        println!("╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗");
        for (y, row) in board.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                print!("║{change}{character}{change}", character = ch, change =
                    if last_move.0 == y && last_move.1 == x { MOVE_INDICATOR }
                    else if changes.iter().any(|&(a, b)| a == y && b == x) { FLIP_INDICATOR }
                    else {' '});
            }
            println!("║");
            if y != board.len() - 1 {
                println!("╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣");
            }
        }
        println!("╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝");
    }
    pub fn print_board2(board: &[[char; 8]; 8]) {
        println!("╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗");
        for (y, row) in board.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                print!("║ {character} ", character = ch);
            }
            println!("║");
            if y != board.len() - 1 {
                println!("╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣");
            }
        }
        println!("╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝");
    }
}