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
    live_pieces: HashMap<Square, Piece>,
    fifty_move_rule: u32, // half-moves, reset upon pawn move or capture
    white_bitmap: u64,
    black_bitmap: u64,

    // tells whoose turn it is
    pub turn: Color,
    
    // used for en passant and for highlighting the squares that was just affected
    // both are set to -1, -1 initially
    pub last_moved_from: Square,
    pub last_moved_to: Square,
    
    // true if the last move was a capture
    pub capture: bool,

}

impl Game {
    pub fn new() -> Self {
        let mut live_pieces = HashMap::new();

        let white_template = Piece { piece_type: PieceType::Pawn, color: Color::White, pos: Square {x: -1, y: -1}, has_moved: false};
        // adding white pieces
        live_pieces.insert(Square {x: 0, y: 0}, Piece { piece_type: PieceType::Rook,    pos: Square {x: 0, y: 0}, ..white_template });
        live_pieces.insert(Square {x: 1, y: 0}, Piece { piece_type: PieceType::Knight,  pos: Square {x: 1, y: 0}, ..white_template });
        live_pieces.insert(Square {x: 2, y: 0}, Piece { piece_type: PieceType::Bishop,  pos: Square {x: 2, y: 0}, ..white_template });
        live_pieces.insert(Square {x: 3, y: 0}, Piece { piece_type: PieceType::Queen,   pos: Square {x: 3, y: 0}, ..white_template });
        live_pieces.insert(Square {x: 4, y: 0}, Piece { piece_type: PieceType::King,    pos: Square {x: 4, y: 0}, ..white_template });
        live_pieces.insert(Square {x: 5, y: 0}, Piece { piece_type: PieceType::Bishop,  pos: Square {x: 5, y: 0}, ..white_template });
        live_pieces.insert(Square {x: 6, y: 0}, Piece { piece_type: PieceType::Knight,  pos: Square {x: 6, y: 0}, ..white_template });
        live_pieces.insert(Square {x: 7, y: 0}, Piece { piece_type: PieceType::Rook,    pos: Square {x: 7, y: 0}, ..white_template });
        for i in 0..8 {
            live_pieces.insert(Square {x: i, y: 1}, Piece { pos: Square {x: i, y: 1}, ..white_template });
        }
        
        let black_template = Piece { color: Color::Black, ..white_template };
        // adding black pieces
        live_pieces.insert(Square {x: 0, y: 7}, Piece { piece_type: PieceType::Rook,    pos: Square {x: 0, y: 7}, ..black_template });
        live_pieces.insert(Square {x: 1, y: 7}, Piece { piece_type: PieceType::Knight,  pos: Square {x: 1, y: 7}, ..black_template });
        live_pieces.insert(Square {x: 2, y: 7}, Piece { piece_type: PieceType::Bishop,  pos: Square {x: 2, y: 7}, ..black_template });
        live_pieces.insert(Square {x: 3, y: 7}, Piece { piece_type: PieceType::Queen,   pos: Square {x: 3, y: 7}, ..black_template });
        live_pieces.insert(Square {x: 4, y: 7}, Piece { piece_type: PieceType::King,    pos: Square {x: 4, y: 7}, ..black_template });
        live_pieces.insert(Square {x: 5, y: 7}, Piece { piece_type: PieceType::Bishop,  pos: Square {x: 5, y: 7}, ..black_template });
        live_pieces.insert(Square {x: 6, y: 7}, Piece { piece_type: PieceType::Knight,  pos: Square {x: 6, y: 7}, ..black_template });
        live_pieces.insert(Square {x: 7, y: 7}, Piece { piece_type: PieceType::Rook,    pos: Square {x: 7, y: 7}, ..black_template });
        for i in 0..8 {
            live_pieces.insert(Square {x: i, y: 6}, Piece { pos: Square {x: i, y: 6}, ..black_template });
        }
        let turn = Color::White;
        let fifty_move_rule = 0;

        let white_bitmap = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
        let black_bitmap = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        let last_moved_from = Square {x: -1, y: -1};
        let last_moved_to = Square {x: -1, y: -1};
        let capture = false;
        Self {live_pieces, turn, fifty_move_rule, white_bitmap, black_bitmap, last_moved_from, last_moved_to, capture}
    }

    pub fn get_board_state(&self) -> HashMap<Square, Piece> {
        self.live_pieces.clone()
    }

