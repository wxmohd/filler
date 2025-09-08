use crate::{GameState, utils::*};

pub struct Player1Strategy;

impl Player1Strategy {
    pub fn calculate_move_score(
        board: &[Vec<char>],
        piece: &[Vec<char>],
        x: i32,
        y: i32,
        territory: &[(usize, usize)],
    ) -> i32 {
        let mut score = 0;
        let width = board[0].len();
        let height = board.len();
        
        let our_chars = ['@', 'a'];
        let opponent_chars = ['$', 's'];
        
        // Count current territories
        let our_territory_count = territory.len();
        let mut opponent_territory_count = 0;
        
        for row in board {
            for &cell in row {
                if opponent_chars.contains(&cell) {
                    opponent_territory_count += 1;
                }
            }
        }
        
        // Game phase detection
        let total_cells = width * height;
        let occupied_cells = our_territory_count + opponent_territory_count;
        let game_progress = occupied_cells as f32 / total_cells as f32;
        
        let is_early_game = game_progress < 0.3;
        let is_mid_game = game_progress >= 0.3 && game_progress < 0.7;
        let is_late_game = game_progress >= 0.7;
        
        // Calculate piece effects
        let mut new_territory_count = 0;
        let mut adjacent_opponent_count = 0;
        let mut adjacent_empty_count = 0;
        let mut edge_control_count = 0;
        let mut corner_control_count = 0;
        
        for py in 0..piece.len() {
            for px in 0..piece[py].len() {
                if piece[py][px] == '*' {
                    let bx = x + px as i32;
                    let by = y + py as i32;
                    
                    if bx >= 0 && by >= 0 && (by as usize) < height && (bx as usize) < width {
                        let bx = bx as usize;
                        let by = by as usize;
                        
                        // Count new territory
                        if board[by][bx] == '.' {
                            new_territory_count += 1;
                        }
                        
                        // Analyze adjacent cells
                        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                        for &(dx, dy) in &directions {
                            let nx = bx as i32 + dx;
                            let ny = by as i32 + dy;
                            
                            if nx >= 0 && ny >= 0 && (ny as usize) < height && (nx as usize) < width {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                
                                if opponent_chars.contains(&board[ny][nx]) {
                                    adjacent_opponent_count += 1;
                                } else if board[ny][nx] == '.' {
                                    adjacent_empty_count += 1;
                                }
                            }
                        }
                        
                        // Strategic position values
                        if (bx == 0 || bx == width - 1) && (by == 0 || by == height - 1) {
                            corner_control_count += 1;
                        } else if bx == 0 || bx == width - 1 || by == 0 || by == height - 1 {
                            edge_control_count += 1;
                        }
                    }
                }
            }
        }
        
        // Phase-specific scoring
        if is_early_game {
            // Early game: Prioritize expansion and position
            score += new_territory_count * 1000;
            score += corner_control_count * 2000;
            score += edge_control_count * 800;
            score += adjacent_empty_count * 300; // Future expansion potential
            score += adjacent_opponent_count * 500; // Some blocking
            
        } else if is_mid_game {
            // Mid game: Balance expansion with blocking
            score += new_territory_count * 800;
            score += adjacent_opponent_count * 1200; // Increased blocking importance
            score += adjacent_empty_count * 200;
            score += edge_control_count * 400;
            
        } else if is_late_game {
            // Late game: Focus on blocking and efficient moves
            score += adjacent_opponent_count * 2000; // Maximum blocking priority
            score += new_territory_count * 600;
            score += adjacent_empty_count * 100;
        }
        
        // Territory ratio bonuses
        if our_territory_count > opponent_territory_count {
            // We're winning - maintain lead
            score += 500;
            score += adjacent_opponent_count * 300; // Block to maintain lead
        } else {
            // We're behind - be more aggressive
            score += new_territory_count * 200; // Extra expansion bonus
            score += adjacent_opponent_count * 800; // Aggressive blocking
        }
        
        // Connection to existing territory
        let mut best_connection_distance = i32::MAX;
        for &(tx, ty) in territory {
            let distance = (x - tx as i32).abs() + (y - ty as i32).abs();
            best_connection_distance = best_connection_distance.min(distance);
        }
        
        if !territory.is_empty() {
            // Prefer moves closer to existing territory
            score += (10 - best_connection_distance.min(10)) * 100;
        }
        
        // Avoid moves that create isolated pockets
        if territory.len() > 5 && best_connection_distance > 4 {
            score -= 1000; // Penalty for isolation
        }
        
        // Map-specific adjustments
        if width > 40 || height > 40 {
            // Large maps: prioritize rapid expansion
            score += new_territory_count * 200;
            score += adjacent_empty_count * 150;
        } else {
            // Small maps: prioritize blocking and efficiency
            score += adjacent_opponent_count * 300;
        }
        
        // Piece efficiency - prefer moves that use more of the piece
        let piece_size = piece.iter().flatten().filter(|&&c| c == '*').count();
        score += (piece_size * 50) as i32;
        
        score
    }
}
