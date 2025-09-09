use crate::game_state::GameState;
use crate::game_piece::GamePiece;

pub struct StrategyEngine {
    player_symbols: (char, char),
    enemy_symbols: (char, char),
}

impl StrategyEngine {
    pub fn new(player_id: u8) -> Self {
        let (player_symbols, enemy_symbols) = if player_id == 1 {
            (('@', 'a'), ('$', 's'))
        } else {
            (('$', 's'), ('@', 'a'))
        };
        
        StrategyEngine {
            player_symbols,
            enemy_symbols,
        }
    }
    
    pub fn find_optimal_move(&self, state: &GameState, piece: &GamePiece) -> (usize, usize) {
        let mut candidate_moves = Vec::new();
        
        // Generate all valid moves in deterministic order
        for row in 0..state.height {
            for col in 0..state.width {
                if self.is_placement_valid(state, piece, col, row) {
                    let move_score = self.evaluate_move_strength(state, piece, col, row);
                    candidate_moves.push((col, row, move_score));
                }
            }
        }
        
        if candidate_moves.is_empty() {
            return (0, 0);
        }
        
        // Completely deterministic sorting - score first, then position
        candidate_moves.sort_by(|a, b| {
            match b.2.cmp(&a.2) {
                std::cmp::Ordering::Equal => {
                    // For equal scores, prefer positions that maximize early game advantage
                    // Prefer top-left positions for deterministic behavior
                    match a.1.cmp(&b.1) {
                        std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                        other => other,
                    }
                }
                other => other,
            }
        });
        
        // Additional deterministic check - if multiple moves have the same top score,
        // prefer the one that maximizes territory control
        let best_score = candidate_moves[0].2;
        let best_moves: Vec<_> = candidate_moves.iter()
            .take_while(|&&(_, _, score)| score == best_score)
            .collect();
        
        if best_moves.len() > 1 {
            // Among equally good moves, prefer corner/edge positions
            for &(x, y, _) in &best_moves {
                if (*x == 0 || *x == state.width - 1) && (*y == 0 || *y == state.height - 1) {
                    return (*x, *y); // Corner position
                }
            }
            for &(x, y, _) in &best_moves {
                if *x == 0 || *x == state.width - 1 || *y == 0 || *y == state.height - 1 {
                    return (*x, *y); // Edge position
                }
            }
        }
        
        (candidate_moves[0].0, candidate_moves[0].1)
    }
    
    fn is_placement_valid(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> bool {
        let mut territory_overlaps = 0;
        let is_first_move = state.my_territory.is_empty();
        
        for (piece_row, row_data) in piece.shape.iter().enumerate() {
            for (piece_col, &cell) in row_data.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    let board_x = start_x + piece_col;
                    let board_y = start_y + piece_row;
                    
                    // Boundary check
                    if board_x >= state.width || board_y >= state.height {
                        return false;
                    }
                    
                    let board_cell = state.board[board_y][board_x];
                    
                    // Enemy collision check
                    if board_cell == self.enemy_symbols.0 || board_cell == self.enemy_symbols.1 {
                        return false;
                    }
                    
                    // Territory overlap counting
                    if board_cell == self.player_symbols.0 || board_cell == self.player_symbols.1 {
                        territory_overlaps += 1;
                    }
                }
            }
        }
        
