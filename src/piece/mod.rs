use std::cmp::Ordering;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Piece<'a> {
    pub _type: &'a str,
    pub has_moved: bool,
    pub side: &'a str,
    pub location: (char, u32),
    pub value: i32,
}

impl Piece<'_> {
    pub fn legal_moves<'a>(&self, pieces: &Vec<Piece>) -> Vec<(char, u32, i32)> {
        let (col, row) = &self.location;
        let mut moves: Vec<(char, u32, i32)> = Vec::new();

        match self._type {
            "Rook" => {
                moves = self.legal_forward_moves(*row + 1, 9, moves, pieces, &col, true);
                moves = self.legal_backward_moves(
                    if *row - 1 > 0 { *row } else { 0 },
                    0,
                    moves,
                    pieces,
                    &col,
                    true,
                );

                let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
                let current_col_index = cols.iter().position(|&c| c == *col).unwrap();

                moves = self.legal_left_to_right_moves(
                    current_col_index.try_into().unwrap(),
                    7,
                    moves,
                    pieces,
                    &row,
                    true,
                );

                moves = self.legal_right_to_left_moves(
                    current_col_index.try_into().unwrap(),
                    0,
                    moves,
                    pieces,
                    &row,
                    true,
                );
            }
            "Pawn" => {
                let from_row = *row + 1;
                let to_row = if self.has_moved { row + 2 } else { row + 3 }; // add one to the row than normal chess move to account for index 0

                moves = self.legal_forward_moves(from_row, to_row, moves, pieces, &col, false);

                let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
                let current_col_index = cols.iter().position(|&c| c == *col).unwrap();
                let index = current_col_index.try_into().unwrap();
                moves = self.legal_diag_right_to_left_moves(
                    index,
                    if index > 0 { index - 1 } else { index },
                    moves,
                    pieces,
                    &row,
                    true,
                );

                moves = self.legal_diag_left_to_right_moves(
                    index,
                    index + 1,
                    moves,
                    pieces,
                    &row,
                    true,
                );
            }
            "Bishop" => {
                let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
                let current_col_index = cols.iter().position(|&c| c == *col).unwrap();

                moves = self.legal_diag_left_to_right_moves(
                    current_col_index.try_into().unwrap(),
                    7,
                    moves,
                    pieces,
                    &row,
                    true,
                );

                moves = self.legal_diag_right_to_left_backwards_moves(
                    current_col_index.try_into().unwrap(),
                    0,
                    moves,
                    pieces,
                    &row,
                    true,
                );

                moves = self.legal_diag_right_to_left_moves(
                    current_col_index.try_into().unwrap(),
                    0,
                    moves,
                    pieces,
                    &row,
                    true,
                );

                moves = self.legal_diag_left_to_right_backwards_moves(
                    current_col_index.try_into().unwrap(),
                    7,
                    moves,
                    pieces,
                    &row,
                    true,
                );
            }
            "Queen" => {
                let from_row = *row + 1;
                let to_row = if self.has_moved { row + 2 } else { row + 3 }; // add one to the row than normal chess move to account for index 0

                moves = self.legal_forward_moves(from_row, to_row, moves, pieces, &col, false)
            }
            "King" => {
                let from_row = *row + 1;
                let to_row = if self.has_moved { row + 2 } else { row + 3 }; // add one to the row than normal chess move to account for index 0

                moves = self.legal_forward_moves(from_row, to_row, moves, pieces, &col, false)
            }
            "Knight" => {
                let from_row = *row + 1;
                let to_row = if self.has_moved { row + 2 } else { row + 3 }; // add one to the row than normal chess move to account for index 0

                moves = self.legal_forward_moves(from_row, to_row, moves, pieces, &col, false)
            }
            &_ => (),
        }
        moves
    }

    pub fn legal_forward_moves(
        &self,
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        col: &char,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        for step in from..to {
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
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        col: &char,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        for step in (to..from).rev() {
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
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        &row: &u32,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let start = if from < 8 { from + 1 } else { from };

        for step in start..to + 1 {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture (if not pawn) and is blocked
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == row {
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
                moves.push((cols[step as usize], row, 0));
            } else if capture {
                moves.push((cols[step as usize], row, captured_score));
                break;
            } else {
                break;
            }
        }

        moves
    }

    pub fn legal_right_to_left_moves(
        &self,
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        &row: &u32,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        for step in (to..from).rev() {
            // check if another piece exists
            // if friendly, blocked
            // if enemy, could capture (if not pawn) and is blocked
            // if none, can keep moving
            let mut blocked: bool = false;
            let mut capture: bool = false;
            let mut captured_score: i32 = 0;
            for piece in pieces.iter() {
                let (piece_col, piece_row) = piece.location;
                if piece_col == cols[step as usize] && piece_row == row {
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
                moves.push((cols[step as usize], row, 0));
            } else if capture {
                moves.push((cols[step as usize], row, captured_score));
                break;
            } else {
                break;
            }
        }

        moves
    }

    pub fn legal_diag_left_to_right_moves(
        &self,
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        &row: &u32,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let start = if from < 8 { from + 1 } else { from };
        let mut running_total: u32 = 1;

        for step in start..to + 1 {
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
            if !blocked && !capture && (self._type != "Pawn") {
                // a pawn must capture if going diagnally
                moves.push((cols[step as usize], row + running_total, 0));
            } else if capture {
                moves.push((cols[step as usize], row + running_total, captured_score));
                break;
            } else {
                break;
            }
            running_total = running_total + 1;
        }

        moves
    }

    pub fn legal_diag_left_to_right_backwards_moves(
        &self,
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        &row: &u32,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let start = if from < 8 { from + 1 } else { from };
        let mut running_total: u32 = 1;

        for step in start..to + 1 {
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
            running_total = running_total + 1;
        }

        moves
    }

    pub fn legal_diag_right_to_left_moves(
        &self,
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        &row: &u32,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

        let mut running_total: u32 = 1;

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
            if !blocked && !capture && (self._type != "Pawn") {
                // a pawn must capture if going diagnally
                moves.push((cols[step as usize], row + running_total, 0));
            } else if capture {
                moves.push((cols[step as usize], row + running_total, captured_score));
                break;
            } else {
                break;
            }
            running_total = running_total + 1;
        }

        moves
    }

    pub fn legal_diag_right_to_left_backwards_moves(
        &self,
        from: u32,
        to: u32,
        mut moves: Vec<(char, u32, i32)>,
        pieces: &Vec<Piece>,
        &row: &u32,
        can_capture: bool,
    ) -> Vec<(char, u32, i32)> {
        let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

        let mut running_total: u32 = 1;

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
            running_total = running_total + 1;
        }

        moves
    }
}

