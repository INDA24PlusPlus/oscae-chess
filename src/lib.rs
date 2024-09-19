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

use std::{collections::HashMap, ops::Not};

// DATA
pub struct Game {
    live_pieces: HashMap<Square, Piece>,
    fifty_move_rule: u32, // half-moves, reset upon pawn move or capture
    previous_states: HashMap<BoardValue, u8>, // used for draw by repetition rule, stores count in value
    white_bitmap: u64,
    black_bitmap: u64,

    // tells whoose turn it is
    pub turn: PieceColor,
    // tells the result
    pub result: ChessResult,
    
    // used for en passant and for highlighting the squares that was just affected
    // both are set to -1, -1 initially
    pub last_moved_from: Square,
    pub last_moved_to: Square,
    
    // true if the last move was a capture
    pub capture: bool,

    // true if the current player is in check
    pub check: bool,

    // true if the move lead to promotion and pawn_promotion() has to be called
    pub promotion: bool,

    // lists of captured pieces
    // white pieces that was captured
    pub white_captured_pieces : Vec<PieceType>,
    // black pieces that was captured
    pub black_captured_pieces : Vec<PieceType>,

    // starts at 1 and is incremented when black moves
    pub fullmoves: u32,
}

impl Game {
    // creates a new chess instance with the initial starting positions of chess
    pub fn new() -> Self {
        let mut live_pieces = HashMap::new();

        let white_template = Piece { piece_type: PieceType::Pawn, color: PieceColor::White, pos: Square {x: -1, y: -1}, has_moved: false};
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
        
        let black_template = Piece { color: PieceColor::Black, ..white_template };
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
        let turn = PieceColor::White;
        let result = ChessResult::Ongoing;
        let fifty_move_rule = 0;
        let previous_states = HashMap::new();

        let white_bitmap = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
        let black_bitmap = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        let last_moved_from = Square {x: -1, y: -1};
        let last_moved_to = Square {x: -1, y: -1};
        let capture = false;
        let check = false;
        let promotion = false;
        let white_captured_pieces = Vec::new();
        let black_captured_pieces = Vec::new();
        let fullmoves = 1;
        let mut game = Self {live_pieces, turn, result, fifty_move_rule, previous_states, white_bitmap, black_bitmap, last_moved_from, last_moved_to, capture, check, promotion, white_captured_pieces, black_captured_pieces, fullmoves};
        game.previous_states.insert(BoardValue::from(&game), 1);

        game
    }

    // creates a chess instance from FEN
    pub fn from_fen(fen: &String) -> Self {
        let mut game = Game::new();

        let fields: Vec<&str> = fen.trim().split_whitespace().collect();
        
        // 1 pieces
        if fields.len() < 1 { return game; }
        
        game.live_pieces.clear();

        let template = Piece { piece_type: PieceType::Pawn, color: PieceColor::White, pos: Square {x: -1, y: -1}, has_moved: true};

        let mut x = 0;
        let mut y = 7;

        for c in fields[0].chars() {
            let mut piece_type = PieceType::Pawn;
            let color = if c.to_ascii_lowercase() == c {
                PieceColor::Black
            } else {
                PieceColor::White
            };

            let mut has_moved = true;

            match c.to_ascii_uppercase() {
                '/' => { y -= 1; x = 0; continue; },
                'K' => { piece_type = PieceType::King; has_moved = false },
                'Q' => piece_type = PieceType::Queen,
                'B' => piece_type = PieceType::Bishop,
                'N' => piece_type = PieceType::Knight,
                'R' => piece_type = PieceType::Rook,
                'P' => piece_type = {
                    if match color {
                        PieceColor::White => 1,
                        PieceColor::Black => 6,
                    } == y {
                       has_moved = false; 
                    }
                    PieceType::Pawn
                },
                _ => {
                    match c.to_string().parse::<i8>() {
                        Ok(i) => {
                            x += i;
                            continue;
                        },
                        Err(_) => (),
                    }
                },
            }
            game.live_pieces.insert(Square {x, y}, Piece { piece_type, color, pos: Square {x, y}, has_moved, ..template });
            
            x += 1;
        }

        // 2 turn
        if fields.len() < 2 { return game; }

        game.turn = match fields[1] { // post_move() will change turn later
            "b" => PieceColor::White,
            _ => PieceColor::Black,
        };

        // 3 castling rights
        if fields.len() < 3 { return game; }

        if fields[2].contains("Q") {
            match game.live_pieces.get_mut(&Square::from((0, 0))) {
                Some(rook) => rook.has_moved = false,
                None => (),
            }
        }

        if fields[2].contains("K") {
            match game.live_pieces.get_mut(&Square::from((7, 0))) {
                Some(rook) => rook.has_moved = false,
                None => (),
            }
        }

        if fields[2].contains("q") {
            match game.live_pieces.get_mut(&Square::from((0, 7))) {
                Some(rook) => rook.has_moved = false,
                None => (),
            }
        }

        if fields[2].contains("k") {
            match game.live_pieces.get_mut(&Square::from((7, 7))) {
                Some(rook) => rook.has_moved = false,
                None => (),
            }
        }

        // 4 possible en passant target
        if fields.len() < 4 { return game; }

        if fields[3] != "-"
        {
            game.last_moved_to = Square::from(fields[3]);
            game.last_moved_from = game.last_moved_to.moved(0, match game.turn {
                PieceColor::White => 2,
                PieceColor::Black => -2,
            });
        }

        // 5 halfmove clock
        if fields.len() < 5 { return game; }

        match fields[4].parse() {
            Ok(i) => game.fifty_move_rule = i,
            Err(_) => (),
        }

        // 6 fullmove number
        if fields.len() < 6 { return game; }

        match fields[5].parse::<u32>() {
            Ok(i) => game.fullmoves = i,
            Err(_) => (),
        }

        if game.turn == PieceColor::Black && game.fullmoves > 0 {
            game.fullmoves -= 1;
        }

        // bitmaps
        game.white_bitmap = _make_color_bitmap(&game.live_pieces, PieceColor::White);
        game.black_bitmap = _make_color_bitmap(&game.live_pieces, PieceColor::Black);
        
        game.post_move();

        game
    }

