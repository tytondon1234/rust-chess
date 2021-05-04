#[path = "../board/mod.rs"]
pub mod board;

use std::cmp::Ordering;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Piece {
    pub _type: Types,
    pub has_moved: bool,
    pub side: Sides,
    pub location: (char, u32),
    pub value: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Types {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sides {
    White,
    Black,
}

impl Piece {
    pub fn legal_moves(&self, pieces: &[Piece]) -> Vec<(char, u32, i32)> {
        let mut moves: Vec<(char, u32, i32)> = Vec::new();

        match self._type {
            Types::Rook => {
                moves = self.legal_accross_all(moves, pieces);
            }
            Types::Pawn => {
                moves = self.legal_forward_moves(moves, pieces, false);

                moves = self.legal_diag_right_to_left_moves(moves, pieces, true);

                moves = self.legal_diag_left_to_right_moves(moves, pieces, true);
            }
            Types::Bishop => {
                moves = self.legal_diag_all(moves, pieces);
            }
            Types::Queen => {
                moves = self.legal_accross_all(moves, pieces);

                moves = self.legal_diag_all(moves, pieces);
            }
            Types::King => {
                moves = self.legal_accross_all(moves, pieces);

                moves = self.legal_diag_all(moves, pieces);
            }
            Types::Knight => moves = self.legal_l_moves(moves, pieces),
        }
        moves
    }

    pub fn legal_diag_all(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
    ) -> Vec<(char, u32, i32)> {
        moves = self.legal_diag_left_to_right_moves(moves, pieces, true);

        moves = self.legal_diag_right_to_left_backwards_moves(moves, pieces, true);

        moves = self.legal_diag_right_to_left_moves(moves, pieces, true);

        moves = self.legal_diag_left_to_right_backwards_moves(moves, pieces, true);

        moves
    }

    pub fn legal_accross_all(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
    ) -> Vec<(char, u32, i32)> {
        moves = self.legal_forward_moves(moves, pieces, true);

        moves = self.legal_backward_moves(moves, pieces, true);

        moves = self.legal_left_to_right_moves(moves, pieces, true);

        moves = self.legal_right_to_left_moves(moves, pieces, true);
        moves
    }

    pub fn legal_forward_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;
        let to_row: u32 = match &self._type {
            Types::Pawn => {
                if self.has_moved {
                    row + 2
                } else {
                    row + 3
                }
            }
            Types::King => row + 2,
            _ => 9,
        };
        for step in row + 1..to_row {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture (if not pawn) and is blocked
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == *col && piece_row == step {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture {
                moves.push((*col, step, 0));
            } else if capture {
                moves.push((*col, step, captured_score));
                break;
            } else {
                break;
            }
        }

        moves
    }

    pub fn legal_backward_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;
        let to_row: u32 = if *row - 1 > 0 { *row } else { 0 };
        let from_row: u32 = match &self._type {
            Types::King => {
                if *row - 1 > 0 {
                    *row - 1
                } else {
                    0
                }
            }
            _ => 0,
        };
        for step in (from_row..to_row).rev() {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture (if not pawn) and is blocked
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == *col && piece_row == step {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture {
                moves.push((*col, step, 0));
            } else if capture {
                moves.push((*col, step, captured_score));
                break;
            } else {
                break;
            }
        }

        moves
    }

    // Yeah...I see you knights...
    pub fn legal_l_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
    ) -> Vec<(char, u32, i32)> {
        moves = self.legal_l_forward_moves(moves, pieces);

        moves = self.legal_l_backward_moves(moves, pieces);

        moves
    }
    pub fn legal_l_forward_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let can_move_forward: bool = if *row + 2 <= 8 { true } else { false };

        if can_move_forward {
            let forward_row: u32 = *row + 2;

            let cols: Vec<char> = board::cols();

            let current_col_index = cols.iter().position(|&c| c == *col).unwrap();

            let unwraped_index: usize = current_col_index.try_into().unwrap(); // This feels yucky

            let mut forward_l_cols = vec![];

            if unwraped_index + 1 <= 8 {
                forward_l_cols.push(cols[unwraped_index + 1]);
            }

            if unwraped_index - 1 > 0 {
                forward_l_cols.push(cols[unwraped_index - 1]);
            }

            for col in forward_l_cols {
                let mut capture: bool = false;
                let mut captured_score: i32 = 0;
                let mut blocked: bool = false;

                for piece in pieces.iter() {
                    let (piece_col, piece_row) = piece.location;
                    if piece_col == col && piece_row == forward_row {
                        if piece.side == self.side {
                            // friendly, blocked
                            blocked = true;
                            break;
                        } else {
                            // enemy, can capture
                            capture = true;
                            captured_score = self.value.abs() + piece.value.abs();
                            break;
                        }
                    }
                }
                if !blocked && !capture {
                    moves.push((col, forward_row, 0));
                } else if capture {
                    moves.push((col, forward_row, captured_score));
                } else {
                    break;
                }
            }
        }

        moves
    }

    pub fn legal_l_backward_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let can_move_backward: bool = if *row - 2 > 0 { true } else { false };

        if can_move_backward {
            let backward_row: u32 = *row - 2;

            let cols: Vec<char> = board::cols();

            let current_col_index = cols.iter().position(|&c| c == *col).unwrap();

            let unwraped_index: usize = current_col_index.try_into().unwrap(); // This feels yucky

            let mut backward_l_cols = vec![];

            if unwraped_index + 1 <= 8 {
                backward_l_cols.push(cols[unwraped_index + 1]);
            }

            if unwraped_index - 1 > 0 {
                backward_l_cols.push(cols[unwraped_index - 1]);
            }

            for col in backward_l_cols {
                let mut capture: bool = false;
                let mut captured_score: i32 = 0;
                let mut blocked: bool = false;

                for piece in pieces.iter() {
                    let (piece_col, piece_row) = piece.location;
                    if piece_col == col && piece_row == backward_row {
                        if piece.side == self.side {
                            // friendly, blocked
                            blocked = true;
                            break;
                        } else {
                            // enemy, can capture
                            capture = true;
                            captured_score = self.value.abs() + piece.value.abs();
                            break;
                        }
                    }
                }
                if !blocked && !capture {
                    moves.push((col, backward_row, 0));
                } else if capture {
                    moves.push((col, backward_row, captured_score));
                } else {
                    break;
                }
            }
        }

        moves
    }

    pub fn legal_left_to_right_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let cols: Vec<char> = board::cols();

        let current_col_index = cols.iter().position(|&c| c == *col).unwrap();

        let unwraped_index = current_col_index.try_into().unwrap(); // This feels yucky

        let start = if unwraped_index < 8 {
            unwraped_index + 1
        } else {
            unwraped_index
        };

        let to_row: u32 = match &self._type {
            Types::King => start as u32 + 1,
            _ => 8,
        };

        for step in start..to_row {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture (if not pawn) and is blocked
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == *row {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture {
                moves.push((cols[step as usize], *row, 0));
            } else if capture {
                moves.push((cols[step as usize], *row, captured_score));
                break;
            } else {
                break;
            }
        }

        moves
    }

    pub fn legal_right_to_left_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let cols: Vec<char> = board::cols();

        let current_col_index = cols.iter().position(|&c| c == *col).unwrap();

        let unwraped_index = current_col_index.try_into().unwrap(); // This feels yucky

        let from_row: u32 = match &self._type {
            Types::King => current_col_index as u32 - 1,
            _ => 0,
        };

        for step in (from_row..unwraped_index).rev() {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture (if not pawn) and is blocked
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == *row {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture {
                moves.push((cols[step as usize], *row, 0));
            } else if capture {
                moves.push((cols[step as usize], *row, captured_score));
                break;
            } else {
                break;
            }
        }

        moves
    }

    pub fn legal_diag_left_to_right_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let cols: Vec<char> = board::cols();

        let mut running_total: u32 = 1;

        let index: u32 = (cols.iter().position(|&c| c == *col).unwrap())
            .try_into()
            .unwrap();

        let to: u32 = match &self._type {
            Types::Pawn => {
                if index < 8 {
                    index + 2
                } else {
                    index
                }
            }
            Types::King => {
                if index < 8 {
                    index + 2
                } else {
                    index
                }
            }
            _ => 8,
        };

        let from = if index < 7 { index + 1 } else { index };

        for step in from..to {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == row + running_total {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture && (self._type != Types::Pawn) {
                // a pawn must capture if going diagnally
                moves.push((cols[step as usize], row + running_total, 0));
            } else if capture {
                moves.push((cols[step as usize], row + running_total, captured_score));
                break;
            } else {
                break;
            }
            running_total += 1;
        }

        moves
    }

    pub fn legal_diag_left_to_right_backwards_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let cols: Vec<char> = board::cols();

        let index: u32 = (cols.iter().position(|&c| c == *col).unwrap())
            .try_into()
            .unwrap();

        let from = if index < 7 { index + 1 } else { index };

        let to: u32 = match &self._type {
            Types::King => {
                if index < 8 {
                    index + 2
                } else {
                    index
                }
            }
            _ => 8,
        };

        let mut running_total: u32 = 1;

        for step in from..to {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == row - running_total {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture {
                moves.push((cols[step as usize], row - running_total, 0));
            } else if capture {
                moves.push((cols[step as usize], row - running_total, captured_score));
                break;
            } else {
                break;
            }
            running_total += 1;
        }

        moves
    }

    pub fn legal_diag_right_to_left_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let cols: Vec<char> = board::cols();

        let mut running_total: u32 = 1;

        let index: u32 = (cols.iter().position(|&c| c == *col).unwrap())
            .try_into()
            .unwrap();

        let to: u32 = match &self._type {
            Types::Pawn => {
                if index > 0 {
                    index - 1
                } else {
                    index
                }
            }
            Types::King => {
                if index > 0 {
                    index - 1
                } else {
                    index
                }
            }
            _ => 0,
        };

        for step in (to..index).rev() {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == row + running_total {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture && (self._type != Types::Pawn) {
                // a pawn must capture if going diagnally
                moves.push((cols[step as usize], row + running_total, 0));
            } else if capture {
                moves.push((cols[step as usize], row + running_total, captured_score));
                break;
            } else {
                break;
            }
            running_total += 1;
        }

        moves
    }

    pub fn legal_diag_right_to_left_backwards_moves(
        &self,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &[Piece],
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;

        let cols: Vec<char> = board::cols();

        let mut running_total: u32 = 1;

        let to: u32 = (cols.iter().position(|&c| c == *col).unwrap())
            .try_into()
            .unwrap();

        let from: u32 = match &self._type {
            Types::King => {
                if to > 0 {
                    to - 1
                } else {
                    to
                }
            }
            _ => 0,
        };

        for step in (from..to).rev() {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == row - running_total {
                    if piece.side == self.side || !can_capture {
                        // friendly, blocked or can't capture and blocked
                        blocked = true;
                        break;
                    } else if can_capture {
                        // enemy, can capture
                        capture = true;
                        captured_score = self.value.abs() + piece.value.abs();
                        break;
                    }
                }
            }
            if !blocked && !capture {
                moves.push((cols[step as usize], row - running_total, 0));
            } else if capture {
                moves.push((cols[step as usize], row - running_total, captured_score));
                break;
            } else {
                break;
            }
            running_total += 1;
        }

        moves
    }
}