pub fn build<'a>(_type: &'a str, side: &'a str, location: (char, u32), value: i32) -> Piece<'a> {
    Piece {
        _type,
        has_moved: false,
        side,
        location,
        value,
    }
}

pub fn generate_all<'a>() -> Vec<Piece<'a>> {
    let mut pieces: Vec<Piece> = Vec::new();
    let pieces_data: Vec<(&str, &str, (char, u32), i32)> = vec![
        // White
        ("Rook", "White", ('A', 1), 50),
        ("Knight", "White", ('B', 1), 35),
        ("Bishop", "White", ('C', 1), 35),
        ("Queen", "White", ('D', 1), 125),
        ("King", "White", ('E', 1), 200),
        ("Knight", "White", ('F', 1), 35),
        ("Bishop", "White", ('G', 1), 30),
        ("Rook", "White", ('H', 1), 50),
        ("Pawn", "White", ('A', 2), 10),
        ("Pawn", "White", ('B', 2), 10),
        ("Pawn", "White", ('C', 2), 10),
        ("Pawn", "White", ('D', 2), 10),
        ("Pawn", "White", ('E', 2), 10),
        ("Pawn", "White", ('F', 2), 10),
        ("Pawn", "White", ('G', 2), 10),
        ("Pawn", "White", ('H', 2), 10),
        // Black
        ("Rook", "Black", ('A', 8), -50),
        ("Knight", "Black", ('B', 8), -35),
        ("Bishop", "Black", ('C', 8), -35),
        ("Queen", "Black", ('D', 8), -125),
        ("King", "Black", ('E', 8), -200),
        ("Knight", "Black", ('F', 8), -35),
        ("Bishop", "Black", ('G', 8), -30),
        ("Rook", "Black", ('H', 8), -50),
        ("Pawn", "Black", ('A', 7), -10),
        ("Pawn", "Black", ('B', 7), -10),
        ("Pawn", "Black", ('C', 7), -10),
        ("Pawn", "Black", ('D', 7), -10),
        ("Pawn", "Black", ('E', 7), -10),
        ("Pawn", "Black", ('F', 7), -10),
        ("Pawn", "Black", ('G', 7), -10),
        ("Pawn", "Black", ('H', 7), -10),
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
