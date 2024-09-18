# oscae-chess
# Usage
# The Game struct
# Structs and Enums

`pub fn new() -> Self`

// returns a reference to the hashmap of live pieces
`pub fn get_board_state(&self) -> &HashMap<Square, Piece>`

// returns a vec of Square, of all legal moves that can be made from the square "from" considering turn
`pub fn get_moves_list(&self, from : &Square) -> Vec<Square>`

// returns a bitmap of all legal moves that can be made from the square "from" considering turn
`pub fn get_moves_bitmap(&self, from: &Square) -> u64`

// does a move and returns true if it was successful
`pub fn do_move(&mut self, from: &Square, to: &Square) -> bool`

// selects the piece to promote a pawn to. will return false if invalid PieceType whas passed
`pub fn pawn_promotion(&mut self, class: PieceType) -> bool`

// ends the game in a draw, only works if game is ongoing
`pub fn declare_draw(&mut self)`

// ends the game immediatly and declares a winner
`pub fn declare_win(&mut self, color: PieceColor)`