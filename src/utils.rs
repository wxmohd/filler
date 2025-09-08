use crate::GameState;

pub fn can_place_piece(
    game_state: &GameState,
    x: i32,
    y: i32,
    opponent_chars: &[char],
    our_chars: &[char],
    first_move: bool,
) -> bool {
    let mut overlap_count = 0;
    
    for py in 0..game_state.piece.len() {
        for px in 0..game_state.piece[py].len() {
            if game_state.piece[py][px] == '*' || game_state.piece[py][px] == '#' {
                let bx = x + px as i32;
                let by = y + py as i32;
                
                // Check bounds
                if bx < 0 || by < 0 || by >= game_state.grid.len() as i32 || bx >= game_state.grid[0].len() as i32 {
                    return false;
                }
                
                let bx = bx as usize;
                let by = by as usize;
                
                // Check opponent overlap
                if opponent_chars.contains(&game_state.grid[by][bx]) {
                    return false;
                }
                
                // Count our territory overlaps
                if our_chars.contains(&game_state.grid[by][bx]) {
                    overlap_count += 1;
                }
            }
        }
    }
    
    // First move: no overlap required, subsequent moves: exactly one overlap
    if first_move { overlap_count == 0 } else { overlap_count == 1 }
}

pub fn get_player_chars(player_num: u8) -> ([char; 2], [char; 2]) {
    if player_num == 1 {
        (['@', 'a'], ['$', 's'])
    } else {
        (['$', 's'], ['@', 'a'])
    }
}

pub fn count_adjacent_cells(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    target_chars: &[char],
) -> i32 {
    let mut count = 0;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];
    
    for &(dx, dy) in &directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        
        if nx >= 0 && ny >= 0 && ny < grid.len() as i32 && nx < grid[0].len() as i32 {
            let nx = nx as usize;
            let ny = ny as usize;
            
            if target_chars.contains(&grid[ny][nx]) {
                count += 1;
            }
        }
    }
    
    count
}
