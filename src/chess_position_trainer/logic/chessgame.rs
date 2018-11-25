use shakmaty::{Piece, Square};
use shakmaty::fen::Fen;

#[derive(Clone)]
pub struct ChessGame
{
    position: Fen,
}

impl ChessGame
{
    pub fn new_from_fen(position_str: &str) -> Option<ChessGame>
    {
        match Fen::from_ascii(position_str.as_bytes()) {
            Ok(fen) => Some(
                ChessGame{
                    position: fen,
                }
            ),
            _ => None
        }
    }

    pub fn is_white_turn(&self) -> bool
    {
        self.position.turn.is_white()
    }

    pub fn piece_at_cell(&self, file: i8, rank: i8) -> Option<Piece>
    {
        self.position.board.piece_at(Square::new((file + rank * 8) as i8))
    }
}