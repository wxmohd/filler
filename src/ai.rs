use crate::{GameState, Piece};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AIDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

pub trait AIPlayer {
    fn choose_move(&mut self, game: &GameState, piece: &Piece) -> Option<(usize, usize)>;
    fn get_name(&self) -> &str;
}

pub struct RandomAI {
    name: String,
}

impl RandomAI {
    pub fn new() -> Self {
        Self {
            name: "Random AI".to_string(),
        }
    }
}

impl AIPlayer for RandomAI {
    fn choose_move(&mut self, game: &GameState, piece: &Piece) -> Option<(usize, usize)> {
        let valid_moves = game.get_valid_moves(piece);
        if valid_moves.is_empty() {
            return None;
        }
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..valid_moves.len());
        Some(valid_moves[index])
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct GreedyAI {
    name: String,
}

impl GreedyAI {
    pub fn new() -> Self {
        Self {
            name: "Greedy AI".to_string(),
        }
    }

    fn evaluate_move(&self, game: &GameState, piece: &Piece, x: usize, y: usize) -> i32 {
        let mut score = 0;
        
        // Score based on piece size (larger pieces are better)
        score += piece.shape.len() as i32 * 10;
        
        // Score based on proximity to center
        let center_x = game.width / 2;
        let center_y = game.height / 2;
        let distance_to_center = ((x as i32 - center_x as i32).abs() + (y as i32 - center_y as i32).abs()) as i32;
        score -= distance_to_center;
        
        // Score based on blocking opponent
        let opponent_territory_nearby = self.count_opponent_cells_nearby(game, piece, x, y);
        score += opponent_territory_nearby * 5;
        
        score
    }

    fn count_opponent_cells_nearby(&self, game: &GameState, piece: &Piece, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let opponent_cells = if game.current_player == 1 {
            [crate::Cell::Player2Old, crate::Cell::Player2New]
        } else {
            [crate::Cell::Player1Old, crate::Cell::Player1New]
        };

        for (px, py) in &piece.shape {
            let abs_x = x + px;
            let abs_y = y + py;
            
            // Check surrounding cells
            for dy in -1..=1i32 {
                for dx in -1..=1i32 {
                    let check_x = abs_x as i32 + dx;
                    let check_y = abs_y as i32 + dy;
                    
                    if check_x >= 0 && check_y >= 0 && 
                       (check_x as usize) < game.width && 
                       (check_y as usize) < game.height {
                        let cell = &game.board[check_y as usize][check_x as usize];
                        if opponent_cells.contains(cell) {
                            count += 1;
                        }
                    }
                }
            }
        }
        
        count
    }
}

impl AIPlayer for GreedyAI {
    fn choose_move(&mut self, game: &GameState, piece: &Piece) -> Option<(usize, usize)> {
        let valid_moves = game.get_valid_moves(piece);
        if valid_moves.is_empty() {
            return None;
        }
        
        let mut best_move = valid_moves[0];
        let mut best_score = self.evaluate_move(game, piece, best_move.0, best_move.1);
        
        for &(x, y) in &valid_moves[1..] {
            let score = self.evaluate_move(game, piece, x, y);
            if score > best_score {
                best_score = score;
                best_move = (x, y);
            }
        }
        
        Some(best_move)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct MinimaxAI {
    name: String,
    depth: u32,
    transposition_table: HashMap<String, (i32, u32)>,
}

impl MinimaxAI {
    pub fn new(depth: u32) -> Self {
        Self {
            name: format!("Minimax AI (depth {})", depth),
            depth,
            transposition_table: HashMap::new(),
        }
    }

    fn evaluate_position(&self, game: &GameState) -> i32 {
        let (p1_score, p2_score) = game.calculate_scores();
        
        let score_diff = if game.current_player == 1 {
            p1_score as i32 - p2_score as i32
        } else {
            p2_score as i32 - p1_score as i32
        };
        
        // Add positional bonuses
        let mut position_bonus = 0;
        
        // Control of center is valuable
        let center_control = self.evaluate_center_control(game);
        position_bonus += center_control;
        
        // Territory compactness (connected regions are better)
        let compactness = self.evaluate_compactness(game);
        position_bonus += compactness;
        
        score_diff * 100 + position_bonus
    }

    fn evaluate_center_control(&self, game: &GameState) -> i32 {
        let center_x = game.width / 2;
        let center_y = game.height / 2;
        let mut control = 0;
        
        let player_cells = if game.current_player == 1 {
            [crate::Cell::Player1Old, crate::Cell::Player1New]
        } else {
            [crate::Cell::Player2Old, crate::Cell::Player2New]
        };
        
        // Check 3x3 area around center
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                let x = center_x as i32 + dx;
                let y = center_y as i32 + dy;
                
                if x >= 0 && y >= 0 && (x as usize) < game.width && (y as usize) < game.height {
                    let cell = &game.board[y as usize][x as usize];
                    if player_cells.contains(cell) {
                        control += 3 - dx.abs() - dy.abs(); // Closer to center = more points
                    }
                }
            }
        }
        
        control
    }

