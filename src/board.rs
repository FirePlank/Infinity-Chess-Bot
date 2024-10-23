// src/board.rs
use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

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

pub struct Board {
    pub state: HashMap<Coordinate, Piece>,
    pub castling_rights: u8,
    pub en_passant: Option<Coordinate>,
    pub side_to_move: bool, // true for white, false for black
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

        // Move the piece
        let piece = self.state.remove(&from).unwrap();
        self.state.insert(to.clone(), piece);

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

        // Toggle side to move
        self.side_to_move = !self.side_to_move;
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