    // moves the piece and takes whatever is in the way, does not do any checks
    fn force_move(&mut self, piece: &mut Piece, to: Square) -> Result<(), String> {
        if to.x > 7 || to.y > 7 {
            Err("Position out of bounds!".to_string())
        } else {
            let pos_bitmap = to.to_bitmap();

            // increment fifty_move_rule every move. Incremented before it might be reset.
            self.fifty_move_rule += 1;

            if (self.black_bitmap | self.white_bitmap) & pos_bitmap != 0 { // if there is other piece in pos
                self.live_pieces.remove(&to); // we capture it (we dont care what piece it was in this function)
                self.fifty_move_rule = 0;
            }

            // reset fifty_move_rule if a pawn was moved
            if (piece.piece_type == PieceType::Pawn) {
                self.fifty_move_rule = 0;
            }

            match piece.color {
                Color::White =>  {
                    self.white_bitmap &= !piece.pos.to_bitmap(); // turn off bit we moved from
                    self.white_bitmap |= pos_bitmap; // turn on bit we moved to
                },
                Color::Black => {
                    self.black_bitmap &= !piece.pos.to_bitmap(); // turn off bit we moved from
                    self.black_bitmap |= pos_bitmap; // turn on bit we moved to
                }
            }
            // we remove the piece to change its key
            self.live_pieces.remove(&piece.pos);

            // we set the last_moved data
            self.last_moved_from = piece.pos;
            self.last_moved_to = to;

            // we change som data of piece
            piece.pos = to;
            piece.has_moved = true;


            // we insert the piece so that the key was changed
            // cloning piece should be fine as it only contains primitive types
            self.live_pieces.insert(to, piece.clone());
            
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
        let mut moves: u64 = piece.pos.to_bitmap();
        moves |= piece.pos.left(1).to_bitmap() | piece.pos.right(1).to_bitmap();
        moves |= moves << 8 | moves >> 8;
        match piece.color {
            // this will also remove all squares that are occupied by the same colored pieces
            Color::White => moves & !&self.white_bitmap,
            Color::Black => moves & !&self.black_bitmap,
        }
    }
    
    fn psuedo_legal_moves_queen(&self, piece : Piece) -> u64 {
        let own_color_bitmap = match piece.color {
            // this will also remove all squares that are occupied by the same colored pieces
            Color::White => &self.white_bitmap,
            Color::Black => &self.black_bitmap,
        };

        let mut moves: u64 = piece.pos.to_bitmap();
        let mut square = piece.pos;
        loop  {
            square = square.left(1);
            let new_moves: u64 = moves | (square.to_bitmap() & !own_color_bitmap);
            if new_moves == moves {
                break
            } else {
                moves = new_moves;
            }
        };
        moves & !own_color_bitmap
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
    pub pos: Square,
    pub has_moved: bool, // used for castling
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    pub x: i8, // A-H (represented in code as 0-7)
    pub y: i8, // 1-8 (represented in code as 0-7)
}

impl Square {
    // initialize from a tuple of (i8, i8)
    pub fn from(pos: (i8, i8)) -> Self {
        Self {x: pos.0, y: pos.1}
    }

    // returns the square number as if they were indexed from 0,0 to 7,0 to 1,0 and so on until 7,7
    fn to_index(&self) -> i8 {
        self.x + self.y * 8
    }

    // returns the position as a bitmap, if the position is outside the board it returns 0 (empty bitmap)
    fn to_bitmap(&self) -> u64 {
        if self.x < 0 || self.x >= 8 || self.y < 0 || self.y >= 8 {
            return 0;
        }

        (1 << self.y*8) << self.x
    }

    fn to_tuple(&self) -> (i8, i8) {
        (self.x, self.y)
    }

    // returns a new square positioned i squares to the left, can be outside of board!
    fn left(&self, i: i8) -> Self {
        Self { x: self.x - i, y: self.y }
    }
    
    // returns a new square positioned i squares to the right, can be outside of board!
    fn right(&self, i: i8) -> Self {
        Self { x: self.x + i, y: self.y }
    }

    // returns a new square positioned i squares below, can be outside of board!
    fn down(&self, i: i8) -> Self {
        Self { x: self.x, y: self.y - i }
    }

    // returns a new square positioned i squares above, can be outside of board!
    fn up(&self, i: i8) -> Self {
        Self { x: self.x, y: self.y + i }
    }
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

fn bitmap_line(start: Square, dx: i8, dy: i8)
{
    
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

        bitmap |= piece.pos.to_bitmap();

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
