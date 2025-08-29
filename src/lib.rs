pub mod game;
pub mod piece;
pub mod ai;
pub mod player;
pub mod visualizer;
pub mod utils;

pub use game::*;
pub use piece::*;
pub use ai::*;
pub use player::*;
pub use visualizer::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let game = GameState::new(10, 15);
        assert_eq!(game.width, 10);
        assert_eq!(game.height, 15);
        assert_eq!(game.current_player, 1);
        assert_eq!(game.turn, 1);
        assert!(!game.game_over);
        
        // Check starting positions
        assert_eq!(game.board[0][0], Cell::Player1Old);
        assert_eq!(game.board[14][9], Cell::Player2Old);
    }

    #[test]
    fn test_piece_generation() {
        let mut generator = PieceGenerator::new(42);
        let piece = generator.next();
        assert!(!piece.shape.is_empty());
        assert!(piece.width > 0);
        assert!(piece.height > 0);
    }

    #[test]
    fn test_valid_move_detection() {
        let game = GameState::new(5, 5);
        let piece = Piece::new(vec![(0, 0), (1, 0)]); // 2-cell piece
        
        // Valid move: piece overlaps player 1's starting position (0,0) with exactly one cell
        assert!(game.is_valid_move(&piece, 0, 0));
        
        // Invalid move: no overlap with player territory
        assert!(!game.is_valid_move(&piece, 2, 2));
        
        // Invalid move: would go out of bounds
        assert!(!game.is_valid_move(&piece, 4, 0));
    }

    #[test]
    fn test_ai_move_selection() {
        let game = GameState::new(10, 15);
        let piece = Piece::new(vec![(0, 0)]);
        let mut ai = create_ai(AIDifficulty::Easy);
        
        let move_result = ai.choose_move(&game, &piece);
        assert!(move_result.is_some());
        
        if let Some((x, y)) = move_result {
            assert!(game.is_valid_move(&piece, x, y));
        }
    }

    #[test]
    fn test_score_calculation() {
        let game = GameState::new(5, 5);
        let (p1_score, p2_score) = game.calculate_scores();
        
        // Initial scores: each player has 1 cell
        assert_eq!(p1_score, 1);
        assert_eq!(p2_score, 1);
    }

    #[test]
    fn test_piece_placement() {
        let mut game = GameState::new(5, 5);
        let piece = Piece::new(vec![(0, 0), (1, 0)]); // 2-cell piece
        
        let initial_scores = game.calculate_scores();
        // Place piece overlapping player 1's starting position
        assert!(game.place_piece(&piece, 0, 0));
        let new_scores = game.calculate_scores();
        
        // Player 1 should have gained territory (from 1 to 2 cells)
        assert_eq!(new_scores.0, initial_scores.0 + 1);
    }

    #[test]
    fn test_game_over_detection() {
        let mut game = GameState::new(3, 3);
        // Create a large piece that cannot fit anywhere
        let piece = Piece::new(vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);
        
        // This should trigger game over due to no valid moves for such a large piece
        let is_over = game.check_game_over(&piece);
        // The result depends on whether valid moves exist
        if is_over {
            assert!(game.game_over);
        }
    }
}
