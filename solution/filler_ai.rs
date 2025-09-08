mod game_state;
mod game_piece;
mod input_parser;
mod strategy_engine;

use std::io::{self, Write};
use game_state::GameState;
use game_piece::GamePiece;
use input_parser::{read_game_state, read_piece_data};
use strategy_engine::StrategyEngine;

fn main() {
    let mut game_input = String::new();
    
    // Initialize player identification with better error handling
    if io::stdin().read_line(&mut game_input).is_err() {
        std::process::exit(1);
    }
    
    let player_id = if game_input.to_lowercase().contains("p1") { 1 } else { 2 };
    let strategy_engine = StrategyEngine::new(player_id);
    
    // Main game loop with robust error handling
    loop {
        match read_game_state(&mut game_input, player_id) {
            Ok(game_state) => {
                match read_piece_data(&mut game_input) {
                    Ok(piece) => {
                        let optimal_move = strategy_engine.find_optimal_move(&game_state, &piece);
                        
                        // Ensure coordinates are within valid bounds
                        let safe_x = optimal_move.0.min(game_state.width.saturating_sub(1));
                        let safe_y = optimal_move.1.min(game_state.height.saturating_sub(1));
                        
                        println!("{} {}", safe_x, safe_y);
                        if io::stdout().flush().is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            Err(_) => break,
        }
    }
}
