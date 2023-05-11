use std::borrow::Borrow;
use std::cmp;
use crate::game::Game;
use crate::{config, evaluation};
use crate::evaluation::Evaluation;

pub struct Minmax {}

impl Minmax {
    pub fn best_move(
        board: &[[char; 8]; 8],
        possible_moves: Vec<(usize, usize)>,
        player: char,
        tactic: &Evaluation
    ) -> (usize, usize) {

        let mut evals: Vec<isize> = Vec::with_capacity(possible_moves.len());

        for &p_move in possible_moves.iter() {
            let mut board_clone = board.clone();
            Game::make_move(&mut board_clone, p_move.0, p_move.1, player);
            evals.push(Self::minmax(
                board_clone,
                Game::swap_player(player),
                //evaluation,
                false,
                match player {
                    'W' => config::PLAYER1_DEPTH,
                    _ => config::PLAYER2_DEPTH,
                },
                isize::MIN,
                isize::MAX,
                tactic
            ))
        }

        let mut max = isize::MIN;
        let mut i: usize = 0;

        for (j, eval) in evals.iter().enumerate() {
            if max < *eval {
                max = *eval;
                i = j
            }
        }
        return possible_moves[i];
    }

    fn minmax(
        mut board: [[char; 8]; 8],
        player: char,
        maximizing: bool,
        depth: usize,
        mut alpha: isize,
        mut beta: isize,
        tactic: &Evaluation
    ) -> isize {

        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        Game::possible_moves(&board, player, &mut possible_moves);



        if maximizing {

            if depth == 0 || possible_moves.len() == 0 {
                return Evaluation::score(&board, Game::swap_player(player), tactic);
            }

            let mut max_eval = isize::MIN;

            for &p_move in possible_moves.iter() {

                let mut board_clone = board.clone();
                //board_clone[p_move.0][p_move.1] = player;
                Game::make_move(&mut board_clone, p_move.0, p_move.1, player);

                let eval = Self::minmax(
                    board_clone,
                    Game::swap_player(player),
                    //evaluation,
                    !maximizing,
                    depth - 1,
                    alpha,
                    beta,
                    tactic.clone()
                );

                max_eval = cmp::max(max_eval, eval);
                alpha = cmp::max(alpha, eval);
                //if beta <= alpha { break }
            }

            return max_eval
        }
        else {

            if depth == 0 || possible_moves.len() == 0 {
                return -Evaluation::score(&board, player, tactic);
            }

            let mut min_eval = isize::MAX;

            for &p_move in possible_moves.iter() {

                let mut board_clone = board.clone();
                Game::make_move(&mut board_clone, p_move.0, p_move.1, player);

                let eval = Self::minmax(
                    board_clone,
                    Game::swap_player(player),
                    //evaluation,
                    !maximizing,
                    depth - 1,
                    alpha,
                    beta,
                    tactic
                );

                min_eval = cmp::min(min_eval, eval);
                beta = cmp::min(beta, eval);
                //if beta <= alpha { break }
            }

            return min_eval
        }
    }
}