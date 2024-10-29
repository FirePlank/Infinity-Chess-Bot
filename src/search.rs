use crate::board::{Board, Coordinate, Piece, PIECE_VALUES};
use crate::r#move::movegen::{Move, MoveGen};
use std::mem::MaybeUninit;
use std::time::{SystemTime, UNIX_EPOCH};
use num_bigint::BigInt;
use num_traits::Signed;
use crate::r#move::MoveList;
use array_init::array_init;

pub const MAX_PLY: usize = 127;
pub const INFINITY: i32 = 1000000;
pub const MATE_VALUE: i32 = INFINITY - 150;
pub const MATE_SCORE: i32 = INFINITY - 300;
pub const TIME_UP: i32 = INFINITY + 500;

pub static mut STOP: bool = false;

#[derive(Clone)]
pub struct Searcher {
    pub ply: u8,
    pub nodes: u64,
    pub time: u128,
    pub killers: Vec<Vec<Move>>,
    pub pv_table: Vec<Vec<Move>>,
    pub pv_length: [u8; MAX_PLY],
    pub follow_pv: bool,
    pub score_pv: bool,
    pub full_depth_moves: u8,
    pub reduction_limit: u8,
    pub inc: i32,
    pub movetime: i32,
    pub movestogo: i32,
    pub playtime: i32,
    pub timeset: bool,
    pub stoptime: u128,
}

impl Searcher {
    pub fn new() -> Searcher {
        let default_move = Move::Normal(Coordinate::new(0, 0), Coordinate::new(0, 0));

        // Initialize the arrays using Vec
        let killers = vec![vec![default_move.clone(); MAX_PLY]; 2];
        let pv_table = vec![vec![default_move.clone(); MAX_PLY]; MAX_PLY];

        Searcher {
            ply: 0,
            nodes: 0,
            time: 0,
            killers,
            pv_table,
            pv_length: [0; MAX_PLY],
            follow_pv: false,
            score_pv: false,
            full_depth_moves: 3,
            reduction_limit: 2,
            inc: 0,
            movetime: -1,
            movestogo: 30,
            playtime: -1,
            timeset: false,
            stoptime: 0,
        }
    }

    pub fn stop_search(&mut self) -> bool {
        if unsafe { STOP } || (self.timeset && SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() > self.stoptime) {
            return true;
        }
        false
    }

    pub fn search_position(&mut self, board: &mut Board, depth: u8) -> Move {
        unsafe { STOP = false; }

        let mut best_move = Move::Normal(Coordinate::new(0, 0), Coordinate::new(0, 0));

        for current_depth in 1..=depth {
            if self.stop_search() {
                break;
            }
            self.follow_pv = true;

            let mut score = -INFINITY;

            score = self.negamax(board, -INFINITY, INFINITY, current_depth);

            if self.stop_search() {
                break;
            }

            if score > -MATE_VALUE && score < -MATE_SCORE {
                print!("info score mate {} depth {} nodes {} time {} pv ", -(self.pv_length[0] as i16) / 2 - 1, current_depth, self.nodes, SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - self.time);
            } else if score > MATE_SCORE && score < MATE_VALUE {
                print!("info score mate {} depth {} nodes {} time {} pv ", self.pv_length[0] / 2 + 1, current_depth, self.nodes, SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - self.time);
            } else {
                print!("info score cp {} depth {} nodes {} time {} pv ", score, current_depth, self.nodes, SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - self.time);
            }
            for count in 0..self.pv_length[0] {
                println!("{:?}", self.pv_table[0][count as usize]);
            }
            println!();

            best_move = self.pv_table[0][0].clone();
        }

        best_move
    }

