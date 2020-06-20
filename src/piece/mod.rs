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
                moves = self.legal_forward_moves(moves, pieces, true);
                moves = self.legal_backward_moves(moves, pieces, true);
                moves = self.legal_left_to_right_moves(moves, pieces, true);
                moves = self.legal_right_to_left_moves(moves, pieces, true);
            }
            Types::Pawn => {
                moves = self.legal_forward_moves(moves, pieces, false);

                moves = self.legal_diag_right_to_left_moves(moves, pieces, true);

                moves = self.legal_diag_left_to_right_moves(moves, pieces, true);
            }
            Types::Bishop => {
                moves = self.legal_diag_left_to_right_moves(moves, pieces, true);

                moves = self.legal_diag_right_to_left_backwards_moves(moves, pieces, true);

                moves = self.legal_diag_right_to_left_moves(moves, pieces, true);

                moves = self.legal_diag_left_to_right_backwards_moves(moves, pieces, true);
            }
            Types::Queen => moves = self.legal_forward_moves(moves, pieces, false),
            Types::King => moves = self.legal_forward_moves(moves, pieces, false),
            Types::Knight => moves = self.legal_forward_moves(moves, pieces, false),
        }
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
                        captured_score = self.value + piece.value;
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
        for step in (0..to_row).rev() {
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
                        captured_score = self.value + piece.value;
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

        for step in start..8 {
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
                        captured_score = self.value + piece.value;
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

        for step in (0..unwraped_index).rev() {
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
                        captured_score = self.value + piece.value;
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
                        captured_score = self.value + piece.value;
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

        let to: u32 = 8;

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
                        captured_score = self.value + piece.value;
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
                        captured_score = self.value + piece.value;
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

        let from: u32 = (cols.iter().position(|&c| c == *col).unwrap())
            .try_into()
            .unwrap();

        let to: u32 = 0;

        for step in (to..from).rev() {
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
                        captured_score = self.value + piece.value;
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
        (Types::Rook, Sides::White, ('A', 1), 50),
        (Types::Knight, Sides::White, ('B', 1), 35),
        (Types::Bishop, Sides::White, ('C', 1), 35),
        (Types::Queen, Sides::White, ('D', 1), 125),
        (Types::King, Sides::White, ('E', 1), 200),
        (Types::Knight, Sides::White, ('F', 1), 35),
        (Types::Bishop, Sides::White, ('G', 1), 30),
        (Types::Rook, Sides::White, ('H', 1), 50),
        (Types::Pawn, Sides::White, ('A', 2), 10),
        (Types::Pawn, Sides::White, ('B', 2), 10),
        (Types::Pawn, Sides::White, ('C', 2), 10),
        (Types::Pawn, Sides::White, ('D', 2), 10),
        (Types::Pawn, Sides::White, ('E', 2), 10),
        (Types::Pawn, Sides::White, ('F', 2), 10),
        (Types::Pawn, Sides::White, ('G', 2), 10),
        (Types::Pawn, Sides::White, ('H', 2), 10),
        // Black
        (Types::Rook, Sides::Black, ('A', 8), -50),
        (Types::Knight, Sides::Black, ('B', 8), -35),
        (Types::Bishop, Sides::Black, ('C', 8), -35),
        (Types::Queen, Sides::Black, ('D', 8), -125),
        (Types::King, Sides::Black, ('E', 8), -200),
        (Types::Knight, Sides::Black, ('F', 8), -35),
        (Types::Bishop, Sides::Black, ('G', 8), -30),
        (Types::Rook, Sides::Black, ('H', 8), -50),
        (Types::Pawn, Sides::Black, ('A', 7), -10),
        (Types::Pawn, Sides::Black, ('B', 7), -10),
        (Types::Pawn, Sides::Black, ('C', 7), -10),
        (Types::Pawn, Sides::Black, ('D', 7), -10),
        (Types::Pawn, Sides::Black, ('E', 7), -10),
        (Types::Pawn, Sides::Black, ('F', 7), -10),
        (Types::Pawn, Sides::Black, ('G', 7), -10),
        (Types::Pawn, Sides::Black, ('H', 7), -10),
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
