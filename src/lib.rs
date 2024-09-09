// Bitmaps are used to represent a set of selected squares.
// A single u64 can represent the whole board, bit[0] = A1, bit[7] = H1, bit[63] = H8.
// To represent all positions where a piece can move, a bitmap is used
// To represent squares that would place the king in check, all bitmaps of the opponents legal
//  moves are added together using the | operator.
//
// There are legal moves and psuedo-legal moves:
//  Legal moves are the moves that a piece is allowed to make
//  Psuedo-legal moves are moves that a piece can make without checking if the king is placed in check.

//use std::vec;

use std::collections::HashMap;

// DATA
pub struct Game {
    live_pieces: HashMap<u8, Piece>,
    turn: Color,
    fifty_move_rule: u32, // half-moves, reset upon pawn move or capture
    white_bitmap: u64,
    black_bitmap: u64,
}

impl Game {
    pub fn new(&self) -> Self {
        let mut live_pieces = HashMap::new();

        let white_template = Piece { piece_type: PieceType::Pawn, color: Color::White, x: 8, y: 0, has_moved: false, just_moved: false};
        // adding white pieces
        live_pieces.insert(0, Piece { piece_type: PieceType::Rook,   x: 0, ..white_template });
        live_pieces.insert(1, Piece { piece_type: PieceType::Knight, x: 1, ..white_template });
        live_pieces.insert(2, Piece { piece_type: PieceType::Bishop, x: 2, ..white_template });
        live_pieces.insert(3, Piece { piece_type: PieceType::Queen,  x: 3, ..white_template });
        live_pieces.insert(4, Piece { piece_type: PieceType::King,   x: 4, ..white_template });
        live_pieces.insert(5, Piece { piece_type: PieceType::Bishop, x: 5, ..white_template });
        live_pieces.insert(6, Piece { piece_type: PieceType::Knight, x: 6, ..white_template });
        live_pieces.insert(7, Piece { piece_type: PieceType::Rook,   x: 7, ..white_template });
        for i in 0..8 {
            live_pieces.insert(8 + i, Piece { x: i, y: 1, ..white_template });
        }
        
        let black_template = Piece { color: Color::Black, y: 7, ..white_template };
        // adding black pieces
        live_pieces.insert(56, Piece { piece_type: PieceType::Rook,   x: 0, ..black_template });
        live_pieces.insert(57, Piece { piece_type: PieceType::Knight, x: 1, ..black_template });
        live_pieces.insert(58, Piece { piece_type: PieceType::Bishop, x: 2, ..black_template });
        live_pieces.insert(59, Piece { piece_type: PieceType::Queen,  x: 3, ..black_template });
        live_pieces.insert(60, Piece { piece_type: PieceType::King,   x: 4, ..black_template });
        live_pieces.insert(61, Piece { piece_type: PieceType::Bishop, x: 5, ..black_template });
        live_pieces.insert(62, Piece { piece_type: PieceType::Knight, x: 6, ..black_template });
        live_pieces.insert(63, Piece { piece_type: PieceType::Rook,   x: 7, ..black_template });
        for i in 0..8 {
            live_pieces.insert(48 + i, Piece { x: i, y: 6, ..black_template });
        }
        let turn = Color::White;
        let fifty_move_rule = 0;

        let white_bitmap = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
        let black_bitmap = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;

        Self {live_pieces, turn, fifty_move_rule, white_bitmap, black_bitmap}
    }

    pub fn get_board_state(&self) -> HashMap<u8, Piece> {
        self.live_pieces.clone()
    }

    // moves the piece and takes whatever is in the way, does not do any checks
    fn force_move(&mut self, piece: &mut Piece, x: u8, y: u8) -> Result<(), String> {
        if x > 7 || y > 7 {
            Err("Position out of bounds!".to_string())
        } else {
            let pos_bitmap = pos_to_bitmap(x, y);

            if (self.black_bitmap | self.white_bitmap) & pos_bitmap != 0 { // if there is other piece in pos
                self.live_pieces.remove(&pos_to_index(x, y));
            }

            if piece.color == Color::White {
                
                self.white_bitmap &= !pos_to_bitmap(piece.x, piece.y); // turn off bit we moved from
                self.white_bitmap |= pos_bitmap; // turn on bit we moved to
            } else {
                self.black_bitmap &= !pos_to_bitmap(piece.x, piece.y); // turn off bit we moved from
                self.black_bitmap |= pos_bitmap; // turn on bit we moved to
            }
            
            piece.x = x;
            piece.y = y;
            piece.has_moved = true;
            piece.just_moved = true;
            Ok(())
        }
    }

    fn psuedo_legal_moves(&self, piece : Piece) -> u64 {
        match piece.piece_type {
            PieceType::King => return self.psuedo_legal_moves_king(piece),
            PieceType::Queen => return self.psuedo_legal_moves_queen(piece),
            PieceType::Bishop => return self.psuedo_legal_moves_bishop(piece),
            PieceType::Knight => return self.psuedo_legal_moves_knight(piece),
            PieceType::Rook => return self.psuedo_legal_moves_rook(piece),
            PieceType::Pawn => return self.psuedo_legal_moves_pawn(piece),
        }
    }
    
    fn psuedo_legal_moves_king(&self, piece : Piece) -> u64 {
        let mut moves: u64 = pos_to_bitmap(piece.x, piece.y);
        todo!();
        moves
    }
    
    fn psuedo_legal_moves_queen(&self, piece : Piece) -> u64 {
        let mut moves: u64 = 0;
        todo!();
        moves
    }
    
    fn psuedo_legal_moves_bishop(&self, piece : Piece) -> u64 {
        let mut moves: u64 = 0;
        todo!();
        moves
    }
    
    fn psuedo_legal_moves_knight(&self, piece : Piece) -> u64 {
        let mut moves: u64 = 0;
        todo!();
        moves
    }
    
    fn psuedo_legal_moves_rook(&self, piece : Piece) -> u64 {
        let mut moves: u64 = 0;
        todo!();
        moves
    }
    
    fn psuedo_legal_moves_pawn(&self, piece : Piece) -> u64 {
        let mut moves: u64 = 0;
        todo!();
        moves
    }
    
}

#[derive(Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub x: u8, // A-H (represented in code as 0-7)
    pub y: u8, // 1-8 (represented in code as 0-7)
    pub has_moved: bool,
    pub just_moved: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

fn pos_to_bitmap(x: u8, y: u8) -> u64 {
    (1 << y*8) << x
}

fn pos_to_index(x: u8, y: u8) -> u8 {
    x + y * 8
}

fn print_bitmap(bitmap: u64) {
    let bits : String = format!("{:064b}", bitmap).chars().rev().collect();

    println!("8 {}\n7 {}\n6 {}\n5 {}\n4 {}\n3 {}\n2 {}\n1 {}\n  ABCDEFGH", &bits[56..64], &bits[48..56], &bits[40..48], &bits[32..40], &bits[24..32], &bits[16..24], &bits[8..16], &bits[0..8]);
    
}

fn make_color_bitmap(game: Game, color: Color) -> u64 {
    let mut bitmap = 0;
    for piece in game.live_pieces.values() {
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
