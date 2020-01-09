use chess::{Board, Color, Piece, NUM_PIECES};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct BoardKey([u64; NUM_PIECES + 2]);

impl BoardKey {
    fn for_board(board: &Board) -> Self {
        let pawn_bit = board.pieces(Piece::Pawn).0;
        let knight_bit = board.pieces(Piece::Knight).0;
        let bishop_bit = board.pieces(Piece::Bishop).0;
        let rook_bit = board.pieces(Piece::Rook).0;
        let queen_bit = board.pieces(Piece::Queen).0;
        let king_bit = board.pieces(Piece::King).0;
        let white_bit = board.color_combined(Color::White).0;
        let black_bit = board.color_combined(Color::Black).0;
        BoardKey([
            pawn_bit, knight_bit, bishop_bit, rook_bit, queen_bit, king_bit, white_bit, black_bit,
        ])
    }
}

struct BoardEvaluation {
    value: f32,
    depth: u32,
}

impl BoardEvaluation {
    fn new(depth: u32, value: f32) -> Self {
        BoardEvaluation { depth, value }
    }
}

pub struct TranspositionTable {
    map: HashMap<BoardKey, BoardEvaluation>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            map: HashMap::with_capacity(10000),
        }
    }

    pub fn get_evaluation(&self, board: &Board, depth: u32) -> Option<f32> {
        if depth <= 0 {
            return None;
        }
        let board_key = BoardKey::for_board(board);
        self.map
            .get(&board_key)
            .filter(|evaluation| evaluation.depth >= depth)
            .map(|evaluation| evaluation.value)
    }

    pub fn put_evaluation(&mut self, board: &Board, depth: u32, evaluation: f32) {
        let board_key = BoardKey::for_board(board);
        let better_evaluation_already_present = self
            .map
            .get(&board_key)
            .filter(|evaluation| evaluation.depth >= depth)
            .is_some();
        if !better_evaluation_already_present {
            let board_evaluation = BoardEvaluation::new(depth, evaluation);
            self.map.insert(board_key, board_evaluation);
        }
    }
}

#[cfg(test)]
mod tests {
    use chess::{
        Board, BoardBuilder,
        Color::{Black, White},
        Piece::{King, Pawn},
        Square,
    };
    use std::convert::TryInto;

    use super::TranspositionTable;

    #[test]
    fn get_evaluation_returns_none_when_table_is_empty() {
        let board: Board = create_a_board();
        let table = TranspositionTable::new();

        assert_eq!(None, table.get_evaluation(&board, 2))
    }

    #[test]
    fn get_evaluation_returns_some_when_an_evaluation_at_the_same_depth_exists() {
        let board: Board = create_a_board();
        let mut table = TranspositionTable::new();

        table.put_evaluation(&board, 2, 33.3);

        assert_eq!(Some(33.3), table.get_evaluation(&board, 2));
    }

    #[test]
    fn get_evaluation_returns_some_when_an_evaluation_at_a_higher_depth_exists() {
        let board: Board = create_a_board();
        let mut table = TranspositionTable::new();

        table.put_evaluation(&board, 3, 33.3);

        assert_eq!(Some(33.3), table.get_evaluation(&board, 2));
    }

    #[test]
    fn get_evaluation_returns_none_when_an_evaluation_at_a_lower_depth_exists() {
        let board: Board = create_a_board();
        let mut table = TranspositionTable::new();

        table.put_evaluation(&board, 2, 33.3);

        assert_eq!(None, table.get_evaluation(&board, 3));
    }

    #[test]
    fn get_evaluation_returns_none_when_the_board_is_different() {
        let board: Board = create_a_board();
        let another_board: Board = create_another_board();
        let mut table = TranspositionTable::new();

        table.put_evaluation(&board, 8, 33.3);

        assert_eq!(None, table.get_evaluation(&another_board, 3));
    }

    #[test]
    fn put_evaluation_keeps_the_old_value_if_its_depth_is_higher() {
        let board: Board = create_a_board();
        let mut table = TranspositionTable::new();

        table.put_evaluation(&board, 8, 20.6);
        table.put_evaluation(&board, 5, 33.3);

        assert_eq!(Some(20.6), table.get_evaluation(&board, 3));
    }

    #[test]
    fn put_evaluation_sets_the_new_value_if_its_depth_is_higher() {
        let board: Board = create_a_board();
        let mut table = TranspositionTable::new();

        table.put_evaluation(&board, 5, 20.6);
        table.put_evaluation(&board, 8, 33.3);

        assert_eq!(Some(33.3), table.get_evaluation(&board, 3));
    }

    fn create_a_board() -> Board {
        BoardBuilder::new()
            .piece(Square::A1, King, White)
            .piece(Square::A8, King, Black)
            .try_into()
            .unwrap()
    }

    fn create_another_board() -> Board {
        BoardBuilder::new()
            .piece(Square::D1, King, White)
            .piece(Square::D8, King, Black)
            .piece(Square::E5, Pawn, Black)
            .try_into()
            .unwrap()
    }
}
