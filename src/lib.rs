mod utils;

use chess::{Board, BoardBuilder, ChessMove, Color, Game, MoveGen, Piece, Square, ALL_SQUARES};
use std::cmp::{max, min};
//use rand::seq::IteratorRandom;
use wasm_bindgen::__rt::core::f32::MIN;
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

static mut eval_count: isize = 0;

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
            eval_count = 0;
        }
        let mut move_gen = MoveGen::new_legal(&self.game.current_position());
        //        let chess_move = move_gen.choose(&mut rand::thread_rng());
        move_gen
            .map(|chess_move| {
                let new_board = &self.game.current_position().make_move_new(chess_move);
                (chess_move, {
                    let is_maximising_player = false;
                    minimax(new_board, 5, -10000, 10000, is_maximising_player)
                })
            })
            .max_by(|(_, estimation1), (_, estimation2)| estimation1.cmp(estimation2))
            .map(|(chess_move, _)| chess_move)
            .into_iter()
            .for_each(|chess_move| {
                unsafe {
                    console_log!("best move {} among {}", chess_move, eval_count);
                }
                &self.game.make_move(chess_move);
            });
        let position_as_fen = format!("{}", self.game.current_position());
        position_as_fen
    }
}

fn minimax(board: &Board, depth: u32, alpha: i32, beta: i32, is_maximising_player: bool) -> i32 {
    if depth == 0 {
        return -evaluate_board(board);
    }
    let mut move_gen = MoveGen::new_legal(board);
    let evaluation = if is_maximising_player {
        let mut best_evaluation = -9999;
        let mut alpha = alpha;
        for chess_move in move_gen {
            let new_board = board.make_move_new(chess_move);
            best_evaluation = max(
                best_evaluation,
                minimax(&new_board, depth - 1, alpha, beta, !is_maximising_player),
            );
            alpha = max(alpha, best_evaluation);
            if beta <= alpha {
                break;
            }
        }
        best_evaluation
    } else {
        let mut best_evaluation = 9999;
        let mut beta = beta;
        for chess_move in move_gen {
            let new_board = board.make_move_new(chess_move);
            best_evaluation = min(
                best_evaluation,
                minimax(&new_board, depth - 1, alpha, beta, !is_maximising_player),
            );
            beta = min(beta, best_evaluation);
            if beta <= alpha {
                break;
            }
        }
        best_evaluation
    };
    //    console_log!(
    //        "{} - evaluation {}",
    //        "  ".repeat((3 - depth) as usize),
    //        evaluation
    //    );
    evaluation
}

fn evaluate_board(board: &Board) -> i32 {
    unsafe {
        eval_count = eval_count + 1;
    }
    ALL_SQUARES.iter().fold(0, |acc, &square| {
        acc + board
            .color_on(square)
            .and_then(|color| {
                board
                    .piece_on(square)
                    .map(|piece| piece_value(&piece, &color))
            })
            .unwrap_or(0)
    })
}

fn piece_value(piece: &Piece, color: &Color) -> i32 {
    let value = match piece {
        Piece::Pawn => 10,
        Piece::Knight => 30,
        Piece::Bishop => 30,
        Piece::Rook => 50,
        Piece::Queen => 90,
        Piece::King => 900,
    };
    match color {
        Color::White => value,
        Color::Black => -value,
    }
}
