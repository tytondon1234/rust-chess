pub mod board;
pub mod piece;

fn main() {
    let _board = board::create();
    let pieces = piece::generate_all();

    let knight = piece::build(piece::Types::Knight, piece::Sides::White, ('E', 4), 35);

    println!("{:?}", knight.legal_moves(&pieces));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rook_cannot_move_at_start_of_game() {
        let pieces = piece::generate_all();
        assert!(pieces[0].legal_moves(&pieces) == []);
    }

    #[test]
    fn a_pawn_can_move_two_spaces_at_start_of_game() {
        let pieces = piece::generate_all();
        assert!(pieces[8].legal_moves(&pieces) == [('A', 3, 0), ('A', 4, 0)]);
    }

    #[test]
    fn a_pawn_can_capture_diagnally() {
        let mut pieces = piece::generate_all();

        pieces.push(piece::build(
            piece::Types::Pawn,
            piece::Sides::Black,
            ('B', 3),
            -1,
        ));

        assert!(pieces[8].legal_moves(&pieces) == [('A', 3, 0), ('A', 4, 0), ('B', 3, 2)]);
    }

    #[test]
    fn a_rook_in_middle_of_board_at_start_of_game_can_move() {
        let pieces = piece::generate_all();
        let rook = piece::build(piece::Types::Rook, piece::Sides::White, ('E', 4), 5);
        assert!(
            rook.legal_moves(&pieces)
                == [
                    ('E', 5, 0),
                    ('E', 6, 0),
                    ('E', 7, 6),
                    ('E', 3, 0),
                    ('F', 4, 0),
                    ('G', 4, 0),
                    ('H', 4, 0),
                    ('D', 4, 0),
                    ('C', 4, 0),
                    ('B', 4, 0),
                    ('A', 4, 0)
                ]
        );
    }

    #[test]
    fn a_bishop_in_middle_of_board_at_start_of_game_can_move() {
        let pieces = piece::generate_all();
        let bishop = piece::build(piece::Types::Bishop, piece::Sides::White, ('E', 6), 3);
        assert!(
            bishop.legal_moves(&pieces)
                == [
                    ('F', 7, 4),
                    ('D', 5, 0),
                    ('C', 4, 0),
                    ('B', 3, 0),
                    ('D', 7, 4),
                    ('F', 5, 0),
                    ('G', 4, 0),
                    ('H', 3, 0)
                ]
        );
    }

    #[test]
    fn a_rook_should_take_an_unprotected_queen_if_no_checkmate_available() {
        let mut pieces = piece::generate_all();

        pieces.push(piece::build(
            piece::Types::Queen,
            piece::Sides::Black,
            ('B', 4),
            9,
        ));

        let rook = piece::build(piece::Types::Rook, piece::Sides::White, ('E', 4), 5);

        let moves = rook.legal_moves(&pieces);

        assert!(piece::best_move_out_of_these(moves) == ('B', 4, 14));
    }

    #[test]
    fn a_queen_in_middle_of_board_at_start_of_game_can_move() {
        let pieces = piece::generate_all();

        let queen = piece::build(piece::Types::Queen, piece::Sides::White, ('E', 4), 9);

        assert!(
            queen.legal_moves(&pieces)
                == [
                    ('E', 5, 0),
                    ('E', 6, 0),
                    ('E', 7, 10),
                    ('E', 3, 0),
                    ('F', 4, 0),
                    ('G', 4, 0),
                    ('H', 4, 0),
                    ('D', 4, 0),
                    ('C', 4, 0),
                    ('B', 4, 0),
                    ('A', 4, 0),
                    ('F', 5, 0),
                    ('G', 6, 0),
                    ('H', 7, 10),
                    ('D', 3, 0),
                    ('D', 5, 0),
                    ('C', 6, 0),
                    ('B', 7, 10),
                    ('F', 3, 0)
                ]
        );
    }

    #[test]
    fn a_king_in_middle_of_board_at_start_of_game_can_move() {
        let pieces = piece::generate_all();

        let king = piece::build(piece::Types::King, piece::Sides::White, ('E', 5), 0);

        assert!(
            king.legal_moves(&pieces)
                == [
                    ('E', 6, 0),
                    ('E', 4, 0),
                    ('F', 5, 0),
                    ('D', 5, 0),
                    ('F', 6, 0),
                    ('D', 4, 0),
                    ('D', 6, 0),
                    ('F', 4, 0)
                ]
        );
    }

    #[test]
    fn a_knight_in_middle_of_board_at_start_of_game_can_move() {
        let pieces = piece::generate_all();
        let knight = piece::build(piece::Types::Knight, piece::Sides::White, ('E', 6), 3);

        assert!(
            knight.legal_moves(&pieces)
                == [
                    ('F', 8, 6),
                    ('D', 8, 12),
                    ('F', 4, 0),
                    ('D', 4, 0)
                ]
        );
    }
}
