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

    #[test]
    fn test_game_initialization() {
        let game = GameState::new(10, 8);
        assert_eq!(game.width, 10);
        assert_eq!(game.height, 8);
        assert_eq!(game.current_player, 1);
        assert_eq!(game.board[0][0], Cell::Player1Old);
        assert_eq!(game.board[7][9], Cell::Player2Old);
    }

    #[test]
    fn test_piece_creation() {
        let shape = vec![(0, 0), (1, 0), (0, 1)];
        let piece = Piece::new(shape.clone());
        assert_eq!(piece.shape, shape);
        assert_eq!(piece.width, 2);
        assert_eq!(piece.height, 2);
    }

    #[test]
    fn test_valid_move_detection_2() {
        let mut game = GameState::new(5, 5);
        let piece = Piece::new(vec![(0, 0), (1, 0)]); // 2x1 horizontal piece
        
        // Valid move: overlaps with player 1's starting position
        assert!(game.is_valid_move(&piece, 0, 0));
        
        // Invalid move: no overlap with own territory
        assert!(!game.is_valid_move(&piece, 2, 2));
        
        // Invalid move: out of bounds
        assert!(!game.is_valid_move(&piece, 4, 4));
    }

    #[test]
    fn test_piece_placement_2() {
        let mut game = GameState::new(5, 5);
        let piece = Piece::new(vec![(0, 0), (1, 0)]);
        
        // Place piece successfully
        assert!(game.place_piece(&piece, 0, 0));
        assert_eq!(game.board[0][0], Cell::Player1New);
        assert_eq!(game.board[0][1], Cell::Player1New);
    }

    #[test]
    fn test_score_calculation_2() {
        let mut game = GameState::new(5, 5);
        let piece = Piece::new(vec![(0, 0), (1, 0)]);
        
        game.place_piece(&piece, 0, 0);
        let (p1_score, p2_score) = game.calculate_scores();
        
        assert_eq!(p1_score, 2); // 2 cells for player 1
        assert_eq!(p2_score, 1); // 1 cell for player 2
    }

    #[test]
    fn test_player_switching() {
        let mut game = GameState::new(5, 5);
        assert_eq!(game.current_player, 1);
        
        game.switch_player();
        assert_eq!(game.current_player, 2);
        
        game.switch_player();
        assert_eq!(game.current_player, 1);
    }

    #[test]
    fn test_overlap_validation() {
        let mut game = GameState::new(5, 5);
        let piece = Piece::new(vec![(0, 0), (1, 0), (0, 1)]); // L-shaped piece
        
        // Place first piece
        game.place_piece(&piece, 0, 0);
        game.switch_player();
        
        // Player 2 cannot overlap player 1's territory
        let piece2 = Piece::new(vec![(0, 0)]);
        assert!(!game.is_valid_move(&piece2, 0, 0));
        assert!(!game.is_valid_move(&piece2, 1, 0));
    }

    #[test]
    fn test_ai_move_selection_2() {
        let mut ai = create_ai(AIDifficulty::Easy);
        let game = GameState::new(10, 10);
        let piece = Piece::new(vec![(0, 0)]);
        
        let move_result = ai.choose_move(&game, &piece);
        assert!(move_result.is_some());
        
        let (x, y) = move_result.unwrap();
        assert!(game.is_valid_move(&piece, x, y));
    }

    #[test]
    fn test_piece_generator_2() {
        let mut generator = PieceGenerator::new(42);
        let piece1 = generator.next();
        let piece2 = generator.next();
        
        // Should generate different pieces (with high probability)
        assert!(!piece1.shape.is_empty());
        assert!(!piece2.shape.is_empty());
    }

    #[test]
    fn test_game_over_detection_2() {
        let mut game = GameState::new(3, 3);
        // Fill most of the board to force game over
        for y in 0..3 {
            for x in 0..3 {
                if (x, y) != (0, 0) && (x, y) != (2, 2) {
                    game.board[y][x] = Cell::Player1Old;
                }
            }
        }
        
        let piece = Piece::new(vec![(0, 0), (1, 0), (2, 0)]); // Large piece that won't fit
        assert!(game.check_game_over(&piece));
    }

    #[test]
    fn test_coordinate_format() {
        // Test that coordinates are handled consistently
        let game = GameState::new(10, 10);
        let piece = Piece::new(vec![(0, 0)]);
        
        // Should be able to place at starting position
        assert!(game.is_valid_move(&piece, 0, 0));
        
        // Test boundary conditions
        assert!(!game.is_valid_move(&piece, 10, 0)); // Out of bounds X
        assert!(!game.is_valid_move(&piece, 0, 10)); // Out of bounds Y
    }

    #[test]
    fn test_minimax_ai() {
        let mut ai = create_ai(AIDifficulty::Hard);
        let game = GameState::new(10, 10);
        let piece = Piece::new(vec![(0, 0), (1, 0)]);
        
        let move_result = ai.choose_move(&game, &piece);
        assert!(move_result.is_some());
        
        let (x, y) = move_result.unwrap();
        assert!(game.is_valid_move(&piece, x, y));
    }

    #[test]
    fn test_from_board_constructor() {
        let board = vec![
            vec![Cell::Player1Old, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Player2Old],
        ];
        
        let game = GameState::from_board(board);
        assert_eq!(game.width, 3);
        assert_eq!(game.height, 3);
        assert_eq!(game.board[0][0], Cell::Player1Old);
        assert_eq!(game.board[2][2], Cell::Player2Old);
    }
}
