use filler_ai::{GameState, AI};

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
