use crate::board::{Board, Coordinate, Piece};
use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::Signed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    Normal(Coordinate, Coordinate),
    Castling(Coordinate, Coordinate),
    EnPassant(Coordinate, Coordinate),
    Promotion(Coordinate, Coordinate, Piece),
    InfiniteMove(Coordinate, Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

pub struct MoveGen;

impl MoveGen {
    pub fn generate_moves(board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        for (coord, piece) in &board.state {
            if (board.side_to_move && piece.is_white()) || (!board.side_to_move && piece.is_black()) {
                match piece {
                    Piece::WhitePawn | Piece::BlackPawn => {
                        moves.extend(Self::generate_pawn_moves(board, coord.clone(), *piece));
                    }
                    Piece::WhiteRook | Piece::BlackRook => {
                        moves.extend(Self::generate_rook_moves(board, coord.clone(), *piece));
                    }
                    Piece::WhiteKnight | Piece::BlackKnight => {
                        moves.extend(Self::generate_knight_moves(board, coord.clone(), *piece));
                    }
                    Piece::WhiteBishop | Piece::BlackBishop => {
                        moves.extend(Self::generate_bishop_moves(board, coord.clone(), *piece));
                    }
                    Piece::WhiteQueen | Piece::BlackQueen => {
                        moves.extend(Self::generate_queen_moves(board, coord.clone(), *piece));
                    }
                    Piece::WhiteKing | Piece::BlackKing => {
                        moves.extend(Self::generate_king_moves(board, coord.clone(), *piece));
                    }
                }
            }
        }
        moves
    }

    fn generate_pawn_moves(board: &Board, coord: Coordinate, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let direction = if piece == Piece::WhitePawn { 1 } else { -1 };
        let start_row = if piece == Piece::WhitePawn { 2 } else { 7 };
        let promotion_row = if piece == Piece::WhitePawn { 8 } else { 1 };

        // Single move forward
        let forward = Coordinate(coord.0.clone(), coord.1.clone() + direction);
        if board.get_piece(&forward).is_none() {
            if forward.1 == BigInt::from(promotion_row) {
                moves.push(Move::Promotion(coord.clone(), forward.clone(), Piece::WhiteQueen));
                moves.push(Move::Promotion(coord.clone(), forward.clone(), Piece::WhiteRook));
                moves.push(Move::Promotion(coord.clone(), forward.clone(), Piece::WhiteKnight));
                moves.push(Move::Promotion(coord.clone(), forward.clone(), Piece::WhiteBishop));
            } else {
                moves.push(Move::Normal(coord.clone(), forward.clone()));
            }
        }

        // Double move forward
        if coord.1 == BigInt::from(start_row) {
            let double_forward = Coordinate(coord.0.clone(), coord.1.clone() + 2 * direction);
            if board.get_piece(&double_forward).is_none() && board.get_piece(&forward).is_none() {
                moves.push(Move::Normal(coord.clone(), double_forward));
            }
        }

        // Capture moves
        let capture_directions = [1, -1];
        for &dx in &capture_directions {
            let capture = Coordinate(coord.0.clone() + BigInt::from(dx), coord.1.clone() + BigInt::from(direction));
            if let Some(target_piece) = board.get_piece(&capture) {
                if Self::is_opponent_piece(piece, *target_piece) {
                    if capture.1 == BigInt::from(promotion_row) {
                        moves.push(Move::Promotion(coord.clone(), capture.clone(), Piece::WhiteQueen));
                        moves.push(Move::Promotion(coord.clone(), capture.clone(), Piece::WhiteRook));
                        moves.push(Move::Promotion(coord.clone(), capture.clone(), Piece::WhiteKnight));
                        moves.push(Move::Promotion(coord.clone(), capture.clone(), Piece::WhiteBishop));
                    } else {
                        moves.push(Move::Normal(coord.clone(), capture.clone()));
                    }
                }
            } else if let Some(en_passant) = board.en_passant.clone() {
                if capture == en_passant {
                    moves.push(Move::EnPassant(coord.clone(), capture.clone()));
                }
            }
        }

        moves
    }

    fn generate_rook_moves(board: &Board, coord: Coordinate, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let directions = [
            (0, 1),  // Up
            (0, -1), // Down
            (1, 0),  // Right
            (-1, 0), // Left
        ];

        for &(dx, dy) in &directions {
            let mut path_clear = true;
            let mut closest_piece: Option<(&Coordinate, &Piece)> = None;

            for (target_coord, target_piece) in &board.state {
                if (dx == 0 && target_coord.0 == coord.0 && ((dy > 0 && target_coord.1 > coord.1) || (dy < 0 && target_coord.1 < coord.1))) ||
                    (dy == 0 && target_coord.1 == coord.1 && ((dx > 0 && target_coord.0 > coord.0) || (dx < 0 && target_coord.0 < coord.0))) {
                    if closest_piece.is_none() ||
                        ((dx == 0 && (target_coord.1.clone() - coord.1.clone()).abs() < (closest_piece.unwrap().0.1.clone() - coord.1.clone()).abs()) ||
                            (dy == 0 && (target_coord.0.clone() - coord.0.clone()).abs() < (closest_piece.unwrap().0.0.clone() - coord.0.clone()).abs())) {
                        closest_piece = Some((target_coord, target_piece));
                    }
                }
            }

            if let Some((target_coord, target_piece)) = closest_piece {
                if Self::is_opponent_piece(piece, *target_piece) {
                    moves.push(Move::Normal(coord.clone(), target_coord.clone()));
                }
                path_clear = false;
            }

            if path_clear {
                let infinite_move = match (dx, dy) {
                    (0, 1) => Direction::Top,
                    (0, -1) => Direction::Bottom,
                    (1, 0) => Direction::Right,
                    (-1, 0) => Direction::Left,
                    _ => unreachable!(),
                };
                moves.push(Move::InfiniteMove(coord.clone(), infinite_move));
            }
        }

        moves
    }

    fn generate_bishop_moves(board: &Board, coord: Coordinate, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let directions = [
            (1, 1),   // Top-right
            (1, -1),  // Bottom-right
            (-1, 1),  // Top-left
            (-1, -1), // Bottom-left
        ];

        for &(dx, dy) in &directions {
            let mut path_clear = true;
            let mut closest_piece: Option<(&Coordinate, &Piece)> = None;

            for (target_coord, target_piece) in &board.state {
                if (target_coord.0.clone() - coord.0.clone()).abs() == (target_coord.1.clone() - coord.1.clone()).abs() &&
                    ((dx > 0 && target_coord.0 > coord.0) || (dx < 0 && target_coord.0 < coord.0)) &&
                    ((dy > 0 && target_coord.1 > coord.1) || (dy < 0 && target_coord.1 < coord.1)) {
                    if closest_piece.is_none() ||
                        ((target_coord.0.clone() - coord.0.clone()).abs() < (closest_piece.unwrap().0.0.clone() - coord.0.clone()).abs()) {
                        closest_piece = Some((target_coord, target_piece));
                    }
                }
            }

            if let Some((target_coord, target_piece)) = closest_piece {
                if Self::is_opponent_piece(piece, *target_piece) {
                    moves.push(Move::Normal(coord.clone(), target_coord.clone()));
                }
                path_clear = false;
            }

            if path_clear {
                let infinite_move = match (dx, dy) {
                    (1, 1) => Direction::TopRight,
                    (1, -1) => Direction::BottomRight,
                    (-1, 1) => Direction::TopLeft,
                    (-1, -1) => Direction::BottomLeft,
                    _ => unreachable!(),
                };
                moves.push(Move::InfiniteMove(coord.clone(), infinite_move));
            }
        }

        moves
    }

    fn generate_knight_moves(board: &Board, coord: Coordinate, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2),
        ];

        for &(dx, dy) in &knight_moves {
            let next_coord = Coordinate(coord.0.clone() + dx, coord.1.clone() + dy);
            if let Some(target_piece) = board.get_piece(&next_coord) {
                if Self::is_opponent_piece(piece, *target_piece) {
                    moves.push(Move::Normal(coord.clone(), next_coord.clone()));
                }
            } else {
                moves.push(Move::Normal(coord.clone(), next_coord.clone()));
            }
        }

        moves
    }

    fn generate_queen_moves(board: &Board, coord: Coordinate, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        moves.extend(Self::generate_rook_moves(board, coord.clone(), piece));
        moves.extend(Self::generate_bishop_moves(board, coord.clone(), piece));
        moves
    }

    fn generate_king_moves(board: &Board, coord: Coordinate, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let king_moves = [
            (1, 0), (1, 1), (0, 1), (-1, 1),
            (-1, 0), (-1, -1), (0, -1), (1, -1),
        ];

        for &(dx, dy) in &king_moves {
            let next_coord = Coordinate(coord.0.clone() + BigInt::from(dx), coord.1.clone() + BigInt::from(dy));
            if let Some(target_piece) = board.get_piece(&next_coord) {
                if Self::is_opponent_piece(piece, *target_piece) {
                    moves.push(Move::Normal(coord.clone(), next_coord.clone()));
                }
            } else {
                moves.push(Move::Normal(coord.clone(), next_coord.clone()));
            }
        }

        // Castling logic
        if piece == Piece::WhiteKing && coord == Coordinate::new(5, 1) {
            if board.castling_rights & 0b1000 != 0 && board.get_piece(&Coordinate::new(6, 1)).is_none() && board.get_piece(&Coordinate::new(7, 1)).is_none() {
                moves.push(Move::Castling(coord.clone(), Coordinate::new(7, 1)));
            }
            if board.castling_rights & 0b0100 != 0 && board.get_piece(&Coordinate::new(4, 1)).is_none() && board.get_piece(&Coordinate::new(3, 1)).is_none() && board.get_piece(&Coordinate::new(2, 1)).is_none() {
                moves.push(Move::Castling(coord.clone(), Coordinate::new(3, 1)));
            }
        } else if piece == Piece::BlackKing && coord == Coordinate::new(5, 8) {
            if board.castling_rights & 0b0010 != 0 && board.get_piece(&Coordinate::new(6, 8)).is_none() && board.get_piece(&Coordinate::new(7, 8)).is_none() {
                moves.push(Move::Castling(coord.clone(), Coordinate::new(7, 8)));
            }
            if board.castling_rights & 0b0001 != 0 && board.get_piece(&Coordinate::new(4, 8)).is_none() && board.get_piece(&Coordinate::new(3, 8)).is_none() && board.get_piece(&Coordinate::new(2, 8)).is_none() {
                moves.push(Move::Castling(coord.clone(), Coordinate::new(3, 8)));
            }
        }

        moves
    }

    fn is_opponent_piece(piece: Piece, target_piece: Piece) -> bool {
        matches!(
            (piece, target_piece),
            (Piece::WhitePawn, Piece::BlackPawn)
                | (Piece::WhitePawn, Piece::BlackRook)
                | (Piece::WhitePawn, Piece::BlackKnight)
                | (Piece::WhitePawn, Piece::BlackBishop)
                | (Piece::WhitePawn, Piece::BlackQueen)
                | (Piece::WhitePawn, Piece::BlackKing)
                | (Piece::WhiteRook, Piece::BlackPawn)
                | (Piece::WhiteRook, Piece::BlackRook)
                | (Piece::WhiteRook, Piece::BlackKnight)
                | (Piece::WhiteRook, Piece::BlackBishop)
                | (Piece::WhiteRook, Piece::BlackQueen)
                | (Piece::WhiteRook, Piece::BlackKing)
                | (Piece::WhiteKnight, Piece::BlackPawn)
                | (Piece::WhiteKnight, Piece::BlackRook)
                | (Piece::WhiteKnight, Piece::BlackKnight)
                | (Piece::WhiteKnight, Piece::BlackBishop)
                | (Piece::WhiteKnight, Piece::BlackQueen)
                | (Piece::WhiteKnight, Piece::BlackKing)
                | (Piece::WhiteBishop, Piece::BlackPawn)
                | (Piece::WhiteBishop, Piece::BlackRook)
                | (Piece::WhiteBishop, Piece::BlackKnight)
                | (Piece::WhiteBishop, Piece::BlackBishop)
                | (Piece::WhiteBishop, Piece::BlackQueen)
                | (Piece::WhiteBishop, Piece::BlackKing)
                | (Piece::WhiteQueen, Piece::BlackPawn)
                | (Piece::WhiteQueen, Piece::BlackRook)
                | (Piece::WhiteQueen, Piece::BlackKnight)
                | (Piece::WhiteQueen, Piece::BlackBishop)
                | (Piece::WhiteQueen, Piece::BlackQueen)
                | (Piece::WhiteQueen, Piece::BlackKing)
                | (Piece::WhiteKing, Piece::BlackPawn)
                | (Piece::WhiteKing, Piece::BlackRook)
                | (Piece::WhiteKing, Piece::BlackKnight)
                | (Piece::WhiteKing, Piece::BlackBishop)
                | (Piece::WhiteKing, Piece::BlackQueen)
                | (Piece::WhiteKing, Piece::BlackKing)
                | (Piece::BlackPawn, Piece::WhitePawn)
                | (Piece::BlackPawn, Piece::WhiteRook)
                | (Piece::BlackPawn, Piece::WhiteKnight)
                | (Piece::BlackPawn, Piece::WhiteBishop)
                | (Piece::BlackPawn, Piece::WhiteQueen)
                | (Piece::BlackPawn, Piece::WhiteKing)
                | (Piece::BlackRook, Piece::WhitePawn)
                | (Piece::BlackRook, Piece::WhiteRook)
                | (Piece::BlackRook, Piece::WhiteKnight)
                | (Piece::BlackRook, Piece::WhiteBishop)
                | (Piece::BlackRook, Piece::WhiteQueen)
                | (Piece::BlackRook, Piece::WhiteKing)
                | (Piece::BlackKnight, Piece::WhitePawn)
                | (Piece::BlackKnight, Piece::WhiteRook)
                | (Piece::BlackKnight, Piece::WhiteKnight)
                | (Piece::BlackKnight, Piece::WhiteBishop)
                | (Piece::BlackKnight, Piece::WhiteQueen)
                | (Piece::BlackKnight, Piece::WhiteKing)
                | (Piece::BlackBishop, Piece::WhitePawn)
                | (Piece::BlackBishop, Piece::WhiteRook)
                | (Piece::BlackBishop, Piece::WhiteKnight)
                | (Piece::BlackBishop, Piece::WhiteBishop)
                | (Piece::BlackBishop, Piece::WhiteQueen)
                | (Piece::BlackBishop, Piece::WhiteKing)
                | (Piece::BlackQueen, Piece::WhitePawn)
                | (Piece::BlackQueen, Piece::WhiteRook)
                | (Piece::BlackQueen, Piece::WhiteKnight)
                | (Piece::BlackQueen, Piece::WhiteBishop)
                | (Piece::BlackQueen, Piece::WhiteQueen)
                | (Piece::BlackQueen, Piece::WhiteKing)
                | (Piece::BlackKing, Piece::WhitePawn)
                | (Piece::BlackKing, Piece::WhiteRook)
                | (Piece::BlackKing, Piece::WhiteKnight)
                | (Piece::BlackKing, Piece::WhiteBishop)
                | (Piece::BlackKing, Piece::WhiteQueen)
                | (Piece::BlackKing, Piece::WhiteKing)
        )
    }
}