use pleco::{Board};
use pleco::core::{Piece, Player, PieceType};
use pleco::core::sq::{SQ};
use pleco::core::piece_move::{BitMove};
use pleco::core::move_list::{MoveList};

#[derive(Clone)]
pub struct ChessGame
{
    position: Board,
}

impl ChessGame
{
    pub fn new_from_fen(position_str: &str) -> Option<ChessGame>
    {
        match Board::from_fen(position_str) {
            Ok(board) => Some(
                ChessGame{
                    position: board,
                }
            ),
            _ => None
        }
    }

    pub fn is_white_turn(&self) -> bool
    {
        self.position.turn() == Player::White
    }

    pub fn piece_at_cell(&self, cell: SQ) -> Piece
    {
        self.position.piece_at_sq(cell)
    }

    pub fn is_legal_move(&self, start_cell: SQ, end_cell: SQ) -> bool {
        match self.get_matching_move(start_cell, end_cell, PieceType::None, true) {
            Some(_) => true,
            None => false
        }
    }

    pub fn is_promotion_move(&self, start_cell: SQ, end_cell: SQ) -> bool {
        let move_to_test = self.get_matching_move(start_cell, end_cell, PieceType::None, true);
        match move_to_test {
            Some(move_to_test) => move_to_test.is_promo(),
            None => false
        }
    }

    pub fn do_move(&mut self, start_cell: SQ, end_cell: SQ, promotion: PieceType) {
        if let Some(move_to_execute) = self.get_matching_move(start_cell, end_cell, promotion, false) {
            self.position.apply_move(move_to_execute);
        }
    }

    fn get_matching_move(&self, start_cell: SQ, end_cell: SQ,
         expected_promotion: PieceType, skip_promotion_test: bool) -> Option<BitMove> {
        let position: Board = self.position.shallow_clone();
        let legal_moves_list: MoveList = position.generate_moves();

        let expected_moves: Vec<&BitMove> = legal_moves_list.iter().filter(|m| {            
            let mut test = m.get_src() == start_cell &&
                m.get_dest() == end_cell;
            if m.is_promo() && !skip_promotion_test {
                test &= m.promo_piece() == expected_promotion;
            }

            test
        }).collect::<Vec<&BitMove>>();

        if expected_moves.len() > 0 { 
            let the_move = expected_moves[0].clone();
            Some(the_move)
        } else { None }
    }
}