use std::cmp;
use crate::game::Game;

pub struct Minmax {}

impl Minmax {
    pub fn best_move(
        board: &[[char; 8]; 8],
        possible_moves: Vec<(usize, usize)>,
        player: char,
        depth: usize
    ) -> (usize, usize) {

        let mut evals: Vec<isize> = Vec::with_capacity(possible_moves.len());

        for &p_move in possible_moves.iter() {
            let mut board_clone = board.clone();
            board_clone[p_move.0][p_move.1] = player;
            evals.push(Self::minmax(
                board_clone,
                Game::swap_player(player),
                false,
                4
            ))
        }

        return match evals.iter().enumerate().max() {
            Some((i, eval)) => possible_moves[i],
            None => (usize::MAX, usize::MAX)
        }
    }

    fn minmax(
        mut board: [[char; 8]; 8],
        player: char,
        maximizing: bool,
        depth: usize
        //mut alpha: isize,
        //beta: isize
    ) -> isize {

        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        Game::possible_moves(&board, player, &mut possible_moves);

        if depth == 0 || possible_moves.len() == 0 {
            return Self::score(&board, possible_moves, player);
        }

        if maximizing {
            let mut max_eval = isize::MIN;

            for &p_move in possible_moves.iter() {

                let mut board_clone = board.clone();
                board_clone[p_move.0][p_move.1] = player;

                let eval = Self::minmax(
                    board_clone,
                    Game::swap_player(player),
                    !maximizing,
                    depth - 1
                    //alpha,
                    //beta
                );

                max_eval = cmp::max(max_eval, eval);
                //alpha = cmp::max(alpha, eval);

                //if beta <= alpha { break }
            }

            return max_eval
        }
        else {
            let mut min_eval = isize::MAX;

            for &p_move in possible_moves.iter() {

                let mut board_clone = board.clone();
                board_clone[p_move.0][p_move.1] = player;

                let eval = Self::minmax(
                    board_clone,
                    Game::swap_player(player),
                    !maximizing,
                    depth - 1
                    //alpha,
                    //beta
                );

                min_eval = cmp::min(min_eval, eval);
                //alpha = cmp::max(alpha, eval);

                //if beta <= alpha { break }
            }

            return min_eval
        }
    }

    fn score(
        board: &[[char; 8]; 8],
        possible_moves: Vec<(usize, usize)>,
        player: char
    ) -> isize {

        let mut score = possible_moves.len();

        for row in board.iter() {
            for element in row.iter() {
                if *element == player { score += 1; }
            }
        }

        for x in 0..8 {
            if board[0][x] == player { score += 1 }
        }
        for x in 0..8 {
            if board[7][x] == player { score += 1 }
        }
        for y in 0..8 {
            if board[y][0] == player { score += 1 }
        }
        for y in 0..8 {
            if board[y][7] == player { score += 1 }
        }

        return score as isize
    }
}