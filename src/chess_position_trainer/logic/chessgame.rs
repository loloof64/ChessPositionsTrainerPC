use shakmaty::{Piece, Square, Role, MoveList, Move, Position, Chess};
use shakmaty::fen::{Fen, epd};
use super::computer::Computer;

pub struct ChessGame
{
    position: Fen,
    computer: Computer,
}

impl ChessGame
{
    pub fn new_from_fen(position_str: &str) -> Result<ChessGame, String>
    {
        let engine_path = "/home/laurent-bernabe/Android"; //"/home/laurent-bernabe/Programmes/Echecs/Moteurs/stockfish-10-linux/Linux/stockfish_10_x64_modern";
        let computer = Computer::new_from_path(engine_path);
        match computer {
            Ok(computer) => {
                match Fen::from_ascii(position_str.as_bytes()) {
                    Ok(position) => Ok(
                        ChessGame{
                            position,
                            computer,
                        }
                    ),
                    _ => Err(format!("The position {} is illegal !", position_str))
                }
            },
            Err(error) => Err(format!("{}", error))
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

    pub fn is_legal_move<P: Position>(&self, start_cell: (u8, u8), end_cell: (u8, u8)) -> bool {
        let move_to_test = self.get_matching_move::<Chess>(start_cell, end_cell, None, true);
        let position: P = self.position.position().expect("Failed to get current position !");
        match move_to_test {
            Some(move_to_test) => position.is_legal(&move_to_test),
            None => false
        }
    }

    pub fn is_promotion_move(&self, start_cell: (u8, u8), end_cell: (u8, u8)) -> bool {
        let move_to_test = self.get_matching_move::<Chess>(start_cell, end_cell, None, true);
        match move_to_test {
            Some(move_to_test) => move_to_test.is_promotion(),
            None => false
        }
    }

    pub fn do_move<P: Position>(&mut self, start_cell: (u8, u8), end_cell: (u8, u8), promotion: Option<Role>) {
        let move_to_execute = self.get_matching_move::<Chess>(start_cell, end_cell, promotion, false).expect("Not a legal move !");
        let mut position: P = self.position.position().expect("Failed to get current position !");
        position.play_unchecked(&move_to_execute);
        self.position = Fen::from_ascii(epd(&position).as_bytes()).expect("Failed to save the move !");
    }

    fn get_matching_move<P: Position>(&self, start_cell: (u8, u8), end_cell: (u8, u8),
         expected_promotion: Option<Role>, skip_promotion_test: bool) -> Option<Move> {
        let position: P = self.position.position().expect("Failed to get current position !");
        let mut legal_moves_list = MoveList::new();
        position.legal_moves(&mut legal_moves_list);

        let expected_moves: Vec<&Move> = legal_moves_list.iter().filter(|m| {
            let mut test = m.from().expect("The move has no from square !") == 
                Square::from_index((start_cell.0 + start_cell.1 * 8) as i8).expect("Failed to build square") &&
            m.to() == 
                Square::from_index((end_cell.0 + end_cell.1 * 8) as i8).expect("Failed to build square");
            if !skip_promotion_test {
                test &= m.promotion() == expected_promotion;
            }

            test
        }).collect::<Vec<&Move>>();

        if expected_moves.len() > 0 {
            let the_move = expected_moves[0].clone();
            Some(the_move)
        } else { None }
    }

    pub fn get_bestmove(&self) -> String
    {
        self.computer.set_engine_position(epd(&self.position).as_ref());
        self.computer.get_bestmove()
    }
}