    // returns a reference to the hashmap of live pieces
    pub fn get_board_state(&self) -> &HashMap<Square, Piece> {
        &self.live_pieces
    }

    // returns a vec of Square, of all legal moves that can be made from the square "from" considering turn
    pub fn get_moves_list(&self, from : &Square) -> Vec<Square> {
        match self.live_pieces.get(from) {
            Some(piece) => {
                let mut moves = Vec::new();
                if piece.color != self.turn {
                    return moves;
                }

                let moves_bitmap = self.legal_moves(piece);
                for i in 0..64 {
                    if (moves_bitmap >> i) & 1 != 0 {
                        moves.push(Square::from(i));
                    }
                }
                moves
            },
            None => return Vec::new(),
        }
    }

    // returns a bitmap of all legal moves that can be made from the square "from" considering turn
    pub fn get_moves_bitmap(&self, from: &Square) -> u64 {
        match self.live_pieces.get(from) {
            Some(piece) => {
                if piece.color == self.turn {
                    self.legal_moves(piece)
                } else {
                    0
                }
            },
            None => 0,
        }
    }

    // does a move and returns true if it was successful
    pub fn do_move(&mut self, from: &Square, to: &Square) -> bool {

        // get piece at "from"
        let mut piece = match self.live_pieces.get(from) {
            Some(p) => p.clone(),
            None => return false,
        };

        // return false if it is not this piece's turn
        if piece.color != self.turn {
            return false;
        }

        // return false if a promotion has to be done using pawn_promotion()
        if self.promotion {
            return false;
        }

        if self.legal_moves(&piece) & to.to_bitmap() != 0 {
            // legal move
            match self.force_move(&mut piece, *to) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            // illegal move
            false
        }
    }

    // selects the piece to promote a pawn to. will return false if invalid PieceType whas passed
    pub fn pawn_promotion(&mut self, class: PieceType) -> bool {
        // return false if class is king or pawn
        if class == PieceType::King || class == PieceType::Pawn || !self.promotion {
            return false;
        }

        // do promotion and finish move with post_move()
        match self.live_pieces.get_mut(&self.last_moved_to) {
            Some(piece) => {
                piece.piece_type = class;
                self.promotion = false;
                self.post_move();
                true
            },
            None => false,
        }
    }

    // ends the game in a draw, only works if game is ongoing
    pub fn declare_draw(&mut self) {
        if self.result == ChessResult::Ongoing {
            self.result = ChessResult::Draw;
        }
    }

    // ends the game immediatly and declares a winner
    pub fn declare_win(&mut self, color: PieceColor) {
        if self.result == ChessResult::Ongoing {
            self.result = match color {
                PieceColor::White => ChessResult::WhiteWon,
                PieceColor::Black => ChessResult::BlackWon,
            };
        }
    }

    // returns a FEN string of the current game
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // 1 pieces
        let mut y = 7;
        while y >= 0 {
            let mut empty_spaces = 0;
            for x in 0..8 {
                match self.live_pieces.get(&Square::from((x, y))) {
                    Some(piece) => {
                        if empty_spaces > 0 {
                            fen.push_str(empty_spaces.to_string().as_str());
                            empty_spaces = 0;
                        }

                        let mut c = match piece.piece_type {
                            PieceType::King => 'K',
                            PieceType::Queen => 'Q',
                            PieceType::Bishop => 'B',
                            PieceType::Knight => 'N',
                            PieceType::Rook => 'R',
                            PieceType::Pawn => 'P',
                        };

                        if piece.color == PieceColor::Black {
                            c = c.to_ascii_lowercase();
                        }

                        fen.push(c);
                    },
                    None => empty_spaces += 1,
                }
            }

            if empty_spaces > 0 {
                fen.push_str(empty_spaces.to_string().as_str());
            }

            if y == 0 {
                break;
            }
            
            fen.push('/');
            y -= 1;
        }

        // 2 turn
        fen.push_str(match self.turn {
            PieceColor::White => " w ",
            PieceColor::Black => " b ",
        });

        // 3 castling rights
        let mut any_castling_rights = false;

        // white
        match self.live_pieces.get(&Square::from((4, 0))) {
            // white king
            Some(king) => {
                if king.piece_type == PieceType::King && king.has_moved == false {
                    match self.live_pieces.get(&Square::from((7, 0))) {
                        // white king rook
                        Some(rook) => {
                            if rook.piece_type == PieceType::Rook && rook.has_moved == false {
                                any_castling_rights = true;
                                fen.push('K');
                            }
                        },
                        None => (),
                    }

                    match self.live_pieces.get(&Square::from((0, 0))) {
                        // white queen rook
                        Some(rook) => {
                            if rook.piece_type == PieceType::Rook && rook.has_moved == false {
                                any_castling_rights = true;
                                fen.push('Q');
                            }
                        },
                        None => (),
                    }
                }
            },
            None => (),
        }

