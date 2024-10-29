mod board;
mod r#move;
mod evaluation;
mod search;

use num_bigint::BigInt;
use board::{Board, Coordinate, Piece};
use r#move::movegen::{MoveGen, Move};
use crate::r#move::MoveList;
use crate::search::Searcher;

fn main() {
    // let mut board = Board::new();
    // board.make(
    //     Move::Normal(
    //         Coordinate::new(4, 1),
    //         Coordinate::new(9, -4),
    //     )
    // );
    // board.make(
    //     Move::Normal(
    //         Coordinate::new(4, 8),
    //         Coordinate::new(0, 12),
    //     )
    // );
    // board.make(
    //     Move::Normal(
    //         Coordinate::new(9, -4),
    //         Coordinate::new(9, 12),
    //     )
    // );
    let mut board = Board::empty();

    board.set_piece(Coordinate::new(4, 1), Piece::BlackKing);
    board.set_piece(Coordinate::new(4, 8), Piece::WhiteKing);
    board.set_piece(Coordinate::new(3, -5), Piece::WhiteRook);
    board.set_piece(Coordinate::new(6, -5), Piece::WhiteRook);
    board.set_piece(Coordinate::new(0, -10), Piece::WhiteRook);

    board.show(true);

    let mut move_list = MoveList::new();
    MoveGen::generate_moves(&board, &mut move_list);
    let counted = move_list.count;
    for count in 0..counted {
        let mv = move_list.moves[count as usize].clone();
        println!("{:?}", mv);
    }
    board.show(true);
    println!("Evaluation: {:?}", board.evaluate());

    let mut searcher = Searcher::new();
    let best_move = searcher.search_position(&mut board, 10);
    println!("Best move: {:?}", best_move);
}