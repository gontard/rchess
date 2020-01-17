mod board_evaluation;
mod transposition_table;
mod utils;

use crate::board_evaluation::evaluate_board;
use crate::transposition_table::TranspositionTable;
use crate::utils::{read_chess_move, set_panic_hook};
use chess::{Board, ChessMove, Game, MoveGen};
use std::cmp::Ordering::Equal;
use std::fmt;
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
    transposition_table: TranspositionTable,
    stats: Stats,
}

#[wasm_bindgen]
impl RChess {
    pub fn new() -> Self {
        set_panic_hook();
        RChess {
            game: Game::new(),
            transposition_table: TranspositionTable::new(),
            stats: Stats::new(),
        }
    }

    pub fn move_piece(&mut self, new_move: &str) -> String {
        console_log!("-> received={}", new_move);
        let new_chess_move: ChessMove = read_chess_move(new_move);
        let is_legal_move = &self.game.make_move(new_chess_move);
        console_log!("-> move={} legal={}", new_chess_move, is_legal_move);
        format!("{}", self)
    }

    pub fn compute_move(&mut self) -> String {
        self.stats = Stats::new();

        let depth = 4;
        let move_gen = MoveGen::new_legal(&self.game.current_position());
        move_gen
            .map(|chess_move| {
                let new_board = &self.game.current_position().make_move_new(chess_move);
                (chess_move, {
                    let is_maximising_player = false;
                    self.minimax(
                        new_board,
                        depth - 1,
                        -10000.0,
                        10000.0,
                        is_maximising_player,
                    )
                })
            })
            .max_by(|(_, estimation1), (_, estimation2)| {
                estimation1.partial_cmp(estimation2).unwrap_or(Equal)
            })
            .map(|(chess_move, _)| chess_move)
            .into_iter()
            .for_each(|chess_move| {
                console_log!("{:#?}", self.stats);
                &self.game.make_move(chess_move);
            });
        format!("{}", self)
    }

    fn minimax(
        &mut self,
        board: &Board,
        depth: u32,
        alpha: f32,
        beta: f32,
        is_maximising_player: bool,
    ) -> f32 {
        self.stats.eval_count = self.stats.eval_count + 1;
        match self.transposition_table.get_evaluation(board, depth) {
            None => {
                self.stats.tt_miss_count = self.stats.tt_miss_count + 1;
            }
            Some(evaluation) => {
                self.stats.tt_hit_count = self.stats.tt_hit_count + 1;
                return evaluation;
            }
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
            let evaluation =
                self.minimax(&new_board, depth - 1, alpha, beta, !is_maximising_player);
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
        self.transposition_table
            .put_evaluation(board, depth, best_evaluation);
        best_evaluation
    }
}

impl fmt::Display for RChess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, "\"position\":\"{}\"", self.game.current_position())?;
        match self.game.result() {
            None => {}
            Some(result) => {
                write!(f, ",\"result\":\"{:?}\"", result)?;
            }
        }
        write!(f, "}}")
    }
}

#[derive(Debug)]
struct Stats {
    eval_count: isize,
    tt_hit_count: isize,
    tt_miss_count: isize,
}

impl Stats {
    fn new() -> Self {
        Stats {
            eval_count: 0,
            tt_hit_count: 0,
            tt_miss_count: 0,
        }
    }
}
