use std::borrow::Borrow;
use crate::board::{Board};
use crate::config::{PLAYER1_EVAL, PLAYER1_HUMAN, PLAYER2_EVAL, PLAYER2_HUMAN};
use crate::evaluation::Evaluation;
use crate::minmax::Minmax;
use crate::user_input;


pub struct Game {}

impl Game {
    pub fn run() {

        let player1_human: bool = PLAYER1_HUMAN;
        let player2_human: bool = PLAYER2_HUMAN;

        let mut current_player: char = 'W';
        let mut current_board = Board::create_board();
        let mut changes: Vec<(usize, usize)> = Vec::new();

        Board::print_board(&current_board, &changes, (usize::MAX, usize::MAX));

        loop {

            let human = match current_player {
                'W' => player1_human,
                'B' => player2_human,
                _ => true
            };

            let mut possible_moves: Vec<(usize, usize)> = Vec::new();
            Self::possible_moves(&current_board, current_player, &mut possible_moves);

            if possible_moves.is_empty() {
                Self::print_result(&current_board);
                break
            }

            if human {
                loop {
                    let x: isize = user_input::get_input("Get x: ").parse::<isize>().unwrap() - 1;
                    let y: isize = user_input::get_input("Get y: ").parse::<isize>().unwrap() - 1;

                    if y>0 && x>0 && possible_moves.iter().any(|&(a, b)| a == y as usize && b == x as usize) {
                        changes = Self::make_move(&mut current_board, y as usize, x as usize, current_player);
                        Board::print_board(&current_board, &changes, (y as usize, x as usize));
                        break
                    }

                    println!("Please select valid position!")
                }
            }
            else {
                let best_move = Minmax::best_move(&current_board, possible_moves, current_player,
                match current_player {
                    'W' => PLAYER1_EVAL.borrow(),
                    _ => PLAYER2_EVAL.borrow()
                });
                changes = Self::make_move(&mut current_board, best_move.0, best_move.1, current_player);
                Board::print_board(&current_board, &changes, best_move);
            }

            current_player = Self::swap_player(current_player);
        }
    }

    pub fn possible_moves(board: &[[char; 8]; 8], player: char, possible_moves: &mut Vec<(usize, usize)>) {
        for (y, y_iter) in board.iter().enumerate() {
            for (x, _) in y_iter.iter().enumerate() {
                if Self::is_move_possible(board, y, x, player) {
                    possible_moves.push((y, x))
                }
            }
        }
    }

    fn is_move_possible(board: &[[char; 8]; 8], y: usize, x: usize, player: char) -> bool {
        if board[y][x] != ' ' { return false }
        if Self::star_check(board, y, x, player) { return true }
        return false
    }

    fn star_check(board: &[[char; 8]; 8], y: usize, x: usize, player: char) -> bool {
        if
            Self::line_check(board, y, x, -1, -1, player) ||
            Self::line_check(board, y, x, -1, 0, player) ||
            Self::line_check(board, y, x, -1, 1, player) ||
            Self::line_check(board, y, x, 0, -1, player) ||
            Self::line_check(board, y, x, 0, 1, player) ||
            Self::line_check(board, y, x, 1, -1, player) ||
            Self::line_check(board, y, x, 1, 0, player) ||
            Self::line_check(board, y, x, 1, 1, player)
        { return true }
        return false;
    }

    fn line_check(board: &[[char; 8]; 8], y: usize, x: usize, y_delta: isize, x_delta: isize, player: char) -> bool {

        let enemy = Self::swap_player(player);

        let mut y_current: isize = y as isize + y_delta;
        let mut x_current: isize = x as isize + x_delta;
        let mut found_enemy = false;
        while Self::is_point_in_board(y_current, x_current) {
            if !found_enemy && board[y_current as usize][x_current as usize] != enemy { return false } else { found_enemy = true };

            if board[y_current as usize][x_current as usize] == ' ' { return false }
            else if board[y_current as usize][x_current as usize] == player { return true }

            y_current += y_delta;
            x_current += x_delta;
        }

        return false
    }

    fn is_near_enemy(board: &[[char; 8]; 8], y: usize, x: usize) -> bool {
        let enemy = Self::swap_player(board[y][x]);

        if y > 0 {
            if x > 0 { if enemy == board[y - 1][x - 1] { return true } }
            if x < 7 { if enemy == board[y - 1][x + 1] { return true } }
            if enemy == board[y - 1][x] { return true }
        }
        if y < 7 {
            if x > 0 { if enemy == board[y + 1][x - 1] { return true } }
            if x < 7 { if enemy == board[y + 1][x + 1] { return true } }
            if enemy == board[y + 1][x] { return true }
        }
        if x > 0 { if enemy == board[y][x - 1] { return true } }
        if x < 7 { if enemy == board[y][x + 1] { return true } }

        false
    }

    fn is_point_in_board(y: isize, x: isize) -> bool {
        if y >= 0 && y < 8 && x >= 0 && x < 8 { true } else { false }
    }

    pub fn swap_player(player: char) -> char {
        match player {
            'W' => 'B',
            'B' => 'W',
            _ => ' '
        }
    }

    pub fn score(board: &[[char; 8]; 8]) -> (usize, usize) {

        let mut white: usize = 0;
        let mut black: usize = 0;

        for row in board.iter() {
            for &element in row.iter() {
                match element {
                    'W' => white += 1,
                    'B' => black += 1,
                    _ => {}
                }
            }
        }

        return (white, black)
    }

    fn print_result(board: &[[char; 8]; 8]) {
        let results = Self::score(board);
        if results.0 >= results.1 {
            println!("White wins!")
        } else if results.0 == results.1 {
            println!("?Match ends with draw!")
        } else {
            println!("Black wins!")
        }
        println!("Score: {}:{}", results.0, results.1)
    }

    pub fn make_move(board: &mut [[char; 8]; 8], y: usize, x: usize, player: char) -> Vec<(usize, usize)> {

        let mut changes: Vec<(usize, usize)> = Vec::new();
        if !Self::is_move_possible(board, y, x, player) { return changes }

        board[y][x] = player;
        changes.push((y, x));
        Self::star_swap(board, y, x, player, &mut changes);

        return changes.clone()
    }

    fn star_swap(board: &mut [[char; 8]; 8], y: usize, x: usize, player: char, changes: &mut Vec<(usize, usize)>) {
        Self::line_swap(board, y, x, -1, -1, player, changes);
        Self::line_swap(board, y, x, -1, 0, player, changes);
        Self::line_swap(board, y, x, -1, 1, player, changes);
        Self::line_swap(board, y, x, 0, -1, player, changes);
        Self::line_swap(board, y, x, 0, 1, player, changes);
        Self::line_swap(board, y, x, 1, -1, player, changes);
        Self::line_swap(board, y, x, 1, 0, player, changes);
        Self::line_swap(board, y, x, 1, 1, player, changes);
    }

    fn line_swap(board: &mut [[char; 8]; 8], y: usize, x: usize, y_delta: isize, x_delta: isize, player: char, changes: &mut Vec<(usize, usize)>) {

        if !Self::line_check(board, y, x, y_delta, x_delta, player) { return }

        let enemy = Self::swap_player(player);

        let mut y_current = (y as isize + y_delta) as usize;
        let mut x_current = (x as isize + x_delta) as usize;

        while board[y_current][x_current] == enemy {

            board[y_current][x_current] = player;
            changes.push((y_current, x_current));

            y_current = (y_current as isize + y_delta) as usize;
            x_current = (x_current as isize + x_delta) as usize;
        }
    }
}