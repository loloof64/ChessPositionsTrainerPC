/*
    Original code by crazymerlyn
    https://github.com/crazymerlyn/uci-rs

    This version does not panick if the given engine path is wrong
*/
use std::process::{Child, Command, Stdio};

use std::io::Read;
use std::io::Write;

use std::fmt;
use std::thread;
use std::time::Duration;

use std::cell::RefCell;

mod error
{
    use std;
    use std::fmt;
    use std::convert::From;
    use std::io;

    /// The error type for any errors encountered with the engine.
    #[derive(Debug)]
    pub enum EngineError {
        /// Wrapper around any io errors encountered while trying to communicate with the engine.
        Io(io::Error),

        /// Engine doesn't recognize the specified option.
        UnknownOption(String),
    }

    impl fmt::Display for EngineError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                EngineError::Io(ref err) => write!(f, "IO error: {}", err),
                EngineError::UnknownOption(ref option) => write!(f, "No such option: '{}'", option.as_str())
            }
        }
    }

    impl std::error::Error for EngineError {
        fn description(&self) -> &str {
            match *self {
                EngineError::Io(ref err) => err.description(),
                EngineError::UnknownOption(..) => "Unknown option"
            }
        }

        fn cause(&self) -> Option<&std::error::Error> {
            match *self {
                EngineError::Io(ref err) => Some(err),
                EngineError::UnknownOption(..) => None
            }
        }
    }

    impl From<io::Error> for EngineError {
        fn from(err: io::Error) -> EngineError {
            EngineError::Io(err)
        }
    }
}
pub use self::error::EngineError;

pub struct Engine {
    engine: RefCell<Child>,

    movetime: u32
}

const DEFAULT_TIME: u32 = 100;

impl Engine {
    /// Create a new [`Engine`] instance.
    ///
    /// # Arguments
    /// 
    /// * `path` - The path to the engine executable.
    ///
    /// # Return
    ///
    /// Result of Engine or String
    pub fn new(path: &str) -> Result<Engine, String>
    {
        let cmd_result = Command::new(path)
                          .stdin(Stdio::piped())
                          .stdout(Stdio::piped())
                          .spawn();

        match cmd_result {
            Err(_) => Err("Engine could not be created !".to_owned()),
            Ok(cmd) => {
                let res = Engine {
                engine: RefCell::new(cmd),
                    movetime: DEFAULT_TIME
                };

                res.read_line();
                res.command("uci");

                Ok(res)
            }
        }
    }
    
    /// Changes the amount of time the engine spends looking for a move
    ///
    /// # Arguments
    /// 
    /// * `new_movetime` - New timelimit in milliseconds
    pub fn movetime(mut self, new_movetime: u32) -> Engine {
        self.movetime = new_movetime;
        self
    }
    
    /// Asks the engine to play the given moves from the initial position on it's internal board.
    /// 
    /// # Arguments
    ///
    /// * `moves` - A list of moves for the engine to play. Uses Coordinate notation
    ///
    /// # Examples
    ///
    /// ```
    /// let engine = uci::Engine::new("stockfish").unwrap();
    /// let moves = vec!["e2e4".to_string(), "e7e5".to_string()];
    /// engine.make_moves(&moves).unwrap();
    /// ```
    pub fn make_moves(&self, moves: &[String]) {
        self.write_fmt(format_args!("position startpos moves {}\n",
                                    moves.join(" ")));
    }
    
    /// Asks the engine to use the position represented by the given FEN string
    /// 
    /// # Examples
    ///
    /// ```
    /// let engine = uci::Engine::new("stockfish").unwrap();
    /// engine.set_position("2k4R/8/3K4/8/8/8/8/8 b - - 0 1").unwrap();
    /// assert_eq!(engine.bestmove().unwrap(), "c8b7");
    /// ```
    pub fn set_position(&self, fen: &str) {
        let moves: Vec<String> = vec![];
        self.make_moves_from_position(fen, &moves);
    }
    
    /// Asks the engine to use the position represented by the given FEN string
    /// and then play the given moves from that position
    pub fn make_moves_from_position(&self, fen: &str, moves: &Vec<String>) {
        self.write_fmt(format_args!("position fen {} moves {}\n",
                                    fen, moves.join(" ")));
    }
    
    /// Returns the best move in the current position according to the engine
    pub fn bestmove(&self) -> String {
        self.write_fmt(format_args!("go movetime {}\n", self.movetime));
        loop {
            let s = self.read_line();
            debug!("{}", s);
            if s.starts_with("bestmove") {
                return s.split(" ").collect::<Vec<&str>>()[1].trim().to_string();
            }
        }
    }
    
    /// Sets an engine specific option to the given value
    ///
    /// # Arguments
    ///
    /// * `name`  - Name of the option
    /// * `value` - New value for the option
    ///
    /// # Examples
    ///
    /// ```
    /// let engine = uci::Engine::new("stockfish").unwrap();
    /// engine.set_option("Skill Level", "5").unwrap();
    /// ```
    pub fn set_option(&self, name: &str, value: &str) -> Result<(), EngineError> {
        self.write_fmt(format_args!("setoption name {} value {}\n",
                                    name, value));
        let error_msg =  self.read_left_output();
        
        if error_msg.trim().is_empty() {
            Ok(())
        } else {
            Err(EngineError::UnknownOption(name.to_string()))
        }
    }
    
    /// Sends a command to the engine and returns the output
    ///
    /// # Examples
    ///
    /// ```
    /// let engine = uci::Engine::new("stockfish").unwrap();
    /// let analysis = engine.command("go depth 10").unwrap();
    /// println!("{}", analysis);
    /// ```
    pub fn command(&self, cmd: &str) -> String {
        self.write_fmt(format_args!("{}\n", cmd.trim()));
        thread::sleep(Duration::from_millis(100));
        self.read_left_output()
    }

    fn read_left_output(&self) -> String {
        let mut s: Vec<String> = vec![];

        self.write_fmt(format_args!("isready\n"));
        loop {
            let next_line = self.read_line();
            match next_line.trim() {
                "readyok" => return s.join("\n"),
                other     => s.push(other.to_string())
            }
        }
    }

    fn write_fmt(&self, args: fmt::Arguments) {
        info!("Command: {:?}", fmt::format(args));
        self.engine.borrow_mut().stdin.as_mut().unwrap().write_fmt(args).expect("Could not send command to input !");
    }

    fn read_line(&self) -> String {
        let mut s = String::new();
        let mut buf: Vec<u8> = vec![0];

        loop {
            self.engine.borrow_mut().stdout.as_mut().unwrap().read(&mut buf).expect("Could not read line from output !");
            s.push(buf[0] as char);
            if buf[0] == '\n' as u8 {
                break
            }
        }
        s
    }
}