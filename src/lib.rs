// Bitmaps are used to represent a set of selected squares.
// A single u64 can represent the whole board, bit[0] = A1, bit[7] = H1, bit[63] = H8.
// To represent all positions where a piece can move, a bitmap is used
// To represent squares that would place the king in check, all bitmaps of the opponents legal
//  moves are added together using the | operator.
//
// There are legal moves and psuedo-legal moves:
//  Legal moves are the moves that a piece is allowed to make
//  Psuedo-legal moves are moves that a piece can make without checking if the king is placed in check.

use std::vec;

// DATA



struct Game {
    live_pieces: Vec<Piece>,
    turn: Color,
    fifty_move_rule: u32, // half-moves, reset upon pawn move or capture
    white_bitmap: u64,
    black_bitmap: u64,
}

impl Game {
    fn new() -> Self {
        let mut live_pieces = Vec::new();
        // adding white pieces
        live_pieces.push(Piece { piece_type: PieceType::Rook, color: Color::White, x: 0, y: 0, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Knight, color: Color::White, x: 1, y: 0, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Bishop, color: Color::White, x: 2, y: 0, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Queen, color: Color::White, x: 3, y: 0, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::King, color: Color::White, x: 4, y: 0, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Bishop, color: Color::White, x: 5, y: 0, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Knight, color: Color::White, x: 6, y: 0, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Rook, color: Color::White, x: 7, y: 0, has_moved: false, just_moved: false });
        for i in 0..8 {
            live_pieces.push(Piece { piece_type: PieceType::Pawn, color: Color::White, x: i, y: 1, has_moved: false, just_moved: false });
        }

        // adding black pieces
        live_pieces.push(Piece { piece_type: PieceType::Rook, color: Color::Black, x: 0, y: 7, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Knight, color: Color::Black, x: 1, y: 7, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Bishop, color: Color::Black, x: 2, y: 7, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Queen, color: Color::Black, x: 3, y: 7, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::King, color: Color::Black, x: 4, y: 7, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Bishop, color: Color::Black, x: 5, y: 7, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Knight, color: Color::Black, x: 6, y: 7, has_moved: false, just_moved: false });
        live_pieces.push(Piece { piece_type: PieceType::Rook, color: Color::Black, x: 7, y: 7, has_moved: false, just_moved: false });
        for i in 0..8 {
            live_pieces.push(Piece { piece_type: PieceType::Pawn, color: Color::Black, x: i, y: 6, has_moved: false, just_moved: false });
        }
        let turn = Color::White;
        let fifty_move_rule = 0;

        let white_bitmap = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
        let black_bitmap = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;

        Self {live_pieces, turn, fifty_move_rule, white_bitmap, black_bitmap}
    }
    
}

struct Piece {
    piece_type: PieceType,
    color: Color,
    x: u8, // A-H (represented in code as 0-7)
    y: u8, // 1-8 (represented in code as 0-7)
    has_moved: bool,
    just_moved: bool,
}

enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq)]
enum Color {
    White,
    Black,
}



fn psuedo_legal_moves(piece : Piece) -> u64 {
    match piece.piece_type {
        PieceType::King => return psuedo_legal_moves_king(piece),
        PieceType::Queen => return psuedo_legal_moves_queen(piece),
        PieceType::Bishop => return psuedo_legal_moves_bishop(piece),
        PieceType::Knight => return psuedo_legal_moves_knight(piece),
        PieceType::Rook => return psuedo_legal_moves_rook(piece),
        PieceType::Pawn => return psuedo_legal_moves_pawn(piece),
    }
}

fn psuedo_legal_moves_king(piece : Piece) -> u64 {
    let mut moves: u64 = 0;
    todo!();
    moves
}

fn psuedo_legal_moves_queen(piece : Piece) -> u64 {
    let mut moves: u64 = 0;
    todo!();
    moves
}

fn psuedo_legal_moves_bishop(piece : Piece) -> u64 {
    let mut moves: u64 = 0;
    todo!();
    moves
}

fn psuedo_legal_moves_knight(piece : Piece) -> u64 {
    let mut moves: u64 = 0;
    todo!();
    moves
}

fn psuedo_legal_moves_rook(piece : Piece) -> u64 {
    let mut moves: u64 = 0;
    todo!();
    moves
}

fn psuedo_legal_moves_pawn(piece : Piece) -> u64 {
    let mut moves: u64 = 0;
    todo!();
    moves
}

fn pos_to_bitmap(x: u8, y: u8) -> u64 {
    (1 << y*8) << x
}

fn print_bitmap(bitmap: u64) {
    let bits : String = format!("{:064b}", bitmap).chars().rev().collect();

    println!("8 {}\n7 {}\n6 {}\n5 {}\n4 {}\n3 {}\n2 {}\n1 {}\n  ABCDEFGH", &bits[56..64], &bits[48..56], &bits[40..48], &bits[32..40], &bits[24..32], &bits[16..24], &bits[8..16], &bits[0..8]);
    
}

fn make_color_bitmap(game: Game, color: Color) -> u64 {
    let mut bitmap = 0;
    for piece in game.live_pieces {
        if piece.color != color {
            continue;
        }

        bitmap |= pos_to_bitmap(piece.x, piece.y);

    }
    bitmap
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_to_bitmap() {
        let x: u8 = 4;
        let y: u8 = 4;

        assert_eq!(pos_to_bitmap(x, y), 0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000);
    }

    #[test]
    fn test_color() {
        let col1 = Color::White;
        let col2 = Color::White;
        let col3 = Color::Black;
        let col4 = Color::Black;
        assert!(col1 == col2);
        assert!(col2 == col1);
        assert!(col3 == col4);
        assert!(col1 != col3);
        assert!(col4 != col2);
        assert!(!(col1 != col2));
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
