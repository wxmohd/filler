use crate::game::{Anfield, Piece, Player, Cell};

pub struct Evaluator;

impl Evaluator {
    /// Evaluate how good a position is for placing the piece
    /// Higher scores = better positions
    pub fn evaluate_position(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, player: &Player, opponent: &Player) -> i32 {
        let mut score = 0;

        // Base score: number of cells gained
        score += piece.count_cells() as i32 * 10;

        // Expansion potential: how many empty cells are adjacent
        score += self.evaluate_expansion_potential(anfield, piece, x, y) * 5;

        // Blocking potential: how much this move restricts opponent
        score += self.evaluate_blocking_potential(anfield, piece, x, y, opponent) * 3;

        // Center preference: slightly prefer positions closer to center
        let center_x = anfield.width / 2;
        let center_y = anfield.height / 2;
        let distance_from_center = ((x as i32 - center_x as i32).abs() + (y as i32 - center_y as i32).abs()) as i32;
        score -= distance_from_center;

        score
    }

    /// Calculate current territory score for a player
    pub fn calculate_territory_score(&self, anfield: &Anfield, player_id: u8) -> u32 {
        anfield.count_player_cells(player_id)
    }

    /// Evaluate how much this move blocks the opponent
    pub fn evaluate_blocking_potential(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, opponent: &Player) -> i32 {
        let piece_cells = piece.get_absolute_cells(x, y);
        let mut blocking_score = 0;

        for &(px, py) in &piece_cells {
            // Count how many opponent territory cells are adjacent
            let adjacent_opponent_cells = self.count_adjacent_opponent_cells(anfield, px, py, opponent.id);
            blocking_score += adjacent_opponent_cells * 2;
        }

        blocking_score
    }

    /// Evaluate how much this move helps future expansion
    pub fn evaluate_expansion_potential(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize) -> i32 {
        let piece_cells = piece.get_absolute_cells(x, y);
        let mut expansion_score = 0;

        for &(px, py) in &piece_cells {
            // Count adjacent empty cells
            expansion_score += self.count_adjacent_empty_cells(anfield, px, py);
        }

        expansion_score
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
                    if matches!(cell, Cell::Empty) {
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

    /// Evaluate territorial connectivity
    pub fn evaluate_connectivity(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, player_id: u8) -> i32 {
        let piece_cells = piece.get_absolute_cells(x, y);
        let mut connectivity_score = 0;

        for &(px, py) in &piece_cells {
            // Count how many player cells are adjacent
            let adjacent_player_cells = self.count_adjacent_player_cells(anfield, px, py, player_id);
            connectivity_score += adjacent_player_cells;
        }

        connectivity_score
    }

    /// Count adjacent player cells for connectivity
    fn count_adjacent_player_cells(&self, anfield: &Anfield, x: usize, y: usize, player_id: u8) -> i32 {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut count = 0;

        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if let Some(cell) = anfield.get_cell(new_x, new_y) {
                    if cell.is_player(player_id) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}