        // black
        match self.live_pieces.get(&Square::from((4, 7))) {
            // black king
            Some(king) => {
                if king.piece_type == PieceType::King && king.has_moved == false {
                    match self.live_pieces.get(&Square::from((7, 7))) {
                        // black king rook
                        Some(rook) => {
                            if rook.piece_type == PieceType::Rook && rook.has_moved == false {
                                any_castling_rights = true;
                                fen.push('k');
                            }
                        },
                        None => (),
                    }

                    match self.live_pieces.get(&Square::from((0, 7))) {
                        // black queen rook
                        Some(rook) => {
                            if rook.piece_type == PieceType::Rook && rook.has_moved == false {
                                any_castling_rights = true;
                                fen.push('q');
                            }
                        },
                        None => (),
                    }
                }
            },
            None => (),
        }

        if any_castling_rights {
            fen.push(' ');
        }
        else {
            fen.push_str("- ");
        }

        // 4 possible en passant targets
        match self.live_pieces.get(&self.last_moved_to) {
            Some(pawn) => {
                if pawn.piece_type == PieceType::Pawn && self.last_moved_from.moved(0, pawn.get_direction() * 2) == self.last_moved_to {
                    fen.push_str(self.last_moved_to.to_notation().to_ascii_lowercase().as_str());
                    fen.push(' ');
                }
                else {
                    fen.push_str("- ");
                }
            },
            None => fen.push_str("- "),
        }

        // 5 halfmove clock
        fen.push_str(self.fifty_move_rule.to_string().as_str());
        fen.push(' ');
        
        // 6 fullmove number
        fen.push_str(self.fullmoves.to_string().as_str());