    pub fn quiescence(&mut self, board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;

        let eval = board.evaluate();

        if eval >= beta {
            return beta;
        } else if eval > alpha {
            alpha = eval;
        }

        if self.ply >= MAX_PLY as u8 {
            return eval;
        }

        if self.stop_search() {
            return TIME_UP;
        }

        let mut move_list = MoveList::new();
        MoveGen::generate_moves(&board, &mut move_list);
        let mut move_scores: [u32; 256] = unsafe { MaybeUninit::uninit().assume_init() };

        let counted = move_list.count;
        self.assign_move_scores(board, &move_list.moves, &mut move_scores, counted as usize);

        for count in 0..counted {
            let mv = self.sort_next_move(&mut move_list.moves, &mut move_scores, count as usize, counted as usize);

            if move_scores[count as usize] as i32 - 8000 < 0 {
                break;
            }

            if !board.make(mv.clone()) {
                board.unmake(mv);
                continue;
            }

            self.ply += 1;
            let score = -self.quiescence(board, -beta, -alpha);
            board.unmake(mv);
            self.ply -= 1;

            if self.stop_search() {
                return TIME_UP;
            }

            if score > alpha {
                alpha = score;
                if score >= beta {
                    return beta;
                }
            }
        }
        return alpha;
    }

    pub fn negamax(&mut self, board: &mut Board, mut alpha: i32, mut beta: i32, mut depth: u8) -> i32 {
        let pv_node = beta.wrapping_sub(alpha) > 1;
        let mut best_move = Move::Normal(Coordinate::new(0, 0), Coordinate::new(0, 0));

        let mut score;
        let is_root = self.ply == 0;

        self.nodes += 1;

        if self.ply >= MAX_PLY as u8 {
            return board.evaluate();
        }
        //
        // if board.is_fifty() {
        //     return 0;
        // }

        self.pv_length[self.ply as usize] = self.ply;

        if !is_root {
            // if board.is_threefold() {
            //     return 0;
            // }

            if alpha < -MATE_VALUE {
                alpha = -MATE_VALUE;
            }
            if beta > MATE_VALUE - 1 {
                beta = MATE_VALUE - 1;
            }
            if alpha >= beta {
                return alpha;
            }
        }

        if depth == 0 {
            return self.quiescence(board, alpha, beta);
        }

        let in_check = board.is_attacked(board.king_position(board.side_to_move), !board.side_to_move);

        if in_check {
            depth += 1;
        }

        if self.stop_search() {
            return 0;
        }

        let eval = board.evaluate();
        if !in_check && !pv_node {
            if depth < 3 && (beta - 1).abs() > -49000 + 100 {
                let eval_margin = 100 * depth as i32;
                if eval - eval_margin >= beta.into() {
                    return eval - eval_margin;
                }
            }
        }

        if self.stop_search() {
            return TIME_UP;
        }

        let mut legal_moves = 0;
        let mut move_list = MoveList::new();
        MoveGen::generate_moves(&board, &mut move_list);
        let mut move_scores: [u32; 256] = unsafe { MaybeUninit::uninit().assume_init() };

        let counted = move_list.count;
        self.assign_move_scores(board, &move_list.moves, &mut move_scores, counted as usize);

        let mut moves_searched = 0;

        let mut best_score = -INFINITY;
        let mut skip_quiet = false;

        for count in 0..counted {
            let mv = self.sort_next_move(&mut move_list.moves, &mut move_scores, count as usize, counted as usize);

            let is_quiet = match mv {
                Move::Normal(_, ref to) | Move::Promotion(_, ref to, _) => board.get_piece(&to).is_none(),
                _ => false,
            };

            if is_quiet && skip_quiet {
                continue;
            }

            let is_killer = self.killers[0][self.ply as usize] == mv.clone() || self.killers[1][self.ply as usize] == mv.clone();

            if !is_root && best_score > -INFINITY {
                if depth < 8 && is_quiet && !is_killer && eval <= alpha && alpha.abs() < INFINITY - 100 {
                    skip_quiet = true;
                    continue;
                }
            }

            if !board.make(mv.clone()) {
                board.unmake(mv);
                continue;
            }

            self.ply += 1;
            legal_moves += 1;

            if moves_searched == 0 {
                score = -self.negamax(board, -beta, -alpha, depth - 1);
            } else {
                if moves_searched >= self.full_depth_moves && depth >= self.reduction_limit && !in_check {
                    score = -self.negamax(board, -alpha - 1, -alpha, depth - 2);
                } else {
                    score = alpha + 1;
                }
                if score > alpha {
                    score = -self.negamax(board, -alpha - 1, -alpha, depth - 1);
                    if score > alpha && score < beta {
                        score = -self.negamax(board, -beta, -alpha, depth - 1);
                    }
                }
            }

            board.unmake(mv.clone());
            self.ply -= 1;

            if self.stop_search() {
                return TIME_UP;
            }

            moves_searched += 1;

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                best_move = mv.clone();
                best_score = score;
                alpha = score;

                self.pv_table[self.ply as usize][self.ply as usize] = mv.clone();
                for next_ply in self.ply + 1..self.pv_length[self.ply as usize + 1] {
                    self.pv_table[self.ply as usize][next_ply as usize] = self.pv_table[self.ply as usize + 1][next_ply as usize].clone();
                }
                self.pv_length[self.ply as usize] = self.pv_length[self.ply as usize + 1];

                if score >= beta {
                    if is_quiet {
                        self.killers[1][self.ply as usize] = self.killers[0][self.ply as usize].clone();
                        self.killers[0][self.ply as usize] = mv;
                    }
                    return beta;
                }
            }
        }

