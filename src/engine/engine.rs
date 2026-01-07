#![warn(missing_docs, dead_code)]
#![deny(unused_imports, unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::io::{self, Lines, StdinLock, Write};
pub type UciInput<'a> = Lines<StdinLock<'a>>;

use std::marker::PhantomData;
use anyhow::anyhow;
use rand::seq::IndexedRandom;
use rand::rng;

use crate::engine::models::r#move::Move;
use crate::engine::movegen::generate_legal_moves;
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

impl Engine<NotConnected> {
    /// Validate the uci protocol and ready to listen to next uci commands after `uciok`.
    pub fn validate_uci_connection<'a>(self, input: &mut UciInput<'a>) -> anyhow::Result<Engine<Connected>> {
        let protocol = input
            .next()
            .ok_or(anyhow!("stdin closed"))??;
        
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
        io::stdout().flush()?; // IMPORTANT

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

    pub fn play_against_player(&mut self) {
        let mut turn_counter = 0;
        loop {
            //player always plays white, need to play uci
            println!("enter your uci encoded move:");
            let uci = console::Term::stdout().read_line().unwrap();
            let decoded_move = &Move::decode_uci(&uci, &self.chessboard).unwrap();
            let possibles_moves = generate_legal_moves(&mut self.chessboard);

            if possibles_moves.iter().any(|mv| mv.to_string() == decoded_move.to_string()) {
                self.chessboard.make(decoded_move);
            }
            else {
                continue;
            }


            let moves = generate_legal_moves(&mut self.chessboard);
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
    pub fn start_uci_game<'a>(&mut self, input: &mut UciInput<'a>) -> anyhow::Result<()> {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        for line in input {
            let line = line?;
            let line = line.trim();

            match line {
                "isready" => {
                    writeln!(stdout, "readyok")?;
                }
                "quit" => break,
                cmd if cmd.starts_with("position") => {
                    let mut parts = cmd.split_whitespace();

                    match parts.next() {
                        Some("position") => {}
                        _ => return Ok(()),
                    }

                    match parts.next() {
                        Some("startpos") => {
                            self.chessboard = Chessboard::new();
                        }
                        Some("fen") => {
                            let fen: String = parts.by_ref().take(6).collect::<Vec<_>>().join(" ");
                            self.chessboard = Chessboard::from_fen(&fen).unwrap();
                        }
                        _ => {}
                    }

                    if let Some("moves") = parts.next() {
                        for mv in parts {
                            if let Ok(mv) = Move::decode_uci(mv, &self.chessboard) {
                                self.chessboard.make(&mv);
                            } else {
                                // INVALID MOVE → IGNORE (never panic)
                            }
                        }
                    }
                }

                cmd if cmd.starts_with("go") => {
                    if let Some(best_move) = self.search.think(&mut self.chessboard) {
                        writeln!(stdout, "bestmove {}", best_move)?;
                    }
                }
                _ => {
                    // IGNORE unknown commands (REQUIRED by UCI)
                }
            }

            stdout.flush()?;
        }

        Ok(())
    }    
}

pub struct EngineBuilder {
    chessboard: Option<Chessboard>,
    search: Option<Search>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self { chessboard: None, search: None }
    }
    
    pub fn default_fen(mut self) -> Self {
        self.chessboard = Some(Chessboard::new());
        self
    }

    pub fn from_fen(mut self, fen: &str) -> Result<Self, &str> {
        self.chessboard = Some(Chessboard::from_fen(fen)?);
        Ok(self)
    }

    pub fn search(mut self, depth: i32) -> Self {
        self.search = Some(Search::new(depth));
        self
    }

    pub fn build(self, ) -> Result<Engine<NotConnected>, String> {
        if let Some(chessboard) = self.chessboard && let Some(search) = self.search {
            return Ok(Engine { 
                chessboard, 
                search,
                state: PhantomData::<NotConnected>
            })
        }
        else {
            Err("Missing engine fields".to_owned())
        }
    }
}