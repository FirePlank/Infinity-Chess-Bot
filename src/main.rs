mod board;
mod movegen;

use num_bigint::BigInt;
use board::{Board, Coordinate, Piece};
use movegen::{MoveGen, Move};

fn main() {
    let mut board = Board::new();
    board.move_piece(
        Coordinate::new(1, 2),
        Coordinate::new(1, 4),
    );
    board.move_piece(
        Coordinate::new(8, 7),
        Coordinate::new(8, 5),
    );
    board.move_piece(
        Coordinate::new(1, 4),
        Coordinate::new(1, 5),
    );
    board.move_piece(
        Coordinate::new(2, 7),
        Coordinate::new(2, 5),
    );
    board.move_piece(
        Coordinate::new(1, 5),
        Coordinate::new(2, 6),
    );
    let moves = MoveGen::generate_moves(&board);
    for mv in moves {
        println!("{:?}", mv);
    }
    board.show(true);
}