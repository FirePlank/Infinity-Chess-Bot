mod board;
mod movegen;

use num_bigint::BigInt;
use board::{Board, Coordinate, Piece};
use movegen::{MoveGen, Move};

fn main() {
    let mut board = Board::new();
    board.show(true);
    board.move_piece(
        Coordinate::new(1, 1),
        Coordinate(BigInt::parse_bytes(b"-4", 10).unwrap(), BigInt::from(1))
    );
    let moves = MoveGen::generate_moves(&board);
    for mv in moves {
        println!("{:?}", mv);
    }
}