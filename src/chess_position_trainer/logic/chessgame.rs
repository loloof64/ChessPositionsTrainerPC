use chess::{Board, Color};

pub struct ChessGame
{
    board: Board,
}

impl ChessGame
{
    pub fn new_from_fen(position_str: &str) -> Option<ChessGame>
    {
        match Board::from_fen(String::from(position_str)) {
            Some(board) => Some(ChessGame {
                board,
            }),
            _ => None
        }
    }

    pub fn is_white_turn(&self) -> bool
    {
        self.board.side_to_move() == Color::White
    }

    /*pub fn get_position_fen(&self) -> &str
    {
        self.board
    }*/
}