        fen
    }

    // removes any piece in the square and updates bitmaps
    fn capture(&mut self, square: &Square) {

        // remove piece and add it to captured lsit
        match self.live_pieces.remove(&square) {
            Some(piece) => {
                match piece.color {
                    PieceColor::White => self.white_captured_pieces.push(piece.piece_type),
                    PieceColor::Black => self.black_captured_pieces.push(piece.piece_type),
                }

                // reset previous_states because it cant happen again after a capture
                self.previous_states.clear();
            },
            None => (),
        }
        
        // update bitmaps
        self.white_bitmap &= !square.to_bitmap(); 
        self.black_bitmap &= !square.to_bitmap(); 
    }

    // moves the piece and takes whatever is in the way, does not do any checks
    // will also do en passant
    fn force_move(&mut self, piece: &mut Piece, to: Square) -> Result<(), &str> {
        if to.x > 7 || to.y > 7 {
            Err("Position out of bounds!")
        } else if self.promotion {
            Err("Pawn has to be promoted first! call pawn_promotion()")
        } else {
            let pos_bitmap = to.to_bitmap();

            // increment fifty_move_rule every move. Incremented before it might be reset.
            self.fifty_move_rule += 1;
            self.capture = false;

            if (self.black_bitmap | self.white_bitmap) & pos_bitmap != 0 { // if there is other piece in pos, capture

                self.capture(&to); // we capture it (we dont care what piece it was in this function)
                self.fifty_move_rule = 0;
                self.capture = true;
            }

            // reset fifty_move_rule if a pawn was moved. Also check for en passant
            if piece.piece_type == PieceType::Pawn {
                self.fifty_move_rule = 0;

                // check for en passant
                // if it is a pawn that moves and it moved diagonally and did not capture it was en passant
                if piece.pos.x != to.x && !self.capture {
                    // if execution got here it was en passant. Capture the piece that is behind the pawn after the move
                    // determine direction of pawn
                    let direction = piece.get_direction();
                    self.capture(&to.moved(0, -direction));
                    self.capture = true;
                }

                // check for promotion
                if to.y == match piece.color {
                    PieceColor::White => 7, // white promotes at y=7
                    PieceColor::Black => 0, // black promotes at y=0
                } {
                    self.promotion = true;
                }
            }

            // castle
            let (castle_bitmap_add, castle_bitmap_remove) = if piece.piece_type == PieceType::King && !piece.has_moved {
                if piece.pos.moved(-2, 0) == to { // long castle
                    match self.live_pieces.get(&Square::from((0,piece.pos.y))) {
                        Some(rook) => {
                            let mut rook = rook.clone();
                            self.live_pieces.remove(&rook.pos);
                            rook.has_moved = true;
                            let castle_bitmap_remove = rook.pos.to_bitmap();
                            rook.pos = to.moved(1, 0);
                            let castle_bitmap_add = rook.pos.to_bitmap();
                            self.live_pieces.insert(rook.pos, rook);

                            // update bitmap
                            (castle_bitmap_add, castle_bitmap_remove)
                        },
                        None => (0, 0),
                    }
                } else if piece.pos.moved(2, 0) == to { // short castle
                    match self.live_pieces.get(&Square::from((7, piece.pos.y))) {
                        Some(rook) => {
                            let mut rook = rook.clone();
                            self.live_pieces.remove(&rook.pos);
                            rook.has_moved = true;
                            let castle_bitmap_remove = rook.pos.to_bitmap();
                            rook.pos = to.moved(-1, 0);
                            let castle_bitmap_add = rook.pos.to_bitmap();
                            self.live_pieces.insert(rook.pos, rook);

                            // update bitmap
                            (castle_bitmap_add, castle_bitmap_remove)
                        },
                        None => (0, 0),
                    }
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            };

            // update bitmap
            match piece.color {
                PieceColor::White =>  {
                    self.white_bitmap &= !(piece.pos.to_bitmap() | castle_bitmap_remove); // turn off bit we moved from
                    self.white_bitmap |= pos_bitmap | castle_bitmap_add; // turn on bit we moved to
                },
                PieceColor::Black => {
                    self.black_bitmap &= !(piece.pos.to_bitmap() | castle_bitmap_remove); // turn off bit we moved from
                    self.black_bitmap |= pos_bitmap | castle_bitmap_add; // turn on bit we moved to
                }
            }

            // we remove the piece to change its key
            self.live_pieces.remove(&piece.pos);

            // we set the last_moved data
            self.last_moved_from = piece.pos;
            self.last_moved_to = to;

            // we change some data of piece
            piece.pos = to;
            piece.has_moved = true;


            // we insert the piece so that the key was changed
            // cloning piece should be fine as it only contains primitive types
            self.live_pieces.insert(to, piece.clone());
            
            // check for check, game over, 50 move rule, and changes turn
            if !self.promotion {
                self.post_move();
            }

            Ok(())
        }
    }

    // run when a move is finished
    // checks for check, game over, 50 move rule, draw by repetition, draw by insufficient material and changes turn
    fn post_move(&mut self) {
        // determine own and other_color_bitmap
        let (own_color_bitmap, other_color_bitmap) = match self.turn {
            PieceColor::White => (self.white_bitmap, self.black_bitmap),
            PieceColor::Black => (self.black_bitmap, self.white_bitmap),
        };

        // check for check
        // get opponent king bitmap
        let other_king_bitmap = match self.live_pieces.iter().find(|(_, x)| x.piece_type == PieceType::King && x.color != self.turn) {
            Some((pos, _)) => pos.to_bitmap(),
            None => 0, // no king
        };
        self.check = false; // reset check
        // loop through own pieces
        for (_, this_turn_colored_pieces) in self.live_pieces.iter().filter(|(_, x)| { x.color == self.turn}) {
            if self.psuedo_legal_moves(this_turn_colored_pieces, own_color_bitmap, other_color_bitmap) & other_king_bitmap != 0 {
                // the opponent is put in check
                self.check = true;
                break;
            }
        }

        // 50 move rule, check mate will take precedence
        if self.fifty_move_rule >= 100 {
            self.result = ChessResult::Draw;
        }

        // draw by repetition rule, check mate will take precedence
        let board_value = BoardValue::from(&*self);
        match self.previous_states.get_mut(&board_value) {
            Some(val) => {
                *val += 1;
                if *val >= 3 {
                    self.result = ChessResult::Draw;
                }
            },
            None => _ = self.previous_states.insert(board_value, 1),
        }

        // draw by insufficient material
        if self.live_pieces.len() <= 3 {
            let mut do_draw = true;
            for (_, piece) in &self.live_pieces {
                match piece.piece_type {
                    PieceType::King => continue,
                    PieceType::Bishop => continue,
                    PieceType::Knight => continue,
                    _ => { do_draw = false; break; },
                }
            }
            if do_draw {
                self.result = ChessResult::Draw;
            }
        }

        // increment fullmoves
        if self.turn == PieceColor::Black {
            self.fullmoves += 1;
        }
        
        // change whos turn it
        self.turn = !self.turn;

        // check for game finished
        // the game is finished if there are no legal moves
        // loop through all the pieces in the next turns color
        let mut has_legal_moves = false;
        for (_, next_turn_piece) in self.live_pieces.iter().filter(|(_, x)| { x.color == self.turn}) {
            if self.legal_moves(next_turn_piece) != 0 {
                // there are legal moves
                has_legal_moves = true;
                break;
            }
        }
        if has_legal_moves == false {
            // the game is over!
            // change result
            self.result = if self.check {
                // check mate
                match self.turn {
                    PieceColor::White => ChessResult::BlackWon,
                    PieceColor::Black => ChessResult::WhiteWon,
                }
            } else {
                // stale mate
                ChessResult::Draw
            };
        }
    }

    // psuedo legal moves but removes any that puts you in check, includes castling
    fn legal_moves(&self, piece: &Piece) -> u64 {
        let (own_color_bitmap, other_color_bitmap) = match piece.color {
            PieceColor::White => (self.white_bitmap, self.black_bitmap),
            PieceColor::Black => (self.black_bitmap, self.white_bitmap)
        };
        
        let pos_bitmap = piece.pos.to_bitmap();
        let mut moves = self.psuedo_legal_moves(piece, own_color_bitmap, other_color_bitmap);

        // add castling to moves if legal
        if piece.piece_type == PieceType::King && !piece.has_moved && !self.check { // cant escape check by castling

            // castle to the left (long castle)
            match self.live_pieces.get(&Square {x: 0, y: piece.pos.y}) {
                Some(rook) => {
                    if rook.piece_type == PieceType::Rook &&
                        !rook.has_moved &&
                        bitmap_line(piece.pos, -1, 0, own_color_bitmap, other_color_bitmap) & Square::from((1, piece.pos.y)).to_bitmap() != 0 {
                        moves |= piece.pos.moved(-2, 0).to_bitmap();
                    }
                },
                None => {},
            }

            // castle to the right (short castle)
            match self.live_pieces.get(&Square {x: 7, y: piece.pos.y}) {
                Some(rook) => {
                    if rook.piece_type == PieceType::Rook &&
                        !rook.has_moved &&
                        bitmap_line(piece.pos, 1, 0, own_color_bitmap, other_color_bitmap) & Square::from((6, piece.pos.y)).to_bitmap() != 0 {
                        moves |= piece.pos.moved(2, 0).to_bitmap();
                    }
                },
                None => {},
            }
        }

        // remove everything that puts the king in check

        // create a new own_color_bitmap that represents after each move
        // if en passant is possible also make a new other_color_bitmap
        // loop through each opponents pieces and get their psuedo legal moves with the new own color bitmap
        // combine all opponents psuedo moves with | and compare with kings position using &
        // if it is 0 then the move is legal, otherwise remove that bit from the moves bitmap
        // repeat for every move in moves

        // get position of king (assumes the king wasnt moved)
        let own_king_bitmap = match self.live_pieces.iter().find(|(_, x)| x.piece_type == PieceType::King && x.color == piece.color) {
            Some((pos, _)) => pos.to_bitmap(),
            None => return moves, // if there is no king we say that all psuedo legal moves are legal
        };

        // loops through all moves in moves
        for i in 0..64 {
            if (moves >> i) & 1 == 0 {
                continue;
            }
            
            let possible_move = Square::from(i);
            let possible_move_bitmap = possible_move.to_bitmap();
            
            // remove old position and add new position to bitmap:
            let new_own_color_bitmap = (own_color_bitmap & !pos_bitmap) | possible_move_bitmap;

            // only changes if we take and only matters for en passant
            let new_other_color_bitmap;

            // scary situation here: en passant may be one of the allowed moves 
            // and we cant make the assumption that we can take a piece that was pinned because we will not occupy its square
            // we know it is en passant if the piece is a pawn and the psuedo legal move is diagonal and to an empty space
            if piece.piece_type == PieceType::Pawn && piece.pos.x != possible_move.x && other_color_bitmap & possible_move_bitmap == 0 {
                // en passant!!!
                new_other_color_bitmap = other_color_bitmap & !possible_move.moved(0, -piece.get_direction()).to_bitmap();
            } else {
                // not en passant, just remove the square the move is to from opponent bitmap
                new_other_color_bitmap = other_color_bitmap & !possible_move_bitmap;
            }

            // get king position (even if it was moved)
            let own_king_bitmap = if piece.piece_type == PieceType::King {
                possible_move_bitmap
            }
            else {
                own_king_bitmap
            };

            // loop through all opponents pieces that are still alive after the move
            for (_, opponent_piece) in self.live_pieces.iter().filter(|(_, x)| { x.color != piece.color && x.pos.to_bitmap() & new_other_color_bitmap != 0}) {
                // the opponent will have inverted own and other color bitmaps
                if self.psuedo_legal_moves(opponent_piece, new_other_color_bitmap, new_own_color_bitmap) & own_king_bitmap != 0 { // is king put in check
                    moves &= !possible_move_bitmap; // removes this move if the king was put in check
                    break;
                }
            }
        }
        // remove castle if the in between square was in check
        if piece.piece_type == PieceType::King {
            // left castle (long)
            if piece.pos.moved(-1, 0).to_bitmap() & moves == 0 {
                moves &= !piece.pos.moved(-2, 0).to_bitmap();
            }

            // right castle (short)
            if piece.pos.moved(1, 0).to_bitmap() & moves == 0 {
                moves &= !piece.pos.moved(2, 0).to_bitmap();
            }
        }

        moves
    }

    // returns a bitmap of all possible moves for that piece without considering check, and does not include castling
    fn psuedo_legal_moves(&self, piece : &Piece, own_color_bitmap : u64, other_color_bitmap : u64) -> u64 {
        match piece.piece_type {
            PieceType::King => return self.psuedo_legal_moves_king(piece, own_color_bitmap),
            PieceType::Queen => return self.psuedo_legal_moves_queen(piece, own_color_bitmap, other_color_bitmap),
            PieceType::Bishop => return self.psuedo_legal_moves_bishop(piece, own_color_bitmap, other_color_bitmap),
            PieceType::Knight => return self.psuedo_legal_moves_knight(piece, own_color_bitmap),
            PieceType::Rook => return self.psuedo_legal_moves_rook(piece, own_color_bitmap, other_color_bitmap),
            PieceType::Pawn => return self.psuedo_legal_moves_pawn(piece, own_color_bitmap, other_color_bitmap),
        }
    }
    
    // does not include castling because it should not be accounted for in check
    fn psuedo_legal_moves_king(&self, piece : &Piece, own_color_bitmap : u64) -> u64 {
        let moves :u64 =
            piece.pos.moved( 1,  0).to_bitmap() | // east
            piece.pos.moved( 1,  1).to_bitmap() | // north-east
            piece.pos.moved( 0,  1).to_bitmap() | // north
            piece.pos.moved(-1,  1).to_bitmap() | // north-west
            piece.pos.moved(-1,  0).to_bitmap() | // west
            piece.pos.moved(-1, -1).to_bitmap() | // south-west
            piece.pos.moved( 0, -1).to_bitmap() | // south
            piece.pos.moved( 1, -1).to_bitmap();  // south-east

        // cant move onto itself and its own colored pieces
        moves & !own_color_bitmap
    }
    
    fn psuedo_legal_moves_queen(&self, piece : &Piece, own_color_bitmap : u64, other_color_bitmap : u64) -> u64 {
        bitmap_line(piece.pos,  1,  0, own_color_bitmap, other_color_bitmap) | // east
        bitmap_line(piece.pos,  1,  1, own_color_bitmap, other_color_bitmap) | // north-east
        bitmap_line(piece.pos,  0,  1, own_color_bitmap, other_color_bitmap) | // north
        bitmap_line(piece.pos, -1,  1, own_color_bitmap, other_color_bitmap) | // north-west
        bitmap_line(piece.pos, -1,  0, own_color_bitmap, other_color_bitmap) | // west
        bitmap_line(piece.pos, -1, -1, own_color_bitmap, other_color_bitmap) | // south-west
        bitmap_line(piece.pos,  0, -1, own_color_bitmap, other_color_bitmap) | // south
        bitmap_line(piece.pos,  1, -1, own_color_bitmap, other_color_bitmap)   // south-east
    }
    
    fn psuedo_legal_moves_bishop(&self, piece : &Piece, own_color_bitmap : u64, other_color_bitmap : u64) -> u64 {
        bitmap_line(piece.pos,  1,  1, own_color_bitmap, other_color_bitmap) | // north-east
        bitmap_line(piece.pos, -1,  1, own_color_bitmap, other_color_bitmap) | // north-west
        bitmap_line(piece.pos, -1, -1, own_color_bitmap, other_color_bitmap) | // south-west
        bitmap_line(piece.pos,  1, -1, own_color_bitmap, other_color_bitmap)   // south-east
    }
    
    fn psuedo_legal_moves_knight(&self, piece : &Piece, own_color_bitmap : u64) -> u64 {
        let moves :u64 =
            piece.pos.moved( 2,  1).to_bitmap() | // 02:00
            piece.pos.moved( 1,  2).to_bitmap() | // 01:00
            piece.pos.moved(-1,  2).to_bitmap() | // 11:00
            piece.pos.moved(-2,  1).to_bitmap() | // 10:00
            piece.pos.moved(-2, -1).to_bitmap() | // 08:00
            piece.pos.moved(-1, -2).to_bitmap() | // 07:00
            piece.pos.moved( 1, -2).to_bitmap() | // 05:00
            piece.pos.moved( 2, -1).to_bitmap();  // 04:00

        // cant move onto itself and its own colored pieces
        moves & !own_color_bitmap
    }
    
    fn psuedo_legal_moves_rook(&self, piece : &Piece, own_color_bitmap : u64, other_color_bitmap : u64) -> u64 {
        bitmap_line(piece.pos,  1,  0, own_color_bitmap, other_color_bitmap) | // east
        bitmap_line(piece.pos,  0,  1, own_color_bitmap, other_color_bitmap) | // north
        bitmap_line(piece.pos, -1,  0, own_color_bitmap, other_color_bitmap) | // west
        bitmap_line(piece.pos,  0, -1, own_color_bitmap, other_color_bitmap)   // south
    }
    
    fn psuedo_legal_moves_pawn(&self, piece : &Piece, own_color_bitmap : u64, other_color_bitmap : u64) -> u64 {
        // used for move calculation and to determine what direction this pawn moves in
        let direction: i8 = piece.get_direction();

        let all_bitmap = own_color_bitmap | other_color_bitmap;

        let mut moves: u64 = 0;

        moves |= piece.pos.moved(0, direction).to_bitmap() & !all_bitmap; // one step

        // allow 2 steps if the pawn has not moved and it can move one step (and no collision)
        if !piece.has_moved && moves != 0 {
            moves |= piece.pos.moved(0, direction * 2).to_bitmap() & !all_bitmap;
        }

        // may take diagonally
        moves |= (piece.pos.moved(1, direction).to_bitmap() | piece.pos.moved(-1, direction).to_bitmap()) & other_color_bitmap;

        // check for en passant
        // we can assume that it is of the opponents color because last_moved_to is always the opponent
        match self.live_pieces.get(&self.last_moved_to) {
            Some(last_moved_piece) => {
                // conditions for en passant: last moved piece was a pawn and it moved 2 steps and it is next to our pawn
                if last_moved_piece.piece_type == PieceType::Pawn && // was pawn
                    self.last_moved_to.y + direction * 2 == self.last_moved_from.y && // moved 2 steps
                    (piece.pos.moved(-1, 0).to_bitmap() | piece.pos.moved(1, 0).to_bitmap()) & last_moved_piece.pos.to_bitmap() != 0 { // is next to out pawn 
                    moves |= self.last_moved_to.moved(0, direction).to_bitmap(); 
                }
            },
            None => {}, // likely the last moved data wasnt set (as in the start)
        };

        moves 
    }
    
}

