use crate::game::{Anfield, Piece, Player, GameState, Cell};

pub struct Strategy;

impl Strategy {
    pub fn new() -> Self {
        Self
    }

    /// Find the best position to place the piece using advanced strategy
    pub fn find_best_move(&self, game_state: &GameState, piece: &Piece, _player_id: u8) -> Option<(usize, usize)> {
        let valid_positions = self.get_valid_positions(&game_state.anfield, piece, game_state.get_current_player());
        
        if valid_positions.is_empty() {
            return None;
        }

        // Use evaluation to find best position with multiple criteria
        let evaluator = crate::ai::evaluation::Evaluator;
        let mut best_position = None;
        let mut best_score = i32::MIN;

        for (x, y) in valid_positions {
            let mut score = evaluator.evaluate_position(
                &game_state.anfield,
                piece,
                x,
                y,
                game_state.get_current_player(),
                game_state.get_opponent(),
            );

            // Add strategic bonuses
            score += self.evaluate_strategic_value(&game_state.anfield, piece, x, y, game_state.get_current_player().id);
            score += self.evaluate_defensive_value(&game_state.anfield, piece, x, y, game_state.get_opponent().id);
            
            if score > best_score {
                best_score = score;
                best_position = Some((x, y));
            }
        }

        best_position
    }

    /// Evaluate strategic value of a position (territory expansion, center control)
    fn evaluate_strategic_value(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, player_id: u8) -> i32 {
        let mut score = 0;
        let piece_cells = piece.get_absolute_cells(x, y);
        
        // Prefer positions that maximize future expansion opportunities
        for &(px, py) in &piece_cells {
            let adjacent_empty = self.count_adjacent_empty_cells(anfield, px, py);
            score += adjacent_empty * 3;
            
            // Bonus for center control
            let center_x = anfield.width / 2;
            let center_y = anfield.height / 2;
            let distance_to_center = ((px as i32 - center_x as i32).abs() + (py as i32 - center_y as i32).abs()) as i32;
            score += (10 - distance_to_center.min(10)) * 2;
        }
        
        score
    }

    /// Evaluate defensive value (blocking opponent expansion)
    fn evaluate_defensive_value(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, opponent_id: u8) -> i32 {
        let mut score = 0;
        let piece_cells = piece.get_absolute_cells(x, y);
        
        // Count how many opponent expansion opportunities we block
        for &(px, py) in &piece_cells {
            let adjacent_opponent = self.count_adjacent_opponent_cells(anfield, px, py, opponent_id);
            score += adjacent_opponent * 5; // High value for blocking opponent
        }
        
        score
    }

    /// Count adjacent empty cells for expansion potential
    fn count_adjacent_empty_cells(&self, anfield: &Anfield, x: usize, y: usize) -> i32 {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut count = 0;

        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if let Some(cell) = anfield.get_cell(new_x, new_y) {
                    if matches!(cell, crate::game::Cell::Empty) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Count adjacent opponent cells for blocking potential
    fn count_adjacent_opponent_cells(&self, anfield: &Anfield, x: usize, y: usize, opponent_id: u8) -> i32 {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut count = 0;

        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if let Some(cell) = anfield.get_cell(new_x, new_y) {
                    if cell.is_player(opponent_id) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Get all valid positions where the piece can be placed
    pub fn get_valid_positions(&self, anfield: &Anfield, piece: &Piece, player: &Player) -> Vec<(usize, usize)> {
        let mut valid_positions = Vec::new();

        // Try every possible position on the anfield
        for y in 0..anfield.height {
            for x in 0..anfield.width {
                if self.is_valid_placement(anfield, piece, x, y, player) {
                    valid_positions.push((x, y));
                }
            }
        }

        valid_positions
    }

    /// Find valid positions (alias for compatibility)
    pub fn find_valid_positions(&self, anfield: &Anfield, piece: &Piece, player_id: u8) -> Vec<(usize, usize)> {
        let player = Player::new(player_id);
        // Update player territory from anfield
        let mut player_with_territory = player;
        player_with_territory.territory = anfield.get_player_territory(player_id);
        self.get_valid_positions(anfield, piece, &player_with_territory)
    }

    /// Check if placing piece at (x, y) is valid according to game rules
    fn is_valid_placement(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, player: &Player) -> bool {
        let piece_cells = piece.get_absolute_cells(x, y);
        
        // Rule 1: All piece cells must be within anfield bounds
        for &(px, py) in &piece_cells {
            if !anfield.is_valid_position(px, py) {
                return false;
            }
        }

        // Rule 2: Check overlap requirements
        // - Exactly one cell must overlap with player's existing territory
        // - No cells can overlap with opponent's territory
        // - Other cells must be empty
        
        let mut player_overlap_count = 0;
        let opponent_id = if player.id == 1 { 2 } else { 1 };
        
        for &(px, py) in &piece_cells {
            if let Some(cell) = anfield.get_cell(px, py) {
                match *cell {
                    Cell::Player1Old | Cell::Player1New => {
                        if player.id == 1 {
                            player_overlap_count += 1;
                        } else {
                            // Cannot overlap opponent territory
                            return false;
                        }
                    }
                    Cell::Player2Old | Cell::Player2New => {
                        if player.id == 2 {
                            player_overlap_count += 1;
                        } else {
                            // Cannot overlap opponent territory
                            return false;
                        }
                    }
                    Cell::Empty => {
                        // Empty cells are OK
                    }
                }
            }
        }

        // Rule 3: Must have exactly one overlap with player territory
        // (For first move, if player has no territory, any position is valid)
        if player.territory.is_empty() {
            return true;
        }

        player_overlap_count == 1
    }

    /// Check if position is adjacent to player's territory
    fn is_adjacent_to_player_territory(&self, anfield: &Anfield, x: usize, y: usize, player_id: u8) -> bool {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                
                if let Some(cell) = anfield.get_cell(new_x, new_y) {
                    if cell.is_player(player_id) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if piece placement would overlap with opponent
    fn would_overlap_opponent(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, opponent_id: u8) -> bool {
        let piece_cells = piece.get_absolute_cells(x, y);
        
        for &(px, py) in &piece_cells {
            if let Some(cell) = anfield.get_cell(px, py) {
                if cell.is_player(opponent_id) {
                    return true;
                }
            }
        }
        false
    }

    /// Validate a specific move before making it
    pub fn validate_move(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, player: &Player, opponent: &Player) -> bool {
        // Check basic placement rules
        if !self.is_valid_placement(anfield, piece, x, y, player) {
            return false;
        }

        // Check no overlap with opponent
        if self.would_overlap_opponent(anfield, piece, x, y, opponent.id) {
            return false;
        }

        true
    }
}