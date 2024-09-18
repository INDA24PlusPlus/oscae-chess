# oscae-chess
# Usage
# The Game struct
## Variables
`pub turn: PieceColor`
Rells whoose turn it is

`pub result: ChessResult`
Tells the state of the game. (Ongoing, WhiteWon, BlackWon or Draw) You can have you game loop run whenever `game.result == ChessResult::Ongoing`.

`pub last_moved_from: Square`
Represents the square that the last move was made from. Initialized as (-1, -1). Can be used for square highlighting.

`pub last_moved_to: Square`
Represents the square that the last move was made to. Initialized as (-1, -1). Can be used for square highlighting.

`pub capture: bool`
True if the last move was a capture.

`pub check: bool`
True if the current player's king is in check.

`pub promotion: bool`
True if the move lead to promotion and `pawn_promotion()` has to be called. **This is important!**

`pub white_captured_pieces : Vec<PieceType>`
A list of white pieces that have been captured. In order of capture, first to last.

`pub black_captured_pieces : Vec<PieceType>`
A list of black pieces that have been captured. In order of capture, first to last.

## Functions
`pub fn new() -> Self`
Creates and returns a new instance of `Game`.

`pub fn get_board_state(&self) -> &HashMap<Square, Piece>`
Returns an immutable reference to the HashMap of Squares and Pieces.

`pub fn get_moves_list(&self, from : &Square) -> Vec<Square>`
Returns a vec of Square, of all legal moves that can be made from the square "from", considering turn.

`pub fn get_moves_bitmap(&self, from: &Square) -> u64`
Returns a bitmap of all legal moves that can be made from the square "from", considering turn. Such that the least significant bit represents **A1**, the next **B1** and the most signigicant bit represents **H8**.

`pub fn do_move(&mut self, from: &Square, to: &Square) -> bool`
Move a piece by specifying its square and where to move it, and returns true if it was successful.

`pub fn pawn_promotion(&mut self, class: PieceType) -> bool`
Run this function whenever `game.promotion == true` after a move to select the kind of piece to promote a pawn to. Returns false if invalid PieceType was passed.

`pub fn declare_draw(&mut self)`
Ends the game in a draw, only works for ongoing games.

`pub fn declare_win(&mut self, color: PieceColor)`
Ends the game immediatly and declares a winner, only works for ongoing games.


# Structs and Enums