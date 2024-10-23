mod board;
mod movegen;

use num_bigint::BigInt;
use board::{Board, Coordinate, Piece};
use movegen::{MoveGen, Move};

fn main() {
    let mut board = Board::new();
    board.move_piece(
        Coordinate::new(2, 1),
        Coordinate::new(1, -1),
    );
    board.move_piece(
        Coordinate::new(8, 8),
        Coordinate::new(9, 8),
    );
    board.move_piece(
        Coordinate::new(3, 1),
        Coordinate::new(2, 0),
    );
    board.move_piece(
        Coordinate::new(7, 8),
        Coordinate::new(6, 10),
    );
    board.move_piece(
        Coordinate::new(4, 1),
        Coordinate::new(3, 0),
    );
    board.move_piece(
        Coordinate::new(3, 8),
        Coordinate::new(5, 10),
    );
    board.move_piece(
        Coordinate::new(5, 1),
        Coordinate::new(3, 1),
    );
    let moves = MoveGen::generate_moves(&board);
    for mv in moves {
        println!("{:?}", mv);
    }
    board.show(true);
}