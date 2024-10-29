// use num_bigint::BigInt;
// use num_traits::ToPrimitive;
//
// pub fn encode_move(
//     source_x: BigInt,
//     source_y: BigInt,
//     target_x: BigInt,
//     target_y: BigInt,
//     piece: u8,
//     promoted: u8,
//     capture: u8,
//     double: u8,
//     enpassant: u8,
//     castling: u8,
//     infinite: u8,
// ) -> BigInt {
//     let mut result = BigInt::from(0);
//
//     result |= source_x & BigInt::from(0xFFFF);
//     result |= (source_y & BigInt::from(0xFFFF)) << 16;
//     result |= (target_x & BigInt::from(0xFFFF)) << 32;
//     result |= (target_y & BigInt::from(0xFFFF)) << 48;
//     result |= BigInt::from(piece) << 56;
//     result |= BigInt::from(promoted) << 60;
//     result |= BigInt::from(capture) << 61;
//     result |= BigInt::from(double) << 62;
//     result |= BigInt::from(enpassant) << 63;
//     result |= BigInt::from(castling) << 64;
//     result |= BigInt::from(infinite) << 65;
//
//     result
// }
//
// // Extract source x coordinate
// pub fn source_x(move_: BigInt) -> BigInt {
//     return move_ & BigInt::from(0xFFFF);
// }
//
// // Extract source y coordinate
// pub fn source_y(move_: BigInt) -> BigInt {
//     return (move_ >> 16) & BigInt::from(0xFFFF);
// }
//
// // Extract target x coordinate
// pub fn target_x(move_: BigInt) -> BigInt {
//     return (move_ >> 32) & BigInt::from(0xFFFF);
// }
//
// // Extract target y coordinate
// pub fn target_y(move_: BigInt) -> BigInt {
//     return (move_ >> 48) & BigInt::from(0xFFFF);
// }
//
// // Extract piece
// pub fn get_piece(move_: BigInt) -> u8 {
//     return ((move_ >> 56) & BigInt::from(0xF)).to_u64().unwrap() as u8;}
//
// // Extract promoted piece
// pub fn promoted(move_: BigInt) -> u8 {
//     return ((move_ >> 60) & 0x1).to_u8().unwrap();
// }
//
// // Extract capture flag
// pub fn capture(move_: BigInt) -> u8 {
//     return ((move_ >> 61) & 0x1).to_u8().unwrap();
// }
//
// // Extract double pawn push flag
// pub fn double(move_: BigInt) -> u8 {
//     return ((move_ >> 62) & 0x1).to_u8().unwrap();
// }
//
// // Extract enpassant flag
// pub fn enpassant(move_: BigInt) -> u8 {
//     return ((move_ >> 63) & 0x1).to_u8().unwrap();
// }
//
// // Extract castling flag
// pub fn castling(move_: BigInt) -> u8 {
//     return ((move_ >> 64) & 0x1).to_u8().unwrap();
// }
//
// // Extract infinite flag
// pub fn infinite(move_: BigInt) -> u8 {
//     return ((move_ >> 65) & 0x1).to_u8().unwrap();
// }