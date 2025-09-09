use crate::{GameState, player1_strategy::Player1Strategy, utils::*};

pub struct AI;

impl AI {
    pub fn find_best_move(game_state: &GameState) -> (usize, usize) {
        let board = &game_state.grid;
        let piece = &game_state.piece;
        let territory = &game_state.territory;
        let player_number = game_state.player_num;
        
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);
        let mut valid_moves = Vec::new();
        
        // Get player characters
        let (our_chars, opponent_chars) = get_player_chars(player_number);
        let first_move = territory.is_empty();
        
        // Try all possible positions
        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if can_place_piece(game_state, x as i32, y as i32, &opponent_chars, &our_chars, first_move) {
                    let score = Player1Strategy::calculate_move_score(board, piece, x as i32, y as i32, territory);
                    
                    valid_moves.push((x, y, score));
                    
                    if score > best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
            }
        }
        
        // If no valid moves found, return first valid position or fallback
        if valid_moves.is_empty() {
            // Emergency fallback - find any valid position
            for y in 0..board.len() {
                for x in 0..board[0].len() {
                    if can_place_piece(game_state, x as i32, y as i32, &opponent_chars, &our_chars, first_move) {
                        return (x, y);
                    }
                }
            }
            (0, 0)
        } else {
            // Sort moves by score and pick the best
            valid_moves.sort_by(|a, b| b.2.cmp(&a.2));
            
            // Add some randomization among top moves to avoid predictability
            let top_moves = if valid_moves.len() >= 3 {
                &valid_moves[0..3.min(valid_moves.len())]
            } else {
                &valid_moves[..]
            };
            
            // For now, just pick the absolute best
            // In future, could add randomization: top_moves[0]
            best_move
        }
    }
}