pub fn build(_type: Types, side: Sides, location: (char, u32), value: i32) -> Piece {
    Piece {
        _type,
        has_moved: false,
        side,
        location,
        value,
    }
}

pub fn generate_all() -> Vec<Piece> {
    let mut pieces: Vec<Piece> = Vec::new();
    let pieces_data: Vec<(Types, Sides, (char, u32), i32)> = vec![
        // White
        (Types::Rook, Sides::White, ('A', 1), 5),
        (Types::Knight, Sides::White, ('B', 1), 3),
        (Types::Bishop, Sides::White, ('C', 1), 3),
        (Types::Queen, Sides::White, ('D', 1), 9),
        (Types::King, Sides::White, ('E', 1), 0),
        (Types::Knight, Sides::White, ('F', 1), 3),
        (Types::Bishop, Sides::White, ('G', 1), 3),
        (Types::Rook, Sides::White, ('H', 1), 5),
        (Types::Pawn, Sides::White, ('A', 2), 1),
        (Types::Pawn, Sides::White, ('B', 2), 1),
        (Types::Pawn, Sides::White, ('C', 2), 1),
        (Types::Pawn, Sides::White, ('D', 2), 1),
        (Types::Pawn, Sides::White, ('E', 2), 1),
        (Types::Pawn, Sides::White, ('F', 2), 1),
        (Types::Pawn, Sides::White, ('G', 2), 1),
        (Types::Pawn, Sides::White, ('H', 2), 1),
        // Black
        (Types::Rook, Sides::Black, ('A', 8), -5),
        (Types::Knight, Sides::Black, ('B', 8), -3),
        (Types::Bishop, Sides::Black, ('C', 8), -3),
        (Types::Queen, Sides::Black, ('D', 8), -9),
        (Types::King, Sides::Black, ('E', 8), -0),
        (Types::Knight, Sides::Black, ('F', 8), -3),
        (Types::Bishop, Sides::Black, ('G', 8), -3),
        (Types::Rook, Sides::Black, ('H', 8), -5),
        (Types::Pawn, Sides::Black, ('A', 7), -1),
        (Types::Pawn, Sides::Black, ('B', 7), -1),
        (Types::Pawn, Sides::Black, ('C', 7), -1),
        (Types::Pawn, Sides::Black, ('D', 7), -1),
        (Types::Pawn, Sides::Black, ('E', 7), -1),
        (Types::Pawn, Sides::Black, ('F', 7), -1),
        (Types::Pawn, Sides::Black, ('G', 7), -1),
        (Types::Pawn, Sides::Black, ('H', 7), -1),
    ];
    for piece in pieces_data.iter() {
        let (_type, side, location, value) = piece;
        pieces.push(build(*_type, *side, *location, *value));
    }

    pieces
}

pub fn best_move_out_of_these(moves: Vec<(char, u32, i32)>) -> (char, u32, i32) {
    let index_of_best_move: Option<usize> = moves
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            let (_, _, a_value) = a;
            let (_, _, b_value) = b;
            a_value.partial_cmp(b_value).unwrap_or(Ordering::Equal)
        })
        .map(|(index, _)| index);

    moves[index_of_best_move.unwrap()]
}