        // Placement rules: first move = no overlap, others = exactly one overlap
        if is_first_move { territory_overlaps == 0 } else { territory_overlaps == 1 }
    }
    
    fn evaluate_move_strength(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> i32 {
        let mut total_score = 0;
        let mut cells_captured = 0;
        let mut enemy_adjacent_count = 0;
        
        // Deterministic game phase detection with map size awareness
        let total_cells = state.width * state.height;
        let occupied_cells = state.my_territory.len() + self.count_enemy_territory(state);
        let game_progress = occupied_cells as f32 / total_cells as f32;
        let is_large_map = total_cells > 2000; // 100x100 map has 10000 cells
        let is_early_game = if is_large_map { game_progress < 0.25 } else { game_progress < 0.35 };
        let is_very_early = if is_large_map { state.my_territory.len() < 25 } else { state.my_territory.len() < 10 };
        
        // TERMINATOR MODE: Ultra-extreme aggression multiplier for maximum dominance
        let terminator_mode = true; // Always assume facing strongest opponent
        let aggression_multiplier = if terminator_mode { 2.0 } else { 1.0 };
        
        for (piece_row, row_data) in piece.shape.iter().enumerate() {
            for (piece_col, &cell) in row_data.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    let board_x = start_x + piece_col;
                    let board_y = start_y + piece_row;
                    
                    if board_x < state.width && board_y < state.height {
                        let board_cell = state.board[board_y][board_x];
                        
                        if board_cell == '.' {
                            cells_captured += 1;
                            
                            // Ultra-dominant base expansion value - maximum aggression with large map scaling
                            let base_value = if is_very_early {
                                if is_large_map { 3000 } else { 2000 } // Extra aggression on large maps
                            } else if is_early_game {
                                if is_large_map { 2500 } else { 1500 } // Scale up for large maps
                            } else {
                                if is_large_map { 1200 } else { 800 }  // Maintain expansion on large maps
                            };
                            let terminator_base = (base_value as f32 * aggression_multiplier) as i32;
                            total_score += terminator_base;
                            
                            // Strategic positioning - corners and edges are critical (TERMINATOR MODE)
                            let position_bonus = self.compute_simple_position_bonus(board_x, board_y, state.width, state.height);
                            let position_multiplier = if is_very_early {
                                if terminator_mode { 10 } else { 5 } // Ultra-extreme corner priority vs terminator
                            } else if is_early_game {
                                if terminator_mode { 8 } else { 4 } // Maximum corner priority vs terminator
                            } else {
                                if terminator_mode { 4 } else { 2 } // Maintain corner focus vs terminator
                            };
                            total_score += position_bonus * position_multiplier;
                            
                            // Territory connection - maintain cohesion
                            let connection_bonus = self.compute_simple_connection_bonus(state, board_x, board_y);
                            let connection_multiplier = if is_early_game { 2 } else { 1 };
                            total_score += connection_bonus * connection_multiplier;
                            
                            // Enemy disruption - maximum blocking priority (TERMINATOR MODE)
                            let adjacent_enemies = self.count_adjacent_enemies(state, board_x, board_y);
                            if adjacent_enemies > 0 {
                                enemy_adjacent_count += adjacent_enemies;
                                let disruption_bonus = if is_very_early {
                                    if terminator_mode { 6000 } else { 3000 } // Ultra-extreme blocking vs terminator
                                } else if is_early_game {
                                    if terminator_mode { 5000 } else { 2500 } // Maximum blocking vs terminator
                                } else {
                                    if terminator_mode { 3000 } else { 1500 } // Maintain blocking vs terminator
                                };
                                total_score += adjacent_enemies * disruption_bonus;
                            }
                            
                            // Area control - dominate empty spaces (critical on large maps, TERMINATOR MODE)
                            let area_bonus = self.compute_simple_area_control(state, board_x, board_y);
                            let area_multiplier = if is_large_map {
                                if is_early_game { 
                                    if terminator_mode { 10 } else { 5 } // Ultra-high on large maps vs terminator
                                } else { 
                                    if terminator_mode { 8 } else { 4 }
                                }
                            } else {
                                if is_early_game { 
                                    if terminator_mode { 6 } else { 3 }
                                } else { 
                                    if terminator_mode { 4 } else { 2 }
                                }
                            };
                            total_score += area_bonus * area_multiplier;
                            
                            // Deterministic position tiebreaker - prefer top-left for consistency
                            total_score += (state.width - board_x) as i32 + (state.height - board_y) as i32;
                        }
                    }
                }
            }
        }
        
        // Ultra-massive capture bonus - exponential scaling for large moves (TERMINATOR MODE)
        if cells_captured >= 2 {
            let capture_multiplier = if is_very_early {
                if is_large_map { 
                    if terminator_mode { 2400 } else { 1200 } // Double bonus vs terminator
                } else { 
                    if terminator_mode { 1600 } else { 800 }
                }
            } else if is_early_game {
                if is_large_map { 
                    if terminator_mode { 1800 } else { 900 }
                } else { 
                    if terminator_mode { 1200 } else { 600 }
                }
            } else {
                if is_large_map { 
                    if terminator_mode { 1200 } else { 600 }
                } else { 
                    if terminator_mode { 800 } else { 400 }
                }
            };
            total_score += cells_captured * cells_captured * capture_multiplier;
        }
        
        // Ultra-extreme enemy blocking bonus (TERMINATOR MODE)
        if enemy_adjacent_count > 0 {
            let blocking_bonus = if is_very_early {
                if terminator_mode { 5000 } else { 2500 } // Ultra-maximum vs terminator
            } else if is_early_game {
                if terminator_mode { 4000 } else { 2000 } // Maximum vs terminator
            } else {
                if terminator_mode { 2400 } else { 1200 } // Strong vs terminator
            };
            total_score += enemy_adjacent_count * blocking_bonus;
        }

        // Territory advantage - ultra-severe penalties for falling behind (TERMINATOR MODE)
        let my_territory_size = state.my_territory.len();
        let enemy_territory_size = self.count_enemy_territory(state);
        if my_territory_size > enemy_territory_size {
            let advantage_bonus = if terminator_mode { 300 } else { 150 };
            total_score += (my_territory_size - enemy_territory_size) as i32 * advantage_bonus;
        } else if enemy_territory_size > my_territory_size {
            let penalty = if is_very_early {
                if terminator_mode { 800 } else { 400 } // Ultra-extreme penalty vs terminator
            } else if is_early_game {
                if terminator_mode { 600 } else { 300 } // Very high penalty vs terminator
            } else {
                if terminator_mode { 400 } else { 200 } // Strong penalty vs terminator
            };
            total_score -= (enemy_territory_size - my_territory_size) as i32 * penalty;
        }

        // Ultra-dominant corner control - absolutely essential for winning (TERMINATOR MODE)
        let territory_threshold = if is_large_map { 50 } else { 30 };
        if is_early_game && state.my_territory.len() < territory_threshold {
            let corner_multiplier = if is_very_early {
                if terminator_mode { 
                    if is_large_map { 16 } else { 12 } // Ultra-extreme corner priority vs terminator
                } else { 
                    if is_large_map { 8 } else { 6 }
                }
            } else {
                if terminator_mode { 
                    if is_large_map { 14 } else { 10 } // Maximum corner priority vs terminator
                } else { 
                    if is_large_map { 7 } else { 5 }
                }
            };
            total_score += self.compute_simple_corner_bonus(state, start_x, start_y) * corner_multiplier;
        }
        
        total_score
    }
    
    fn compute_position_bonus(&self, x: usize, y: usize, width: usize, height: usize) -> i32 {
        let mut bonus = 0;
        
        // Corner control - maximum strategic value
        if (x == 0 || x == width - 1) && (y == 0 || y == height - 1) {
            bonus += 1200;
        }
        // Edge control - high strategic value
        else if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
            bonus += 600;
            
            // Extra bonus for positions near corners on edges
            let corner_distance = std::cmp::min(
                std::cmp::min(x, width - 1 - x),
                std::cmp::min(y, height - 1 - y)
            );
            if corner_distance <= 2 {
                bonus += 200;
            }
        }
        // Strategic center control
        else {
            let center_x = width / 2;
            let center_y = height / 2;
            let distance_from_center = ((x as i32 - center_x as i32).abs() + (y as i32 - center_y as i32).abs()) as usize;
            
            // Enhanced center control for different board sizes
            let center_radius = std::cmp::max(width, height) / 6;
            if distance_from_center <= center_radius {
                bonus += 400 - (distance_from_center * 40) as i32;
            }
            
            // Bonus for controlling key strategic lines
            if x == center_x || y == center_y {
                bonus += 150;
            }
        }
        
        // Additional strategic positioning bonuses
        
        // Quadrant control bonus - reward controlling different board sections
        let quad_x = if x < width / 2 { 0 } else { 1 };
        let quad_y = if y < height / 2 { 0 } else { 1 };
        
        // Bonus for positions that can influence multiple quadrants
        if x == width / 2 || y == height / 2 {
            bonus += 100;
        }
        
        // Golden ratio positions (often strategically important)
        let golden_x = (width as f32 * 0.618) as usize;
        let golden_y = (height as f32 * 0.618) as usize;
        let golden_distance = ((x as i32 - golden_x as i32).abs() + (y as i32 - golden_y as i32).abs()) as usize;
        if golden_distance <= 1 {
            bonus += 200;
        }
        
        bonus
    }
    
    fn count_adjacent_enemies(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for &(dx, dy) in &adjacent_positions {
            let check_x = x as i32 + dx;
            let check_y = y as i32 + dy;
            
            if check_x >= 0 && check_y >= 0 && 
               check_x < state.width as i32 && check_y < state.height as i32 {
                let check_x = check_x as usize;
                let check_y = check_y as usize;
                let cell = state.board[check_y][check_x];
                
                if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    fn compute_disruption_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut disruption_score = 0;
        let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];
        
        for &(dx, dy) in &adjacent_positions {
            let check_x = x as i32 + dx;
            let check_y = y as i32 + dy;
            
            if check_x >= 0 && check_y >= 0 && 
               check_x < state.width as i32 && check_y < state.height as i32 {
                let check_x = check_x as usize;
                let check_y = check_y as usize;
                let cell = state.board[check_y][check_x];
                
                if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                    // Massive disruption bonus - prioritize blocking enemy
                    disruption_score += 1500;
                }
            }
        }
        
        disruption_score
    }
    
    fn compute_connection_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut connection_score = 0;
        let mut closest_distance = usize::MAX;
        let mut territory_density = 0;
        
        // Enhanced connection scoring with territory density consideration
        for &(territory_x, territory_y) in &state.my_territory {
            let distance = ((x as i32 - territory_x as i32).abs() + (y as i32 - territory_y as i32).abs()) as usize;
            closest_distance = std::cmp::min(closest_distance, distance);
            
            if distance <= 1 {
                connection_score += 500; // Immediate connection bonus
                territory_density += 3;
            } else if distance <= 2 {
                connection_score += 400 / (distance + 1) as i32;
                territory_density += 2;
            } else if distance <= 4 {
                connection_score += 200 / (distance + 1) as i32;
                territory_density += 1;
            } else if distance <= 6 {
                connection_score += 100 / (distance + 1) as i32;
            }
        }
        
        // Bonus for maintaining territory cohesion
        if territory_density >= 3 {
            connection_score += 300; // High density bonus
        } else if territory_density >= 2 {
            connection_score += 150; // Medium density bonus
        }
        
        // Penalty for moves too far from existing territory (avoid fragmentation)
        if closest_distance > 5 && !state.my_territory.is_empty() {
            connection_score -= 200;
        }
        
        connection_score
    }
    
    fn compute_area_control_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut area_bonus = 0;
        let radius = 3;
        
        // Count empty spaces in surrounding area
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 && 
                   check_x < state.width as i32 && check_y < state.height as i32 {
                    let check_x = check_x as usize;
                    let check_y = check_y as usize;
                    let cell = state.board[check_y][check_x];
                    
                    if cell == '.' {
                        let distance = (dx.abs() + dy.abs()) as usize;
                        area_bonus += 50 / (distance + 1) as i32; // Higher bonus for controlling empty areas
                    }
                }
            }
        }
        
        area_bonus
    }
    
    fn compute_blocking_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut blocking_score = 0;
        let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];
        let mut enemy_adjacency_count = 0;
        let mut strategic_blocks = 0;
        
        // Check if this position blocks enemy expansion paths
        for &(dx, dy) in &adjacent_positions {
            let check_x = x as i32 + dx;
            let check_y = y as i32 + dy;
            
            if check_x >= 0 && check_y >= 0 && 
               check_x < state.width as i32 && check_y < state.height as i32 {
                let check_x = check_x as usize;
                let check_y = check_y as usize;
                let cell = state.board[check_y][check_x];
                
                // Strategic blocking near enemy territory
                if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                    enemy_adjacency_count += 1;
                    
                    // Base blocking bonus
                    blocking_score += 600;
                    
                    // Enhanced bonus for blocking enemy expansion routes
                    let expansion_bonus = self.compute_expansion_blocking_bonus(state, check_x, check_y);
                    blocking_score += expansion_bonus;
                    
                    // Check if this is a strategic chokepoint
                    if self.is_strategic_chokepoint(state, x, y, check_x, check_y) {
                        strategic_blocks += 1;
                        blocking_score += 400;
                    }
                }
            }
        }
        
        // Exponential bonus for blocking multiple enemy positions
        if enemy_adjacency_count >= 2 {
            blocking_score += enemy_adjacency_count * enemy_adjacency_count * 200;
        }
        
        // Extra bonus for strategic chokepoints
        if strategic_blocks > 0 {
            blocking_score += strategic_blocks * 300;
        }
        
        // Bonus for blocking enemy access to corners/edges
        blocking_score += self.compute_territorial_denial_bonus(state, x, y);
        
        blocking_score
    }
    
    fn compute_expansion_blocking_bonus(&self, state: &GameState, enemy_x: usize, enemy_y: usize) -> i32 {
        let mut expansion_bonus = 0;
        let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        // Count how many empty spaces this enemy cell can expand to
        for &(dx, dy) in &adjacent_positions {
            let check_x = enemy_x as i32 + dx;
            let check_y = enemy_y as i32 + dy;
            
            if check_x >= 0 && check_y >= 0 && 
               check_x < state.width as i32 && check_y < state.height as i32 {
                let check_x = check_x as usize;
                let check_y = check_y as usize;
                let cell = state.board[check_y][check_x];
                
                if cell == '.' {
                    expansion_bonus += 300; // High bonus for blocking expansion routes
                }
            }
        }
        
        expansion_bonus
    }
    
    fn count_enemy_territory(&self, state: &GameState) -> usize {
        let mut count = 0;
        for row in &state.board {
            for &cell in row {
                if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                    count += 1;
                }
            }
        }
        count
    }
    
    fn compute_territory_efficiency_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut efficiency_bonus = 0;
        
        // Reward moves that create compact, connected territory
        let mut connected_sides = 0;
        let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for &(dx, dy) in &adjacent_positions {
            let check_x = x as i32 + dx;
            let check_y = y as i32 + dy;
            
            if check_x >= 0 && check_y >= 0 && 
               check_x < state.width as i32 && check_y < state.height as i32 {
                let check_x = check_x as usize;
                let check_y = check_y as usize;
                let cell = state.board[check_y][check_x];
                
                if cell == self.player_symbols.0 || cell == self.player_symbols.1 {
                    connected_sides += 1;
                }
            }
        }
        
        // Bonus for moves that connect to existing territory on multiple sides
        efficiency_bonus += connected_sides * 150;
        
        // Extra bonus for creating solid blocks
        if connected_sides >= 2 {
            efficiency_bonus += 200;
        }
        
        efficiency_bonus
    }
    
    fn compute_shape_efficiency_bonus(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> i32 {
        let mut shape_bonus = 0;
        let mut territory_connections = 0;
        let mut edge_connections = 0;
        
        // Analyze how the piece placement affects territory shape
        for (piece_row, row_data) in piece.shape.iter().enumerate() {
            for (piece_col, &cell) in row_data.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    let board_x = start_x + piece_col;
                    let board_y = start_y + piece_row;
                    
                    if board_x < state.width && board_y < state.height {
                        let board_cell = state.board[board_y][board_x];
                        
                        if board_cell == '.' {
                            // Check connections to existing territory
                            let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                            for &(dx, dy) in &adjacent_positions {
                                let check_x = board_x as i32 + dx;
                                let check_y = board_y as i32 + dy;
                                
                                if check_x >= 0 && check_y >= 0 && 
                                   check_x < state.width as i32 && check_y < state.height as i32 {
                                    let check_x = check_x as usize;
                                    let check_y = check_y as usize;
                                    let cell = state.board[check_y][check_x];
                                    
                                    if cell == self.player_symbols.0 || cell == self.player_symbols.1 {
                                        territory_connections += 1;
                                    }
                                    
                                    // Check for edge connections
                                    if check_x == 0 || check_x == state.width - 1 || 
                                       check_y == 0 || check_y == state.height - 1 {
                                        edge_connections += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Reward efficient territory shapes
        shape_bonus += territory_connections * 100;
        shape_bonus += edge_connections * 50;
        
        // Bonus for pieces that create strong territorial presence
        let piece_size = piece.get_active_cells().len();
        if piece_size >= 3 {
            shape_bonus += piece_size as i32 * 75;
        }
        
        shape_bonus
    }
    
    fn is_strategic_chokepoint(&self, state: &GameState, block_x: usize, block_y: usize, enemy_x: usize, enemy_y: usize) -> bool {
        // Check if blocking this position creates a strategic advantage
        let mut enemy_expansion_routes = 0;
        let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for &(dx, dy) in &adjacent_positions {
            let check_x = enemy_x as i32 + dx;
            let check_y = enemy_y as i32 + dy;
            
            if check_x >= 0 && check_y >= 0 && 
               check_x < state.width as i32 && check_y < state.height as i32 {
                let check_x = check_x as usize;
                let check_y = check_y as usize;
                
                if check_x != block_x || check_y != block_y {
                    let cell = state.board[check_y][check_x];
                    if cell == '.' {
                        enemy_expansion_routes += 1;
                    }
                }
            }
        }
        
        // It's a chokepoint if blocking reduces enemy expansion significantly
        enemy_expansion_routes <= 1
    }
    
    fn compute_territorial_denial_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut denial_bonus = 0;
        
        // Check if this position denies enemy access to strategic areas
        let strategic_areas = [
            (0, 0), (0, state.height - 1), 
            (state.width - 1, 0), (state.width - 1, state.height - 1)
        ];
        
        for &(corner_x, corner_y) in &strategic_areas {
            let distance_to_corner = ((x as i32 - corner_x as i32).abs() + (y as i32 - corner_y as i32).abs()) as usize;
            
            // Check if enemy is trying to reach this corner
            let mut enemy_near_corner = false;
            for enemy_x in 0..state.width {
                for enemy_y in 0..state.height {
                    let cell = state.board[enemy_y][enemy_x];
                    if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                        let enemy_distance = ((enemy_x as i32 - corner_x as i32).abs() + (enemy_y as i32 - corner_y as i32).abs()) as usize;
                        if enemy_distance <= 3 && enemy_distance < distance_to_corner {
                            enemy_near_corner = true;
                            break;
                        }
                    }
                }
                if enemy_near_corner { break; }
            }
            
            if enemy_near_corner && distance_to_corner <= 2 {
                denial_bonus += 400;
            }
        }
        
        // Bonus for denying access to edges
        if x <= 1 || x >= state.width - 2 || y <= 1 || y >= state.height - 2 {
            denial_bonus += 150;
        }
        
        denial_bonus
    }
    
    fn compute_dominance_bonus(&self, state: &GameState, x: usize, y: usize, is_early_game: bool) -> i32 {
        let mut dominance_bonus = 0;
        let radius = if is_early_game { 4 } else { 3 };
        
        // Count territory control in surrounding area
        let mut my_cells = 0;
        let mut enemy_cells = 0;
        let mut empty_cells = 0;
        
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 && 
                   check_x < state.width as i32 && check_y < state.height as i32 {
                    let check_x = check_x as usize;
                    let check_y = check_y as usize;
                    let cell = state.board[check_y][check_x];
                    
                    if cell == self.player_symbols.0 || cell == self.player_symbols.1 {
                        my_cells += 1;
                    } else if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                        enemy_cells += 1;
                    } else if cell == '.' {
                        empty_cells += 1;
                    }
                }
            }
        }
        
        // Bonus for dominating regions
        if my_cells > enemy_cells {
            dominance_bonus += (my_cells - enemy_cells) * 200;
        }
        
        // Bonus for controlling areas with many empty cells (expansion potential)
        if empty_cells > 5 {
            dominance_bonus += empty_cells * 100;
        }
        
        dominance_bonus
    }
    
    fn compute_corner_rush_bonus(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> i32 {
        let mut corner_bonus = 0;
        let corners = [(0, 0), (0, state.height - 1), (state.width - 1, 0), (state.width - 1, state.height - 1)];
        
        for &(corner_x, corner_y) in &corners {
            // Check if this move gets us closer to an uncontrolled corner
            let mut corner_controlled_by_enemy = false;
            let mut corner_controlled_by_me = false;
            
            // Check 3x3 area around corner
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let check_x = corner_x as i32 + dx;
                    let check_y = corner_y as i32 + dy;
                    
                    if check_x >= 0 && check_y >= 0 && 
                       check_x < state.width as i32 && check_y < state.height as i32 {
                        let check_x = check_x as usize;
                        let check_y = check_y as usize;
                        let cell = state.board[check_y][check_x];
                        
                        if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                            corner_controlled_by_enemy = true;
                        } else if cell == self.player_symbols.0 || cell == self.player_symbols.1 {
                            corner_controlled_by_me = true;
                        }
                    }
                }
            }
            
            // If corner is not controlled, bonus for getting closer
            if !corner_controlled_by_enemy && !corner_controlled_by_me {
                let distance_to_corner = ((start_x as i32 - corner_x as i32).abs() + (start_y as i32 - corner_y as i32).abs()) as usize;
                if distance_to_corner <= 5 {
                    corner_bonus += 1000 / (distance_to_corner + 1) as i32;
                }
            }
        }
        
        corner_bonus
    }
    
    fn compute_terminator_counter_bonus(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> i32 {
        let mut counter_bonus = 0;
        
        // Terminator-specific strategies
        let total_cells = state.width * state.height;
        let occupied_cells = state.my_territory.len() + self.count_enemy_territory(state);
        let game_progress = occupied_cells as f32 / total_cells as f32;
        
        // Anti-terminator strategy 1: Aggressive early corner control
        if game_progress < 0.2 {
            counter_bonus += self.compute_aggressive_corner_control(state, start_x, start_y);
        }
        
        // Anti-terminator strategy 2: Territory fragmentation prevention
        counter_bonus += self.compute_anti_fragmentation_bonus(state, piece, start_x, start_y);
        
        // Anti-terminator strategy 3: Chokepoint control
        counter_bonus += self.compute_chokepoint_control_bonus(state, start_x, start_y);
        
        // Anti-terminator strategy 4: Edge wall building
        if game_progress > 0.3 {
            counter_bonus += self.compute_edge_wall_bonus(state, start_x, start_y);
        }
        
        // Anti-terminator strategy 5: Territorial density maximization
        counter_bonus += self.compute_density_maximization_bonus(state, piece, start_x, start_y);
        
        counter_bonus
    }
    
    fn compute_lookahead_bonus(&self, state: &GameState, piece: &GamePiece, x: usize, y: usize) -> i32 {
        let mut lookahead_bonus = 0;
        
        // Simulate placing this piece and evaluate resulting position
        let future_territory_size = self.estimate_territory_after_move(state, piece, x, y);
        let current_territory_size = state.my_territory.len();
        
        if future_territory_size > current_territory_size {
            lookahead_bonus += (future_territory_size - current_territory_size) as i32 * 300;
        }
        
        // Evaluate enemy blocking potential after this move
        let enemy_blocking_potential = self.estimate_enemy_blocking_potential(state, x, y);
        lookahead_bonus -= enemy_blocking_potential;
        
        // Evaluate our future expansion potential
        let expansion_potential = self.estimate_expansion_potential(state, piece, x, y);
        lookahead_bonus += expansion_potential;
        
        lookahead_bonus
    }
    
    fn compute_aggressive_corner_control(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut corner_control_bonus = 0;
        let corners = [(0, 0), (0, state.height - 1), (state.width - 1, 0), (state.width - 1, state.height - 1)];
        
        for &(corner_x, corner_y) in &corners {
            let distance = ((x as i32 - corner_x as i32).abs() + (y as i32 - corner_y as i32).abs()) as usize;
            
            // Check if corner is contested
            let mut enemy_near_corner = false;
            for dy in -2..=2 {
                for dx in -2..=2 {
                    let check_x = corner_x as i32 + dx;
                    let check_y = corner_y as i32 + dy;
                    
                    if check_x >= 0 && check_y >= 0 && 
                       check_x < state.width as i32 && check_y < state.height as i32 {
                        let cell = state.board[check_y as usize][check_x as usize];
                        if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                            enemy_near_corner = true;
                            break;
                        }
                    }
                }
                if enemy_near_corner { break; }
            }
            
            if enemy_near_corner && distance <= 3 {
                corner_control_bonus += 2000 / (distance + 1) as i32; // Massive bonus for contested corners
            } else if distance <= 4 {
                corner_control_bonus += 800 / (distance + 1) as i32;
            }
        }
        
        corner_control_bonus
    }
    
    fn compute_anti_fragmentation_bonus(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> i32 {
        let mut anti_frag_bonus = 0;
        
        // Count connections to existing territory
        let mut territory_connections = 0;
        for (piece_row, row_data) in piece.shape.iter().enumerate() {
            for (piece_col, &cell) in row_data.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    let board_x = start_x + piece_col;
                    let board_y = start_y + piece_row;
                    
                    if board_x < state.width && board_y < state.height {
                        // Check 8-directional connections
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 { continue; }
                                
                                let check_x = board_x as i32 + dx;
                                let check_y = board_y as i32 + dy;
                                
                                if check_x >= 0 && check_y >= 0 && 
                                   check_x < state.width as i32 && check_y < state.height as i32 {
                                    let cell = state.board[check_y as usize][check_x as usize];
                                    if cell == self.player_symbols.0 || cell == self.player_symbols.1 {
                                        territory_connections += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Massive bonus for moves that create strong connections
        if territory_connections >= 3 {
            anti_frag_bonus += territory_connections * territory_connections * 200;
        }
        
        anti_frag_bonus
    }
    
    fn compute_chokepoint_control_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut chokepoint_bonus = 0;
        
        // Identify potential chokepoints (narrow passages between territories)
        let mut enemy_territories_nearby = 0;
        let mut empty_spaces_nearby = 0;
        
        for dy in -3..=3 {
            for dx in -3..=3 {
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 && 
                   check_x < state.width as i32 && check_y < state.height as i32 {
                    let cell = state.board[check_y as usize][check_x as usize];
                    
                    if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                        enemy_territories_nearby += 1;
                    } else if cell == '.' {
                        empty_spaces_nearby += 1;
                    }
                }
            }
        }
        
        // Bonus for controlling strategic chokepoints
        if enemy_territories_nearby > 3 && empty_spaces_nearby > 5 {
            chokepoint_bonus += 1500; // High value for chokepoint control
        }
        
        chokepoint_bonus
    }
    
    fn compute_edge_wall_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut wall_bonus = 0;
        
        // Bonus for building walls along edges to contain enemy
        if x == 0 || x == state.width - 1 || y == 0 || y == state.height - 1 {
            // Check if this creates a wall segment
            let mut wall_connections = 0;
            let adjacent_positions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            
            for &(dx, dy) in &adjacent_positions {
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 && 
                   check_x < state.width as i32 && check_y < state.height as i32 {
                    let cell = state.board[check_y as usize][check_x as usize];
                    if cell == self.player_symbols.0 || cell == self.player_symbols.1 {
                        wall_connections += 1;
                    }
                }
            }
            
            if wall_connections >= 1 {
                wall_bonus += wall_connections * 400;
            }
        }
        
        wall_bonus
    }
    
    fn compute_density_maximization_bonus(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> i32 {
        let mut density_bonus = 0;
        let piece_size = piece.get_active_cells().len();
        
        // Bonus for maximizing territorial density
        if piece_size >= 3 {
            density_bonus += piece_size as i32 * piece_size as i32 * 100;
        }
        
        // Extra bonus for pieces that fill gaps in our territory
        let mut gap_filling_score = 0;
        for (piece_row, row_data) in piece.shape.iter().enumerate() {
            for (piece_col, &cell) in row_data.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    let board_x = start_x + piece_col;
                    let board_y = start_y + piece_row;
                    
                    if board_x < state.width && board_y < state.height {
                        // Check if this fills a gap surrounded by our territory
                        let mut surrounding_territory = 0;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 { continue; }
                                
                                let check_x = board_x as i32 + dx;
                                let check_y = board_y as i32 + dy;
                                
                                if check_x >= 0 && check_y >= 0 && 
                                   check_x < state.width as i32 && check_y < state.height as i32 {
                                    let cell = state.board[check_y as usize][check_x as usize];
                                    if cell == self.player_symbols.0 || cell == self.player_symbols.1 {
                                        surrounding_territory += 1;
                                    }
                                }
                            }
                        }
                        
                        if surrounding_territory >= 5 {
                            gap_filling_score += 500; // High bonus for gap filling
                        }
                    }
                }
            }
        }
        
        density_bonus += gap_filling_score;
        density_bonus
    }
    
    fn estimate_territory_after_move(&self, state: &GameState, piece: &GamePiece, x: usize, y: usize) -> usize {
        let mut estimated_size = state.my_territory.len();
        
        for (piece_row, row_data) in piece.shape.iter().enumerate() {
            for (piece_col, &cell) in row_data.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    let board_x = x + piece_col;
                    let board_y = y + piece_row;
                    
                    if board_x < state.width && board_y < state.height {
                        let board_cell = state.board[board_y][board_x];
                        if board_cell == '.' {
                            estimated_size += 1;
                        }
                    }
                }
            }
        }
        
        estimated_size
    }
    
    fn estimate_enemy_blocking_potential(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut blocking_potential = 0;
        
        // Check how many of our expansion routes this move might expose
        for dy in -2..=2 {
            for dx in -2..=2 {
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 && 
                   check_x < state.width as i32 && check_y < state.height as i32 {
                    let cell = state.board[check_y as usize][check_x as usize];
                    
                    if cell == '.' {
                        // Check if enemy can easily reach this empty space
                        let mut enemy_distance = usize::MAX;
                        for enemy_x in 0..state.width {
                            for enemy_y in 0..state.height {
                                let enemy_cell = state.board[enemy_y][enemy_x];
                                if enemy_cell == self.enemy_symbols.0 || enemy_cell == self.enemy_symbols.1 {
                                    let distance = ((check_x as usize).abs_diff(enemy_x) + (check_y as usize).abs_diff(enemy_y));
                                    enemy_distance = enemy_distance.min(distance);
                                }
                            }
                        }
                        
                        if enemy_distance <= 3 {
                            blocking_potential += 100;
                        }
                    }
                }
            }
        }
        
        blocking_potential
    }
    
    fn estimate_expansion_potential(&self, state: &GameState, piece: &GamePiece, x: usize, y: usize) -> i32 {
        let mut expansion_potential = 0;
        
        // Count empty spaces we can reach after this move
        for (piece_row, row_data) in piece.shape.iter().enumerate() {
            for (piece_col, &cell) in row_data.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    let board_x = x + piece_col;
                    let board_y = y + piece_row;
                    
                    if board_x < state.width && board_y < state.height {
                        // Count adjacent empty spaces
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let check_x = board_x as i32 + dx;
                                let check_y = board_y as i32 + dy;
                                
                                if check_x >= 0 && check_y >= 0 && 
                                   check_x < state.width as i32 && check_y < state.height as i32 {
                                    let cell = state.board[check_y as usize][check_x as usize];
                                    if cell == '.' {
                                        expansion_potential += 50;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        expansion_potential
    }
    
    // Simplified helper functions for consistent performance against bender/wall_e
    fn compute_simple_position_bonus(&self, x: usize, y: usize, width: usize, height: usize) -> i32 {
        // Corners are most valuable
        if (x == 0 || x == width - 1) && (y == 0 || y == height - 1) {
            return 1000;
        }
        
        // Edges are valuable
        if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
            return 400;
        }
        
        // Center positions have moderate value
        let center_x = width / 2;
        let center_y = height / 2;
        let distance_from_center = ((x as i32 - center_x as i32).abs() + (y as i32 - center_y as i32).abs()) as usize;
        
        if distance_from_center <= 2 {
            return 200;
        }
        
        0
    }
    
    fn compute_simple_connection_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut connection_bonus = 0;
        
        // Reward proximity to existing territory
        for &(territory_x, territory_y) in &state.my_territory {
            let distance = ((x as i32 - territory_x as i32).abs() + (y as i32 - territory_y as i32).abs()) as usize;
            
            if distance == 1 {
                connection_bonus += 300; // Adjacent to territory
            } else if distance == 2 {
                connection_bonus += 150; // Close to territory
            } else if distance <= 4 {
                connection_bonus += 50; // Somewhat close
            }
        }
        
        connection_bonus
    }
    
    fn compute_simple_area_control(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let mut area_bonus = 0;
        let is_large_map = state.width * state.height > 2000;
        
        // Use larger search radius on large maps for better area control
        let search_radius = if is_large_map { 2 } else { 1 };
        
        // Count empty spaces in expanded area around this position
        for dy in -search_radius..=search_radius {
            for dx in -search_radius..=search_radius {
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 && 
                   check_x < state.width as i32 && check_y < state.height as i32 {
                    let check_x = check_x as usize;
                    let check_y = check_y as usize;
                    let cell = state.board[check_y][check_x];
                    
                    if cell == '.' {
                        // Distance-based bonus - closer empty cells are more valuable
                        let distance = dx.abs() + dy.abs();
                        let base_bonus = if is_large_map { 50 } else { 30 };
                        let distance_penalty = distance * 10;
                        area_bonus += (base_bonus - distance_penalty).max(10);
                    }
                }
            }
        }
        
        area_bonus
    }
    
    fn compute_simple_corner_bonus(&self, state: &GameState, x: usize, y: usize) -> i32 {
        let corners = [(0, 0), (0, state.height - 1), (state.width - 1, 0), (state.width - 1, state.height - 1)];
        let mut corner_bonus = 0;
        
        for &(corner_x, corner_y) in &corners {
            let distance = ((x as i32 - corner_x as i32).abs() + (y as i32 - corner_y as i32).abs()) as usize;
            
            // Check if corner is still available (not controlled by enemy)
            let mut corner_available = true;
            let mut enemy_near_corner = false;
            
            for dy in -3..=3 {
                for dx in -3..=3 {
                    let check_x = corner_x as i32 + dx;
                    let check_y = corner_y as i32 + dy;
                    
                    if check_x >= 0 && check_y >= 0 && 
                       check_x < state.width as i32 && check_y < state.height as i32 {
                        let cell = state.board[check_y as usize][check_x as usize];
                        if cell == self.enemy_symbols.0 || cell == self.enemy_symbols.1 {
                            if dx.abs() <= 1 && dy.abs() <= 1 {
                                corner_available = false; // Enemy controls corner
                            } else {
                                enemy_near_corner = true; // Enemy approaching corner
                            }
                        }
                    }
                }
                if !corner_available { break; }
            }
            
            if corner_available && distance <= 8 {
                // Ultra-high bonus for closer positions - guarantee corner control
                let base_bonus = match distance {
                    0 => 4000, // On corner - massive bonus
                    1 => 3000, // Adjacent to corner - huge bonus
                    2 => 2000, // Close to corner - very high bonus
                    3 => 1500, // Moderately close - high bonus
                    4 => 1000, // Somewhat close - good bonus
                    5 => 700,  // Far but valuable
                    6 => 500,  // Still valuable
                    7 => 300,  // Some value
                    _ => 200,  // Minimal value
                };
                
                // Massive urgency bonus if enemy is approaching this corner
                let urgency_bonus = if enemy_near_corner { base_bonus } else { 0 };
                
                corner_bonus += base_bonus + urgency_bonus;
            }
        }
        
        corner_bonus
    }
}
