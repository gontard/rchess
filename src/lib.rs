mod piece_evaluation;
mod utils;

use crate::piece_evaluation::evaluate_piece;
use chess::{Board, ChessMove, Game, MoveGen, Square, ALL_SQUARES};
use std::cmp::Ordering::Equal;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct RChess {
    game: Game,
}

static mut EVAL_COUNT: isize = 0;

#[wasm_bindgen]
impl RChess {
    pub fn new() -> Self {
        RChess { game: Game::new() }
    }

    pub fn move_piece(&mut self, new_move: &str) -> String {
        console_log!("-> {}", new_move);
        let v: Vec<_> = new_move.split("-").collect();
        let new_chess_move = ChessMove::new(
            Square::from_string(v[0].to_string()).unwrap(),
            Square::from_string(v[1].to_string()).unwrap(),
            None,
        );
        let is_legal_move = &self.game.make_move(new_chess_move);
        console_log!("-> {} {}", new_chess_move, is_legal_move);

        let position_as_fen = format!("{}", self.game.current_position());
        position_as_fen
    }

    pub fn compute_move(&mut self) -> String {
        unsafe {
            EVAL_COUNT = 0;
        }
        let depth = 4;
        let move_gen = MoveGen::new_legal(&self.game.current_position());
        move_gen
            .map(|chess_move| {
                let new_board = &self.game.current_position().make_move_new(chess_move);
                (chess_move, {
                    let is_maximising_player = false;
                    minimax(new_board, depth - 1, -10000.0, 10000.0, is_maximising_player)
                })
            })
            .max_by(|(_, estimation1), (_, estimation2)| {
                estimation1.partial_cmp(estimation2).unwrap_or(Equal)
            })
            .map(|(chess_move, _)| chess_move)
            .into_iter()
            .for_each(|chess_move| {
                unsafe {
                    console_log!("best move {} among {}", chess_move, EVAL_COUNT);
                }
                &self.game.make_move(chess_move);
            });
        let position_as_fen = format!("{}", self.game.current_position());
        position_as_fen
    }
}

fn minimax(board: &Board, depth: u32, alpha: f32, beta: f32, is_maximising_player: bool) -> f32 {
    unsafe {
        EVAL_COUNT = EVAL_COUNT + 1;
    }
    if depth == 0 {
        return -evaluate_board(board);
    }
    let move_gen = MoveGen::new_legal(board);
    let mut best_evaluation: f32 = if is_maximising_player {
        -9999.0
    } else {
        9999.0
    };
    let mut alpha = alpha;
    let mut beta = beta;
    for chess_move in move_gen {
        let new_board = board.make_move_new(chess_move);
        let evaluation = minimax(&new_board, depth - 1, alpha, beta, !is_maximising_player);
        if is_maximising_player {
            best_evaluation = best_evaluation.max(evaluation);
            alpha = alpha.max(best_evaluation);
        } else {
            best_evaluation = best_evaluation.min(evaluation);
            beta = beta.min(best_evaluation);
        }
        if beta <= alpha {
            break;
        }
    }
    best_evaluation
}

fn evaluate_board(board: &Board) -> f32 {
    ALL_SQUARES
        .iter()
        .filter_map(|&square| {
            board.color_on(square).and_then(|color| {
                board
                    .piece_on(square)
                    .map(|piece| evaluate_piece(piece, color, square))
            })
        })
        .sum()
}
