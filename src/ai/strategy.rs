use crate::game::{Anfield, Piece, Player, GameState, Cell};

pub struct Strategy;

impl Strategy {
    pub fn new() -> Self {
        Self
    }

    /// Find the best position to place the piece
    pub fn find_best_move(&self, game_state: &GameState, piece: &Piece, player_id: u8) -> Option<(usize, usize)> {
        let valid_positions = self.get_valid_positions(&game_state.anfield, piece, game_state.get_current_player());
        
        if valid_positions.is_empty() {
            return None;
        }

        // Simple greedy strategy: pick first valid position
        // TODO: Implement better evaluation in future steps
        valid_positions.into_iter().next()
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

    /// Check if placing piece at (x, y) is valid according to game rules
    fn is_valid_placement(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, player: &Player) -> bool {
        let piece_cells = piece.get_absolute_cells(x, y);
        
        // Rule 1: All piece cells must be within anfield bounds
        for &(px, py) in &piece_cells {
            if !anfield.is_valid_position(px, py) {
                return false;
            }
        }

        // Rule 2: All piece cells must be empty (no overlap with existing pieces)
        for &(px, py) in &piece_cells {
            if let Some(cell) = anfield.get_cell(px, py) {
                if !matches!(cell, Cell::Empty) {
                    return false;
                }
            }
        }

        // Rule 3: Exactly one cell must be adjacent to player's existing territory
        // (For first move, if player has no territory, any position is valid)
        if player.territory.is_empty() {
            return true;
        }

        let mut adjacent_count = 0;
        for &(px, py) in &piece_cells {
            if self.is_adjacent_to_player_territory(anfield, px, py, player.id) {
                adjacent_count += 1;
            }
        }

        // Must have exactly one adjacent cell
        adjacent_count == 1
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