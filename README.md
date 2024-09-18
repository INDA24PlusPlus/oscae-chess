# oscae-chess
A chess library written in Rust!
You can add it to your `Cargo.toml` file like this:
```toml
[dependencies]
oscae-chess = { git = "https://github.com/INDA24PlusPlus/oscae-chess.git" }
```

# Usage
```rust
extern crate oscae_chess;
use oscae_chess::*;
```
Create a new instance of Game to start a chess match:

`let mut game = Game::new();`

Use `game.get_board_state()` to get piece information for displaying the chess board.

When a square is clicked, use `game.get_moves_list()` or `game.get_moves_bitmap()` to get legal moves for a square.

Use `game.do_move()` to move pieces.

After a move, if `game.promotion == true` it is **VERY IMPORTANT** to call `game.pawn_promotion()` to specify what the pawn should be promoted to.

As long as `game.result == ChessResult::Ongoing` the game is not finished. Use this for your game loop.

## Additional features
You may use `game.last_moved_from` and `game.last_moved_to` to highlight the squares of the last move.

`game.capture` and `game.check` can be used for effects when a piece is captured or when the king is in check. Such as a different move sound.

# The Game struct
## Variables
`turn: PieceColor`
Tells whoose turn it is

`result: ChessResult`
Tells the state of the game. (Ongoing, WhiteWon, BlackWon or Draw) You can have you game loop run whenever `game.result == ChessResult::Ongoing`.

`last_moved_from: Square`
Represents the square that the last move was made from. Initialized as (-1, -1). Can be used for square highlighting.

`last_moved_to: Square`
Represents the square that the last move was made to. Initialized as (-1, -1). Can be used for square highlighting.

`capture: bool`
True if the last move was a capture.

`check: bool`
True if the current player's king is in check.

`promotion: bool`
True if the move lead to promotion and `pawn_promotion()` has to be called. **This is important!**

`white_captured_pieces : Vec<PieceType>`
A list of white pieces that have been captured. In order of capture, first to last.

`black_captured_pieces : Vec<PieceType>`
A list of black pieces that have been captured. In order of capture, first to last.

## Functions
`new() -> Self`
Creates and returns a new instance of `Game`.

`get_board_state(&self) -> &HashMap<Square, Piece>`
Returns an immutable reference to the HashMap of Squares and Pieces.

`get_moves_list(&self, from : &Square) -> Vec<Square>`
Returns a vec of Square, of all legal moves that can be made from the square "from", considering turn.

`get_moves_bitmap(&self, from: &Square) -> u64`
Returns a bitmap of all legal moves that can be made from the square "from", considering turn. Such that the least significant bit represents **A1**, the next **B1** and the most signigicant bit represents **H8**.

`do_move(&mut self, from: &Square, to: &Square) -> bool`
Move a piece by specifying its square and where to move it, and returns true if it was successful.

`pawn_promotion(&mut self, class: PieceType) -> bool`
Run this function whenever `game.promotion == true` after a move to select the kind of piece to promote a pawn to. Returns false if invalid PieceType was passed.

`declare_draw(&mut self)`
Ends the game in a draw, only works for ongoing games.

`declare_win(&mut self, color: PieceColor)`
Ends the game immediatly and declares a winner, only works for ongoing games.

# Structs
This section explains the public structs that are used in the API.
## Piece
`piece_type: PieceType`
Represents the type of the piece. (King, Queen, Bishop, Knight, Rook, Pawn)

`color: PieceColor`
Represents the color of the piece. (White, Black)

`pos: Square`
Represents tha piece's position on the board.

`has_moved: bool`
True if the piece has been moved once during the game.

## Square
Stores a position on the board.
### Variables
`x: i8`
Represents the position on the x-axis `0-7` meaning `A-H` in chess coordinates.

`y: i8`
Represents the position on the y-axis `0-7` meaning `1-8` in chess coordinates.

### Constructors
`from(value: i8) -> Self`
Creates a Square of from an index that starts at `0` for **A1**, `1` for **B1**, `63` for **H8**.

`from(pos: (i8, i8)) -> Self`
Creates a Square from a tuple `(x: i8, y: i8)`, values from `0-7` are on the board.

`from(pos: &str) -> Self`
Creates a Square from a `&str` that is a chess chess coordinate such as `A1`, `B1` or `H8`.

### Functions
Each contructor has a corresponding function that does the opposite.

`to_index(&self) -> i8`
Returns an index corresponding to the square where **A1** returns `0`, **B1** returns `1` and **H8** returns `63`.

`to_tuple(&self) -> (i8, i8)`
Returns a tuple `(x: i8, y: i8)` such that x and y ranges from `0-7`.

`to_notation(&self) -> String`
Returns a String that represents the square in chess coordinates such as `A1`, `B1` or `H8`.

# Enums
This section explains the public enums that are used in the API.

## PieceType
PieceType is copied by default and can have the following values:
`King`, `Queen`, `Bishop`, `Knight`, `Rook`, `Pawn`

## PieceColor
PieceColor is copied by default and can have the following values:
`White`, `Black`

PieceColor also implemets Not such that `!PieceColor::White == PieceColor::Black` and vice versa.

## ChessResult
ChessResult is copied by default and can have the following values:
`Ongoing`, `WhiteWon`, `BlackWon`, `Draw`

# Board index reference
![ChessBoardIndex](https://github.com/user-attachments/assets/2b826e80-896c-4cf1-a95e-a2023cc31dc1)
