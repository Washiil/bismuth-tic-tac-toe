#[derive(Debug)]
pub enum TicTacToeError {
    MoveError(MoveError),
    GameStateError(GameStateError),
    BoardError(BoardError),
    PlayerError(PlayerError),
}

#[derive(Debug)]
pub enum MoveError {
    InvalidPosition,  // Out of bounds
    OccupiedCell,     // Trying to move on an already occupied cell
}

#[derive(Debug)]
pub enum GameStateError {
    GameAlreadyOver,
    NotYourTurn,
}

#[derive(Debug)]
pub enum BoardError {
    OutOfBounds,
    InvalidConfiguration,  // For any configuration errors in the board
}

#[derive(Debug)]
pub enum PlayerError {
    UnknownPlayer,
    InvalidSymbol,
}
