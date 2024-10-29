// src/board.rs
use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use crate::r#move::{Move, MoveGen, MoveList};


pub const PIECE_VALUES: [i16; 12] = [100, 700, 300, 400, 1200, 0, 100, 700, 300, 400, 1200, 0];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn is_white(&self) -> bool {
        match self {
            Piece::WhitePawn | Piece::WhiteRook | Piece::WhiteKnight | Piece::WhiteBishop | Piece::WhiteQueen | Piece::WhiteKing => true,
            _ => false,
        }
    }

    pub fn is_black(&self) -> bool {
        match self {
            Piece::BlackPawn | Piece::BlackRook | Piece::BlackKnight | Piece::BlackBishop | Piece::BlackQueen | Piece::BlackKing => true,
            _ => false,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(pub BigInt, pub BigInt);

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Self {
        Coordinate(BigInt::from(x), BigInt::from(y))
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub state: HashMap<Coordinate, Piece>,
    pub castling_rights: u8,
    pub en_passant: Option<Coordinate>,
    pub side_to_move: bool, // true for white, false for black
    pub history: Vec<Board> // store board history for make and unmake
}

impl Board {
    pub fn new() -> Self {
        let mut state = HashMap::new();

        // Initialize white pieces
        for i in 1..=8 {
            state.insert(Coordinate::new(i, 2), Piece::WhitePawn);
        }
        state.insert(Coordinate::new(1, 1), Piece::WhiteRook);
        state.insert(Coordinate::new(8, 1), Piece::WhiteRook);
        state.insert(Coordinate::new(2, 1), Piece::WhiteKnight);
        state.insert(Coordinate::new(7, 1), Piece::WhiteKnight);
        state.insert(Coordinate::new(3, 1), Piece::WhiteBishop);
        state.insert(Coordinate::new(6, 1), Piece::WhiteBishop);
        state.insert(Coordinate::new(4, 1), Piece::WhiteQueen);
        state.insert(Coordinate::new(5, 1), Piece::WhiteKing);

        // Initialize black pieces
        for i in 1..=8 {
            state.insert(Coordinate::new(i, 7), Piece::BlackPawn);
        }
        state.insert(Coordinate::new(1, 8), Piece::BlackRook);
        state.insert(Coordinate::new(8, 8), Piece::BlackRook);
        state.insert(Coordinate::new(2, 8), Piece::BlackKnight);
        state.insert(Coordinate::new(7, 8), Piece::BlackKnight);
        state.insert(Coordinate::new(3, 8), Piece::BlackBishop);
        state.insert(Coordinate::new(6, 8), Piece::BlackBishop);
        state.insert(Coordinate::new(4, 8), Piece::BlackQueen);
        state.insert(Coordinate::new(5, 8), Piece::BlackKing);

        Board {
            state,
            castling_rights: 15, // Both sides can castle initially
            en_passant: None,
            side_to_move: true, // White starts
            history: Vec::new()
        }
    }

    pub fn empty() -> Self {
        Board {
            state: HashMap::new(),
            castling_rights: 15,
            en_passant: None,
            side_to_move: true,
            history: Vec::new()
        }
    }

    pub fn get_piece(&self, coord: &Coordinate) -> Option<&Piece> {
        self.state.get(coord)
    }

    pub fn set_piece(&mut self, coord: Coordinate, piece: Piece) {
        self.state.insert(coord, piece);
    }

    pub fn remove_piece(&mut self, coord: &Coordinate) {
        self.state.remove(coord);
    }

    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) {
        // Handle captures
        if let Some(captured_piece) = self.state.remove(&to) {
            // Handle captured piece logic if needed
        }

        // Handle en passant capture
        if let Piece::WhitePawn | Piece::BlackPawn = self.state.get(&from).unwrap() {
            if let Some(en_passant_coord) = &self.en_passant {
                if to == *en_passant_coord {
                    let capture_coord = Coordinate(to.0.clone(), from.1.clone());
                    self.state.remove(&capture_coord);
                }
            }
        }

        // Move the piece
        let piece = self.state.remove(&from).unwrap();
        self.state.insert(to.clone(), piece);

        // Handle castling
        if piece == Piece::WhiteKing && from == Coordinate::new(5, 1) {
            if to == Coordinate::new(3, 1) {
                // Long castling for white
                let rook_from = Coordinate::new(1, 1);
                let rook_to = Coordinate::new(4, 1);
                let rook = self.state.remove(&rook_from).unwrap();
                self.state.insert(rook_to, rook);
            } else if to == Coordinate::new(7, 1) {
                // Short castling for white
                let rook_from = Coordinate::new(8, 1);
                let rook_to = Coordinate::new(6, 1);
                let rook = self.state.remove(&rook_from).unwrap();
                self.state.insert(rook_to, rook);
            }
        } else if piece == Piece::BlackKing && from == Coordinate::new(5, 8) {
            if to == Coordinate::new(3, 8) {
                // Long castling for black
                let rook_from = Coordinate::new(1, 8);
                let rook_to = Coordinate::new(4, 8);
                let rook = self.state.remove(&rook_from).unwrap();
                self.state.insert(rook_to, rook);
            } else if to == Coordinate::new(7, 8) {
                // Short castling for black
                let rook_from = Coordinate::new(8, 8);
                let rook_to = Coordinate::new(6, 8);
                let rook = self.state.remove(&rook_from).unwrap();
                self.state.insert(rook_to, rook);
            }
        }

        // Update castling rights
        match piece {
            Piece::WhiteKing => self.castling_rights &= !0b1100, // White king moved
            Piece::BlackKing => self.castling_rights &= !0b0011, // Black king moved
            Piece::WhiteRook if from == Coordinate::new(1, 1) => self.castling_rights &= !0b1000, // White rook 1 moved
            Piece::WhiteRook if from == Coordinate::new(8, 1) => self.castling_rights &= !0b0100, // White rook 2 moved
            Piece::BlackRook if from == Coordinate::new(1, 8) => self.castling_rights &= !0b0010, // Black rook 1 moved
            Piece::BlackRook if from == Coordinate::new(8, 8) => self.castling_rights &= !0b0001, // Black rook 2 moved
            _ => {}
        }

        // Handle en passant
        self.en_passant = None;
        if let Piece::WhitePawn | Piece::BlackPawn = piece {
            if (from.1.clone() - to.1.clone()).abs() == BigInt::from(2) {
                self.en_passant = Some(Coordinate(from.0.clone(), (from.1 + to.1.clone()) / 2));
            }
        }
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;

        // Efficient insufficient material check
        let mut white_material = 0;
        let mut black_material = 0;
        let mut white_minor_pieces = 0;
        let mut black_minor_pieces = 0;
        let mut white_has_queen_or_rook = false;
        let mut black_has_queen_or_rook = false;

        // Early exit if pawns are present
        let mut white_has_pawn = false;
        let mut black_has_pawn = false;

        for piece in self.state.values() {
            match piece {
                Piece::WhitePawn => {
                    white_material += PIECE_VALUES[*piece as usize];
                    white_has_pawn = true;
                }
                Piece::BlackPawn => {
                    black_material += PIECE_VALUES[*piece as usize];
                    black_has_pawn = true;
                }
                // Track white's queens and rooks
                Piece::WhiteQueen | Piece::WhiteRook => {
                    white_material += PIECE_VALUES[*piece as usize];
                    white_has_queen_or_rook = true;
                }
                // Track black's queens and rooks
                Piece::BlackQueen | Piece::BlackRook => {
                    black_material += PIECE_VALUES[*piece as usize];
                    black_has_queen_or_rook = true;
                }
                // Track white's minor pieces (knights, bishops)
                Piece::WhiteKnight | Piece::WhiteBishop => {
                    white_material += PIECE_VALUES[*piece as usize];
                    white_minor_pieces += 1;
                }
                // Track black's minor pieces (knights, bishops)
                Piece::BlackKnight | Piece::BlackBishop => {
                    black_material += PIECE_VALUES[*piece as usize];
                    black_minor_pieces += 1;
                }
                _ => {}
            }
        }

        if !black_has_pawn && !white_has_pawn {
            let white_insufficient_material =
                !white_has_queen_or_rook && (white_material == 0 && white_minor_pieces <= 1);

            let black_insufficient_material =
                !black_has_queen_or_rook && (black_material == 0 && black_minor_pieces <= 1);

            // Both players have insufficient material for checkmate on infinite board
            if white_insufficient_material && black_insufficient_material {
                return 0; // Draw due to insufficient material
            }
        }

        score += white_material - black_material;

        // Calculate the score
        if { self.side_to_move } {
            // White to move
            score as i32
        } else {
            // Black to move
            -score as i32
        }
    }

    pub fn is_attacked(&mut self, coord: Coordinate, by_white: bool) -> bool {
        let mut move_list = MoveList::new();
        let side_to_move = self.side_to_move;
        self.side_to_move = by_white;
        MoveGen::generate_moves(&self, &mut move_list);
        let counted = move_list.count;

        for count in 0..counted {
            let mv = move_list.moves[count as usize].clone();
            match mv {
                Move::Normal(from, to) | Move::Promotion(from, to, _) => {
                    if to == coord {
                        self.side_to_move = side_to_move;
                        return true;
                    }
                }
                _ => {}
            }
        }

        self.side_to_move = side_to_move;
        false
    }

    pub fn king_position(&self, is_white: bool) -> Coordinate {
        for (coord, piece) in &self.state {
            if (is_white && *piece == Piece::WhiteKing) || (!is_white && *piece == Piece::BlackKing) {
                return coord.clone();
            }
        }
        self.show(true);
        panic!("King not found!");
    }

    pub fn make(&mut self, mv: Move) -> bool {
        self.history.push((*self).clone());
        // Make the move
        match mv.clone() {
            Move::Normal(from, to) => self.move_piece(from, to),
            Move::Promotion(from, to, piece) => {
                self.remove_piece(&from);
                self.set_piece(to, piece);
            }
            Move::Castling(from, to) => {
                self.move_piece(from, to);
            }
            Move::EnPassant(from, to) => {
                self.move_piece(from, to);
            }
            _ => {}
        }

        self.side_to_move = !self.side_to_move;
        // Check if the move leaves the king in check
        let king_pos = self.king_position(!self.side_to_move);
        !self.is_attacked(king_pos, self.side_to_move)
    }

    pub fn unmake(&mut self, mv: Move) {
        * self = self.history.pop().unwrap();
        // let from = Coordinate(Default::default(), Default::default());
        // let to = Coordinate(Default::default(), Default::default());
        // match mv {
        //     Move::Normal(from, to) => {
        //         if let Some(captured_piece) = self.state.remove(&to) {
        //             self.state.insert(to.clone(), captured_piece);
        //         }
        //         self.move_piece(to, from);
        //     }
        //     Move::Promotion(from, to, _) => {
        //         self.remove_piece(&to);
        //         self.set_piece(from.clone(), self.state.get(&from).unwrap().clone());
        //     }
        //     Move::Castling(from, to) => {
        //         self.move_piece(to.clone(), from.clone());
        //         if from == Coordinate::new(5, 1) {
        //             if to == Coordinate::new(3, 1) {
        //                 // Long castling for white
        //                 let rook_from = Coordinate::new(4, 1);
        //                 let rook_to = Coordinate::new(1, 1);
        //                 self.move_piece(rook_from, rook_to);
        //             } else if to == Coordinate::new(7, 1) {
        //                 // Short castling for white
        //                 let rook_from = Coordinate::new(6, 1);
        //                 let rook_to = Coordinate::new(8, 1);
        //                 self.move_piece(rook_from, rook_to);
        //             }
        //         } else if from == Coordinate::new(5, 8) {
        //             if to == Coordinate::new(3, 8) {
        //                 // Long castling for black
        //                 let rook_from = Coordinate::new(4, 8);
        //                 let rook_to = Coordinate::new(1, 8);
        //                 self.move_piece(rook_from, rook_to);
        //             } else if to == Coordinate::new(7, 8) {
        //                 // Short castling for black
        //                 let rook_from = Coordinate::new(6, 8);
        //                 let rook_to = Coordinate::new(8, 8);
        //                 self.move_piece(rook_from, rook_to);
        //             }
        //         }
        //     }
        //     Move::EnPassant(from, to) => {
        //         self.move_piece(to.clone(), from.clone());
        //         let capture_coord = Coordinate(to.0.clone(), from.1.clone());
        //         if let Some(captured_piece) = self.state.remove(&capture_coord) {
        //             self.state.insert(capture_coord, captured_piece);
        //         }
        //     }
        //     Move::InfiniteMove(coord, direction) => {
        //
        //     }
        //     _ => panic!("Invalid move type"),
        // }
        //
        // // Reverse castling rights
        // match self.state.get(&from) {
        //     Some(Piece::WhiteKing) => self.castling_rights |= 0b1100, // Restore white king castling rights
        //     Some(Piece::BlackKing) => self.castling_rights |= 0b0011, // Restore black king castling rights
        //     Some(Piece::WhiteRook) if from == Coordinate::new(1, 1) => self.castling_rights |= 0b1000, // Restore white rook 1 castling rights
        //     Some(Piece::WhiteRook) if from == Coordinate::new(8, 1) => self.castling_rights |= 0b0100, // Restore white rook 2 castling rights
        //     Some(Piece::BlackRook) if from == Coordinate::new(1, 8) => self.castling_rights |= 0b0010, // Restore black rook 1 castling rights
        //     Some(Piece::BlackRook) if from == Coordinate::new(8, 8) => self.castling_rights |= 0b0001, // Restore black rook 2 castling rights
        //     _ => {} // Handle other cases or do nothing
        // }
        //
        // // Restore en passant square
        // if let Some(Piece::WhitePawn) | Some(Piece::BlackPawn) = self.state.get(&from) {
        //     if (from.1.clone() - to.1.clone()).abs() == BigInt::from(2) {
        //         self.en_passant = Some(Coordinate(from.0.clone(), (from.1 + to.1.clone()) / 2));
        //     }
        // }
    }

    pub fn show(&self, unicode: bool) {
        if self.state.is_empty() {
            println!("The board is empty.");
            return;
        }

        let min_y = self.state.keys().map(|coord| coord.1.clone()).min().unwrap();
        let max_y = self.state.keys().map(|coord| coord.1.clone()).max().unwrap();
        let min_x = self.state.keys().map(|coord| coord.0.clone()).min().unwrap();
        let max_x = self.state.keys().map(|coord| coord.0.clone()).max().unwrap();

        let mut rank = max_y.clone();
        while rank >= min_y {
            let mut file = min_x.clone();
            while file <= max_x {
                let coord = Coordinate(file.clone(), rank.clone());
                if file == min_x {
                    print!("{:2} ", rank);
                }
                if let Some(piece) = self.get_piece(&coord) {
                    if unicode {
                        print!("{} ", match piece {
                            Piece::WhitePawn => "♙",
                            Piece::WhiteRook => "♖",
                            Piece::WhiteKnight => "♘",
                            Piece::WhiteBishop => "♗",
                            Piece::WhiteQueen => "♕",
                            Piece::WhiteKing => "♔",
                            Piece::BlackPawn => "♟",
                            Piece::BlackRook => "♜",
                            Piece::BlackKnight => "♞",
                            Piece::BlackBishop => "♝",
                            Piece::BlackQueen => "♛",
                            Piece::BlackKing => "♚",
                        });
                    } else {
                        print!("{:?} ", piece);
                    }
                } else {
                    print!(". ");
                }
                file += 1;
            }
            println!();
            rank -= 1;
        }

        print!("   ");
        let mut file = min_x.clone();
        while file <= max_x {
            print!("{:2} ", file);
            file += 1;
        }
        println!();

        println!("Side to move: {}", if self.side_to_move { "White" } else { "Black" });
        println!("Castling rights: {:?}", self.castling_rights);
        println!("En passant: {:?}", self.en_passant);
    }
}