    fn evaluate_compactness(&self, game: &GameState) -> i32 {
        let player_cells = if game.current_player == 1 {
            [crate::Cell::Player1Old, crate::Cell::Player1New]
        } else {
            [crate::Cell::Player2Old, crate::Cell::Player2New]
        };
        
        let mut compactness = 0;
        
        for y in 0..game.height {
            for x in 0..game.width {
                if player_cells.contains(&game.board[y][x]) {
                    // Count adjacent friendly cells
                    let adjacent = self.count_adjacent_friendly(game, x, y, &player_cells);
                    compactness += adjacent;
                }
            }
        }
        
        compactness
    }

    fn count_adjacent_friendly(&self, game: &GameState, x: usize, y: usize, friendly_cells: &[crate::Cell; 2]) -> i32 {
        let mut count = 0;
        
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 { continue; }
                
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 && 
                   (check_x as usize) < game.width && 
                   (check_y as usize) < game.height {
                    let cell = &game.board[check_y as usize][check_x as usize];
                    if friendly_cells.contains(cell) {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    fn minimax(&mut self, game: &mut GameState, piece: &Piece, depth: u32, alpha: i32, beta: i32, maximizing: bool) -> i32 {
        if depth == 0 || game.game_over {
            return self.evaluate_position(game);
        }
        
        let valid_moves = game.get_valid_moves(piece);
        if valid_moves.is_empty() {
            return self.evaluate_position(game);
        }
        
        if maximizing {
            let mut max_eval = i32::MIN;
            for &(x, y) in &valid_moves {
                let mut game_copy = game.clone();
                game_copy.place_piece(piece, x, y);
                game_copy.switch_player();
                
                let eval = self.minimax(&mut game_copy, piece, depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                
                if beta <= alpha {
                    break; // Alpha-beta pruning
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &(x, y) in &valid_moves {
                let mut game_copy = game.clone();
                game_copy.place_piece(piece, x, y);
                game_copy.switch_player();
                
                let eval = self.minimax(&mut game_copy, piece, depth - 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                
                if beta <= alpha {
                    break; // Alpha-beta pruning
                }
            }
            min_eval
        }
    }
}

impl AIPlayer for MinimaxAI {
    fn choose_move(&mut self, game: &GameState, piece: &Piece) -> Option<(usize, usize)> {
        let valid_moves = game.get_valid_moves(piece);
        if valid_moves.is_empty() {
            return None;
        }
        
        if valid_moves.len() == 1 {
            return Some(valid_moves[0]);
        }
        
        let mut best_move = valid_moves[0];
        let mut best_score = i32::MIN;
        
        for &(x, y) in &valid_moves {
            let mut game_copy = game.clone();
            game_copy.place_piece(piece, x, y);
            game_copy.switch_player();
            
            let score = self.minimax(&mut game_copy, piece, self.depth - 1, i32::MIN, i32::MAX, false);
            
            if score > best_score {
                best_score = score;
                best_move = (x, y);
            }
        }
        
        Some(best_move)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub fn create_ai(difficulty: AIDifficulty) -> Box<dyn AIPlayer> {
    match difficulty {
        AIDifficulty::Easy => Box::new(RandomAI::new()),
        AIDifficulty::Medium => Box::new(GreedyAI::new()),
        AIDifficulty::Hard => Box::new(MinimaxAI::new(3)),
        AIDifficulty::Expert => Box::new(MinimaxAI::new(5)),
    }
}
