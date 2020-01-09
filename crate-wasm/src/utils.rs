use chess::{ChessMove, Square};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn read_chess_move(move_as_str: &str) -> ChessMove {
    let parts: Vec<_> = move_as_str.split("-").collect();
    ChessMove::new(
        Square::from_string(parts[0].to_string()).unwrap(),
        Square::from_string(parts[1].to_string()).unwrap(),
        None,
    )
}

#[cfg(test)]
mod tests {
    use chess::{Square, ChessMove};

    use super::read_chess_move;

    #[test]
    fn read_chess_move_success() {
        assert_eq!(ChessMove::new(Square::A1, Square::E7, None), read_chess_move("a1-e7"));
    }

    #[test]
    #[should_panic]
    fn read_chess_move_failure() {
        read_chess_move("a1e7");
    }
}
