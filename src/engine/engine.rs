#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::marker::PhantomData;

use anyhow::anyhow;
use console::Term;
use rand::seq::IndexedRandom;
use rand::rng;

use crate::engine::models::r#move::Move;
use crate::engine::search::evaluation::Evaluation;
use crate::engine::{models::board::Chessboard, movegen::generate_moves, search::Search};
use crate::pause;

/// `Not Connected` State for the engine.
pub struct NotConnected;
/// `Connected` State for the engine, meaning the uci protocol and connection has been established and validated.
pub struct Connected;
/// This is the entry of our chess engine, which will be used to start the game using a chessboard
/// 
/// The engine will support:
/// - UCI game against another player (i.e. via Litchess bridge)
/// - UCI game against another engine
/// - UCI game against itself
/// - game against itself
/// 
/// # Exemples
/// ```rust
/// use lib::engine::engine::Engine;
/// 
/// let mut engine = Engine::new();
/// // let mut engine = engine.validate_uci_connection().unwrap();
/// // engine.start_uci_game(); // connecting and playing against another player using the litchess bot bridge
/// ```
pub struct Engine<State = NotConnected> {
    /// Internal chessboard used to play by the engine itself.
    chessboard: Chessboard,
    search: Search,
    /// State of the engine, refer to [NotConnected] and [Connected].
    state: PhantomData<State>
}

impl Default for Engine<NotConnected> {
    fn default() -> Engine<NotConnected> {
        Self::new()
    }
}

impl Engine<NotConnected> {
    /// Validate the uci protocol and ready to listen to next uci commands after `uciok`.
    pub fn validate_uci_connection(self) -> anyhow::Result<Engine<Connected>> {
        let protocol = Term::stdout().read_line()?;
    
        if protocol.trim() != "uci" {
            return Err(anyhow!("Invalid UCI protocol"));
        }
        
        println!("id name chessengine");
        println!("id author Jojo");
        println!("option name Move Overhead type spin default 30 min 0 max 5000");
        println!("option name Threads type spin default 4 min 1 max 12");
        println!("option name Hash type spin default 512");
        println!("option name SyzygyPath type string default './syzygy/'");
        println!("option name UCI_ShowWDL type check default true");
        println!("uciok");  

        Ok(Engine { 
            chessboard: Chessboard::new(),
            search: Search::new(3),
            state: PhantomData::<Connected> 
        })
    }

    /// This method starts game against itself, the engine or AI will return after each of its turn its corresponding "best move".
    pub fn start_self_game(&mut self) {
        let mut turn_counter = 0;
        loop {    
            let best_move = self.search.think(&mut self.chessboard);
    
            if let Some(best_move) = best_move {
                self.chessboard.make(&best_move);
                println!("chessboard:\n{}", self.chessboard);
                pause(&format!("-------------- {} {} {}", turn_counter, best_move, Evaluation::evaluate(&self.chessboard)));
            }

            turn_counter += 1;
        }
    }

    /// Start a game against itself with by playing only random moves.
    pub fn start_random_game(&mut self) {
        let mut turn_counter = 0;
        loop {
            let moves = generate_moves(&self.chessboard);
            let moves: Vec<&Move> = moves.iter().filter_map(|mv| {
                self.chessboard.make(mv);
                if !self.chessboard.is_in_check() {
                    self.chessboard.unmake(mv);
                    return Some(mv);
                }
                self.chessboard.unmake(mv);
                None
            }).collect();
    
            let mut rng = rng();
            let random_move = moves.choose(&mut rng);
    
            if let Some(random_move) = random_move {
                self.chessboard.make(&random_move);
                println!("chessboard:\n{}", self.chessboard);
                pause(&format!("-------------- {}", turn_counter));
            }

            turn_counter += 1;
        }
    } 
}

impl Engine<Connected> {
    /// This method starts an UCI game, the engine or AI will return after each of its turn its corresponding "best move" as UCI encoding.
    pub fn start_uci_game(&mut self) -> anyhow::Result<()> {
        loop {
            let input = Term::stdout().read_line()?;
            let mut parts = input.split(' ');
            let command = parts.next().ok_or(anyhow!("no command found"))?;
            let remaining: Vec<&str> = parts.collect();

            match command {
                "quit" => {
                    break;
                },
                "isready" => {
                    println!("readyok");
                },
                "position" => {
                    todo!("position update");
                },
                "go" => {
                    let best_move = self.search.think(&mut self.chessboard);
                    if let Some(best_move) = best_move {
                        println!("bestmove {}", best_move);
                    }
                },
                _ => {
                    return Err(anyhow!("xd"));
                }
            }
        }

        Ok(())
    }
}

impl Engine {
    /// Initializing the engine's chessboard with the classic starting chess position.
    pub fn new() -> Engine {
        Engine {
            chessboard: Chessboard::new(),
            search: Search::new(3),
            state: PhantomData::<NotConnected>
        }
    }

    /// Initializing the engine's chessboard with a custom position, parsed using fen.
    pub fn from_fen(fen: &str) -> Result<Engine, &str> {
        Ok(Engine {
            chessboard: Chessboard::from_fen(fen)?,
            search: Search::new(3),
            state: PhantomData::<NotConnected>
        })
    }
}