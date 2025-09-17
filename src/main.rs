use filler::{GameState, AI};
pub mod game_state;
pub mod game_piece;
pub mod strategy_engine;

use crate::strategy_engine::StrategyEngine;

fn main() {
    let mut game_state = GameState::new();
    
    // Read player assignment
    if game_state.read_player_assignment().is_err() {
        return;
    }
    
    loop {
        // Read game state
        match game_state.read_game_state() {
            Ok(true) => {
                // Find and output best move
                let (x, y) = AI::find_best_move(&game_state);
                if game_state.output_move(x, y).is_err() {
                    break;
                }
            }
            _ => break,
        }
    }
}