#[derive(Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub pos: Square,
    pub has_moved: bool, // used for castling
}

impl Piece {
    fn get_direction(&self) -> i8 {
        match self.color {
            PieceColor::White => 1,
            PieceColor::Black => -1
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    pub x: i8, // A-H (represented in code as 0-7)
    pub y: i8, // 1-8 (represented in code as 0-7)
}

impl From<i8> for Square {
    fn from(value: i8) -> Self {
        Self {x: value % 8, y: value / 8}
    }
}

impl From<(i8, i8)> for Square {
    // initialize from a tuple of (i8, i8)
    fn from(pos: (i8, i8)) -> Self {
        Self {x: pos.0, y: pos.1}
    }
}

impl From<&str> for Square {
    // initialize from algebraic notation
    fn from(pos: &str) -> Self {
        let mut pos = pos.trim().chars();
        let x: i8 = match pos.next(){
            Some(c) => {
                match c.to_ascii_uppercase() {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    'E' => 4,
                    'F' => 5,
                    'G' => 6,
                    'H' => 7,
                    _ => -1,
                }
            },
            None => -1,
        };

        let y: i8 = match pos.next(){
            Some(c) => {
                match c {
                    '1' => 0,
                    '2' => 1,
                    '3' => 2,
                    '4' => 3,
                    '5' => 4,
                    '6' => 5,
                    '7' => 6,
                    '8' => 7,
                    _ => -1,
                }
            },
            None => -1,
        };
        Self {x: x, y: y}
    }
}

impl Square {
    // returns the square number as if they were indexed from 0,0 to 7,0 to 1,0 and so on until 7,7
    pub fn to_index(&self) -> i8 {
        self.x + self.y * 8
    }

    pub fn to_notation(&self) -> String {
        format!("{}{}", (b'A' + self.x as u8) as char, self.y + 1)
    }

    // returns the position as a bitmap, if the position is outside the board it returns 0 (empty bitmap)
    fn to_bitmap(&self) -> u64 {
        if self.x < 0 || self.x >= 8 || self.y < 0 || self.y >= 8 {
            return 0;
        }

        (1 << self.y*8) << self.x
    }

    pub fn to_tuple(&self) -> (i8, i8) {
        (self.x, self.y)
    }

    // returns a new square positioned dx, dy relative, can be outside of board!
    fn moved(&self, dx: i8, dy: i8) -> Self {
        Self { x: self.x + dx, y: self.y + dy }
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
pub enum PieceColor {
    White,
    Black,
}

impl Not for PieceColor {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChessResult {
    Ongoing,
    WhiteWon,
    BlackWon,
    Draw,
}

// makes a line from start (exclusive) until it collides with edge or a piece marked in own_color_bitmap or after colliding with a piece marked in other_color_bitmap
fn bitmap_line(start: Square,  dx: i8, dy: i8, own_color_bitmap: u64, other_color_bitmap: u64) -> u64 {
    let mut square = start;
    let mut moves = 0;
    loop  {
        square = square.moved(dx, dy);
        let new_moves: u64 = moves | (square.to_bitmap() & !own_color_bitmap);
        if new_moves == moves {
            return moves;
        } else if new_moves & other_color_bitmap != 0 {
            return new_moves;
        } else {
            moves = new_moves;
        }
    };
}

fn _print_bitmap(bitmap: u64) {
    let bits : String = format!("{:064b}", bitmap).chars().rev().collect();

    println!("8 {}\n7 {}\n6 {}\n5 {}\n4 {}\n3 {}\n2 {}\n1 {}\n  ABCDEFGH", &bits[56..64], &bits[48..56], &bits[40..48], &bits[32..40], &bits[24..32], &bits[16..24], &bits[8..16], &bits[0..8]);
    
}

fn _make_color_bitmap(live_pieces: &HashMap<Square, Piece>, color: PieceColor) -> u64 {
    let mut bitmap = 0;
    for (_, piece) in live_pieces {
        if piece.color != color {
            continue;
        }

        bitmap |= piece.pos.to_bitmap();
    }
    bitmap
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BoardValue {
    white_bitmap: u64,
    black_bitmap: u64,
    king_bitmap: u64,
    queen_bitmap: u64,
    bishop_bitmap: u64,
    knight_bitmap: u64,
    rook_bitmap: u64,
    pawn_bitmap: u64,
    data: u8, // first bit is if en passant was possible, second = left (long) castle permission, third = right (short) castle permission, +4 for black
}

impl From<&Game> for BoardValue {
    fn from(game: &Game) -> Self {
        let mut white_bitmap = 0;
        let mut black_bitmap = 0;
        let mut king_bitmap = 0;
        let mut queen_bitmap = 0;
        let mut bishop_bitmap = 0;
        let mut knight_bitmap = 0;
        let mut rook_bitmap = 0;
        let mut pawn_bitmap = 0;
        let mut data = 0;

        let live_pieces = game.get_board_state();
        let en_passant_x = match live_pieces.get(&game.last_moved_to) {
            Some(pawn) => {
                if pawn.piece_type == PieceType::Pawn && game.last_moved_from.y + pawn.get_direction() * 2 == game.last_moved_to.y {
                    game.last_moved_to.x
                } else {
                    -8
                }
            },
            None => -8,
        };

        for (square, piece) in live_pieces {
            match piece.color {
                PieceColor::White => white_bitmap |= square.to_bitmap(),
                PieceColor::Black => black_bitmap |= square.to_bitmap(),
            }

            match piece.piece_type {
                PieceType::King => king_bitmap |= square.to_bitmap(),
                PieceType::Queen => queen_bitmap |= square.to_bitmap(),
                PieceType::Bishop => bishop_bitmap |= square.to_bitmap(),
                PieceType::Knight => knight_bitmap |= square.to_bitmap(),
                PieceType::Rook => rook_bitmap |= square.to_bitmap(),
                PieceType::Pawn => pawn_bitmap |= square.to_bitmap(),
            }

            let color_bitshift: u8 = match piece.color {
                PieceColor::White => 0,
                PieceColor::Black => 4,
            };

            if piece.piece_type == PieceType::King && !piece.has_moved {
                data |= 0b0110 << color_bitshift;

                // left castle
                if match live_pieces.get(&Square::from((0, square.y))) { // if has moved
                    Some(rook) => rook.piece_type != PieceType::Rook || rook.has_moved,
                    None => true,
                } {
                    data &= !(0b0010 << color_bitshift);
                }

                // right castle
                if match live_pieces.get(&Square::from((7, square.y))) { // if has moved
                    Some(rook) => rook.piece_type != PieceType::Rook || rook.has_moved,
                    None => true,
                } {
                    data &= !(0b0100 << color_bitshift);
                }
            } else if piece.piece_type == PieceType::Pawn && piece.pos.y == match piece.color {
                PieceColor::White => 4,
                PieceColor::Black => 3,
            } {
                // en passant
                if en_passant_x == piece.pos.x - 1 || en_passant_x == piece.pos.x + 1 {
                    data |= 0b0001 << color_bitshift;
                }
            }
        }

        Self { white_bitmap, black_bitmap, king_bitmap, queen_bitmap, bishop_bitmap, knight_bitmap, rook_bitmap, pawn_bitmap, data }
    }
}

// TODO
// tests for all functions
// timer
// chess notation for importing and testing games
// exporting games
// option in Game to turn off automatic draw due to 3 repetition or 50 move rule as well as 5 repetition and 75 move rule
// make it possible to call out draw if it is not automatic
// and more!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let mut game = Game::new();
        assert!(game.turn == PieceColor::White);
        assert!(game.do_move(&Square::from("E2"), &Square::from("E4")) == true); // (white) pawn opening
        assert!(game.turn == PieceColor::Black);
        assert!(game.do_move(&Square::from("D7"), &Square::from("D5")) == true); // (black) queen side pawn

        assert!(game.do_move(&Square::from("E4"), &Square::from("D5")) == true); // (white) take pawn
        assert!(game.capture == true);
        assert!(game.do_move(&Square::from("C8"), &Square::from("E6")) == true); // (black) move bishop
        
        assert!(game.do_move(&Square::from("D1"), &Square::from("F3")) == true); // (white) move queen
        assert!(game.do_move(&Square::from("B8"), &Square::from("D7")) == true); // (black) move knight
        
        assert!(game.do_move(&Square::from("D5"), &Square::from("E6")) == true); // (white) take bishop with pawn
        assert!(game.capture == true);
        assert!(game.do_move(&Square::from("A7"), &Square::from("A5")) == true); // (black) move A-pawn
        assert!(game.capture == false);
        
        assert!(game.do_move(&Square::from("A2"), &Square::from("A3")) == true); // (white) move A-pawn
        assert!(game.do_move(&Square::from("A5"), &Square::from("A4")) == true); // (black) move A-pawn
        
        assert!(game.do_move(&Square::from("B2"), &Square::from("B4")) == true); // (white) move B-pawn
        assert!(game.do_move(&Square::from("A4"), &Square::from("A3")) == false); // (black) cant move forward
        assert!(game.do_move(&Square::from("A4"), &Square::from("B3")) == true); // (black) en passant
        assert!(game.capture == true);
        
        assert!(game.do_move(&Square::from("F3"), &Square::from("F7")) == true); // (white) move queen to mate
        assert!(game.result == ChessResult::WhiteWon);
    }

    #[test]
    fn test_piece() {
        let piece1 = Piece { piece_type: PieceType::Knight, color: PieceColor::White, pos: Square::from((0,3)), has_moved: false };
        let piece2 = Piece { piece_type: PieceType::Bishop, color: PieceColor::Black, pos: Square::from((3,0)), has_moved: false };
        
        assert!(piece1.get_direction() == 1);
        assert!(piece2.get_direction() == -1);
    }

    #[test]
    fn test_square() { // and bitmap_line()
        let square0 = Square {x: 3, y: 0};
        let square1 = Square::from(46); // (6, 5)
        let square2 = Square::from((4, 5));
        let square3 = Square::from("B3");
        
        assert!(square0.x == 3);
        assert!(square0.y == 0);

        assert!(square1.x == 6);
        assert!(square1.y == 5);

        assert!(square2.x == 4);
        assert!(square2.y == 5);

        assert!(square3.x == 1);
        assert!(square3.y == 2);

        let square4 = square3.moved(1, 2);
        assert!(square4.x == 2);
        assert!(square4.y == 4);

        assert!(square0.to_bitmap() == 0b00001000);

        assert!(square1.to_index() == 46);
        assert!(square2.to_tuple() == (4,5));
        assert!(square3.to_notation() == "B3");

        assert!(bitmap_line(square0,  1, 0, 0, 0) == 0b11110000);
        assert!(bitmap_line(square0, -1, 0, 0, 0) == 0b00000111);
        assert!(bitmap_line(square0, -1, 0, 0b10, 0) == 0b00000100);
        assert!(bitmap_line(square0, -1, 0, 0, 0b10) == 0b00000110);
    }

    #[test]
    fn test_color() {
        let col1 = PieceColor::White;
        let col2 = PieceColor::White;
        let col3 = PieceColor::Black;
        let col4 = PieceColor::Black;
        assert!(col1 == col2);
        assert!(col2 == col1);
        assert!(col3 == col4);
        assert!(col1 != col3);
        assert!(col4 != col2);
        assert!(!(col1 != col2));
    }

    // Regression tests: se till att inte buggar återuppstår

    // Fuzz testing: testa random input

    // Stress testing: Testar hur ett system hanterar exceptionella situationer

    // Benchmarks: Testar performance av kod, hård gräns på tid, t.ex realtime systems (Flygplan, spel etc)
}