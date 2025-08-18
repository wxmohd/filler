pub mod strategy;
pub mod evaluation;

pub use strategy::Strategy;
pub use evaluation::Evaluator;

use crate::game::{GameState, GameParser};
use std::io::{self, BufRead};

pub struct FillterAI {
    player_id: Option<u8>,
    strategy: Strategy,
}

impl FillterAI {
    pub fn new() -> Self {
        Self {
            player_id: None,
            strategy: Strategy::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match GameParser::read_game_input() {
                Ok((player_id, anfield, piece)) => {
                    // Set player ID on first input
                    if self.player_id.is_none() {
                        self.player_id = player_id;
                    }

                    // Process the game state and make a move
                    if let (Some(anfield), Some(piece)) = (anfield, piece) {
                        self.make_move(anfield, piece);
                    }
                }
                Err(_) => {
                    // Input error, output invalid move and exit
                    GameParser::output_move(0, 0);
                    break;
                }
            }
        }
    }

    fn make_move(&self, anfield: crate::game::Anfield, piece: crate::game::Piece) {
        let player_id = self.player_id.unwrap_or(1);
        
        // Create game state
        let mut game_state = GameState::new(anfield.width, anfield.height);
        game_state.anfield = anfield;
        game_state.current_piece = Some(piece.clone());
        game_state.current_player = player_id;

        // Update player territories from anfield
        game_state.player1.territory = game_state.anfield.get_player_territory(1);
        game_state.player2.territory = game_state.anfield.get_player_territory(2);
        game_state.update_scores();

        // Find best move
        if let Some((x, y)) = self.strategy.find_best_move(&game_state, &piece, player_id) {
            GameParser::output_move(x, y);
        } else {
            // No valid moves available
            GameParser::output_move(0, 0);
        }
    }
}