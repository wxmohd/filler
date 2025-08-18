use filler::game::{Anfield, Piece, Player, GameState};
use filler::game::parser::GameParser;
use filler::ai::{FillterAI, Strategy};
use std::io::{self, Write};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_placement_validation() {
        // Test basic piece placement validation
        let mut anfield = Anfield::new(5, 5);
        anfield.set_cell(2, 2, filler::game::anfield::Cell::Player1Old);
        
        let piece = Piece::from_pattern(vec!["OO".to_string()]);
        let strategy = Strategy::new();
        let player = Player::new(1);
        
        // Should find valid positions adjacent to player territory
        let valid_positions = strategy.get_valid_positions(&anfield, &piece, &player);
        assert!(!valid_positions.is_empty(), "Should find valid positions");
    }

    #[test]
    fn test_parser_functionality() {
        // Test parsing player info
        let player_line = "$$$ exec p1 : [robots/test]";
        let player_id = GameParser::parse_player_info(player_line);
        assert_eq!(player_id, Some(1));

        // Test parsing anfield header
        let anfield_line = "Anfield 5 5:";
        let (width, height) = GameParser::parse_anfield_header(anfield_line).unwrap();
        assert_eq!(width, 5);
        assert_eq!(height, 5);

        // Test parsing piece header
        let piece_line = "Piece 2 1:";
        let (width, height) = GameParser::parse_piece_header(piece_line).unwrap();
        assert_eq!(width, 2);
        assert_eq!(height, 1);
    }

    #[test]
    fn test_overlapping_rule() {
        // Test that pieces must overlap exactly one cell with player territory
        let mut anfield = Anfield::new(5, 5);
        anfield.set_cell(2, 2, filler::game::anfield::Cell::Player1Old);
        
        let piece = Piece::from_pattern(vec!["OO".to_string()]);
        let strategy = Strategy::new();
        let player = Player::new(1);
        
        // Check that valid positions require exactly one overlap
        let valid_positions = strategy.get_valid_positions(&anfield, &piece, &player);
        
        for (x, y) in valid_positions {
            let piece_positions = piece.get_absolute_cells(x, y);
            let mut overlap_count = 0;
            
            for (px, py) in piece_positions {
                if let Some(cell) = anfield.get_cell(px, py) {
                    if cell.is_player(1) {
                        overlap_count += 1;
                    }
                }
            }
            
            assert_eq!(overlap_count, 1, "Piece must overlap exactly one cell at position ({}, {})", x, y);
        }
    }

    #[test]
    fn test_ai_decision_making() {
        // Test that AI makes valid moves
        let mut game_state = GameState::new(5, 5);
        game_state.anfield.set_cell(0, 0, filler::game::anfield::Cell::Player1Old);
        game_state.anfield.set_cell(4, 4, filler::game::anfield::Cell::Player2Old);
        
        let piece = Piece::from_pattern(vec!["O".to_string()]);
        let strategy = Strategy::new();
        
        let best_move = strategy.find_best_move(&game_state, &piece, 1);
        assert!(best_move.is_some(), "AI should find a valid move");
    }
}