        if legal_moves == 0 {
            return if in_check {
                -MATE_VALUE + self.ply as i32
            } else {
                0
            }
        }

        alpha
    }

    fn assign_move_scores(&mut self, board: &Board, moves: &[Move; 256], move_scores: &mut [u32; 256], moves_count: usize) {
        for move_index in 0..moves_count {
            move_scores[move_index] = self.score_move(board, &moves[move_index]);
        }
    }

    fn score_move(&mut self, board: &Board, mv: &Move) -> u32 {
        // if move scoring is allowed
        if self.score_pv {
            // make sure we are dealing with PV move
            if self.pv_table[0][self.ply as usize] == *mv {
                // disable score PV flag
                self.score_pv = false;
                // give PV move the highest score, so we search it first
                return 16000;
            }
        }

        let mut score: u32 = 0;

        match mv {
            Move::Normal(from, to) => {
                if let Some(captured) = board.get_piece(to) {
                    // prioritize captures
                    score += 8000;
                    // score move by piece value
                    let piece_value = PIECE_VALUES[*captured as usize] - PIECE_VALUES[*board.get_piece(from).unwrap() as usize];
                    if piece_value > 0 {
                        score += piece_value as u32;
                    } else {
                        score -= piece_value.abs() as u32;
                    }
                } else {
                    // score quiet move
                    if self.killers[0][self.ply as usize] == mv.clone() {
                        // score 1st killer move
                        score += 4000;
                    } else if self.killers[1][self.ply as usize] == mv.clone() {
                        // score 2nd killer move
                        score += 2500;
                    }

                    // reward for castling
                    if let Some(piece) = board.get_piece(from) {
                        if (*piece == Piece::WhiteKing || *piece == Piece::BlackKing) && (from.0.clone() - to.0.clone()).abs() == BigInt::from(2) {
                            score += 400;
                        }
                    }
                }
            }
            Move::Promotion(_, _, promoted) => {
                // promotions always first
                score += 9500 + PIECE_VALUES[*promoted as usize] as u32;
            }
            _ => {}
        }

        score
    }

    fn sort_next_move(&self, moves: &mut [Move; 256], move_scores: &mut [u32; 256], start_index: usize, moves_count: usize) -> Move {
        let mut best_score = move_scores[start_index];
        let mut best_index = start_index;

        for index in (start_index + 1)..moves_count {
            if move_scores[index] > best_score {
                best_score = move_scores[index];
                best_index = index;
            }
        }

        if best_index != start_index {
            moves.swap(start_index, best_index);
            move_scores.swap(start_index, best_index);
        }

        moves[start_index].clone()
    }
}