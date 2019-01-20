use super::uci::Engine;

pub struct Computer{
    engine: Engine,
}

impl Computer {
    pub fn new_from_path(path: &str) -> Result<Computer, String> 
    {
        match Engine::new(path){
            Ok(engine) => Ok(Computer{ engine }),
            Err(e) => Err(format!("error: {}", e))
        }
    }

    pub fn set_engine_position(&self, position_fen: &str)
    {
        self.engine.set_position(position_fen);
    }

    pub fn get_bestmove(&self) -> String
    {
        self.engine.bestmove()
    }
}