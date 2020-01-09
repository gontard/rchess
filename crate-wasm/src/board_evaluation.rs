use chess::{Board, Color, Piece, Square, ALL_SQUARES};

pub fn evaluate_board(board: &Board) -> f32 {
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

pub fn evaluate_piece(piece: Piece, color: Color, square: Square) -> f32 {
    let evaluation = match piece {
        Piece::Pawn => 10.0 + value_at_position(PAWN_VALUES, color, square),
        Piece::Knight => 30.0 + value_at_position(KNIGHT_VALUES, color, square),
        Piece::Bishop => 30.0 + value_at_position(BISHOP_VALUES, color, square),
        Piece::Rook => 50.0 + value_at_position(ROOK_VALUES, color, square),
        Piece::Queen => 90.0 + value_at_position(QUEEN_VALUES, color, square),
        Piece::King => 900.0 + value_at_position(KING_VALUES, color, square),
    };
    match color {
        Color::White => evaluation,
        Color::Black => -evaluation,
    }
}

fn value_at_position(values: [[f32; 8]; 8], color: Color, square: Square) -> f32 {
    match color {
        Color::White => values[7 - square.get_rank().to_index()][square.get_file().to_index()],
        Color::Black => values[square.get_rank().to_index()][square.get_file().to_index()],
    }
}

#[rustfmt::skip]
const PAWN_VALUES: [[f32; 8]; 8] = [
    [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
    [5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0],
    [1.0,  1.0,  2.0,  3.0,  3.0,  2.0,  1.0,  1.0],
    [0.5,  0.5,  1.0,  2.5,  2.5,  1.0,  0.5,  0.5],
    [0.0,  0.0,  0.0,  2.0,  2.0,  0.0,  0.0,  0.0],
    [0.5, -0.5, -1.0,  0.0,  0.0, -1.0, -0.5,  0.5],
    [0.5,  1.0,  1.0, -2.0, -2.0,  1.0,  1.0,  0.5],
    [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0]
];

#[rustfmt::skip]
const KNIGHT_VALUES: [[f32; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0,  0.0,  0.0,  0.0,  0.0, -2.0, -4.0],
    [-3.0,  0.0,  1.0,  1.5,  1.5,  1.0,  0.0, -3.0],
    [-3.0,  0.5,  1.5,  2.0,  2.0,  1.5,  0.5, -3.0],
    [-3.0,  0.0,  1.5,  2.0,  2.0,  1.5,  0.0, -3.0],
    [-3.0,  0.5,  1.0,  1.5,  1.5,  1.0,  0.5, -3.0],
    [-4.0, -2.0,  0.0,  0.5,  0.5,  0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0]
];

#[rustfmt::skip]
const BISHOP_VALUES: [[f32; 8]; 8] = [
    [ -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [ -1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0],
    [ -1.0,  0.0,  0.5,  1.0,  1.0,  0.5,  0.0, -1.0],
    [ -1.0,  0.5,  0.5,  1.0,  1.0,  0.5,  0.5, -1.0],
    [ -1.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0, -1.0],
    [ -1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0, -1.0],
    [ -1.0,  0.5,  0.0,  0.0,  0.0,  0.0,  0.5, -1.0],
    [ -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0]
];

#[rustfmt::skip]
const ROOK_VALUES: [[f32; 8]; 8] = [
    [  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
    [  0.5,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [  0.0,   0.0, 0.0,  0.5,  0.5,  0.0,  0.0,  0.0]
];

#[rustfmt::skip]
const QUEEN_VALUES: [[f32; 8]; 8] = [
    [ -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
    [ -1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0],
    [ -1.0,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -1.0],
    [ -0.5,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -0.5],
    [  0.0,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -0.5],
    [ -1.0,  0.5,  0.5,  0.5,  0.5,  0.5,  0.0, -1.0],
    [ -1.0,  0.0,  0.5,  0.0,  0.0,  0.0,  0.0, -1.0],
    [ -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0]
];

#[rustfmt::skip]
const KING_VALUES: [[f32; 8]; 8] = [
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [ -1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [  2.0,  2.0,  0.0,  0.0,  0.0,  0.0,  2.0,  2.0 ],
    [  2.0,  3.0,  1.0,  0.0,  0.0,  1.0,  3.0,  2.0 ]
];

#[cfg(test)]
mod tests {
    use chess::{
        Color::{Black, White},
        Piece::{King, Pawn},
        Square,
    };

    use super::evaluate_piece;

    #[test]
    fn evaluate_pawn() {
        assert_eq!(10.5, evaluate_piece(Pawn, White, Square::A2));
        assert_eq!(-15.0, evaluate_piece(Pawn, Black, Square::A2));
        assert_eq!(8.0, evaluate_piece(Pawn, White, Square::E2));
        assert_eq!(-15.0, evaluate_piece(Pawn, Black, Square::E2));
    }

    #[test]
    fn evaluate_king() {
        assert_eq!(902.0, evaluate_piece(King, White, Square::A1));
        assert_eq!(-897.0, evaluate_piece(King, Black, Square::A1));
        assert_eq!(896.0, evaluate_piece(King, White, Square::E4));
        assert_eq!(-895.0, evaluate_piece(King, Black, Square::E4));
    }
}
