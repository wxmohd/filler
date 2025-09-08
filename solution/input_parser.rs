use std::io;
use crate::game_state::GameState;
use crate::game_piece::GamePiece;

pub fn read_game_state(input_buffer: &mut String, player_id: u8) -> Result<GameState, ()> {
    input_buffer.clear();
    if io::stdin().read_line(input_buffer).is_err() { return Err(()); }
    
    // Parse board header - more robust parsing
    let header_line = input_buffer.trim();
    if !header_line.starts_with("Anfield") { return Err(()); }
    
    let dimensions: Vec<&str> = header_line.split_whitespace().collect();
    if dimensions.len() < 3 { return Err(()); }
    
    let width: usize = dimensions[1].parse().map_err(|_| ())?;
    let height: usize = dimensions[2].trim_end_matches(':').parse().map_err(|_| ())?;
    
    // Skip coordinate line
    input_buffer.clear();
    if io::stdin().read_line(input_buffer).is_err() { return Err(()); }
    
    // Parse board data with better error handling
    let mut board = Vec::new();
    for _ in 0..height {
        input_buffer.clear();
        if io::stdin().read_line(input_buffer).is_err() { return Err(()); }
        let line = input_buffer.trim();
        
        // Find the start of board data more reliably
        if let Some(board_start) = line.find(|c: char| ".$@as".contains(c)) {
            let board_line = &line[board_start..];
            let mut row = Vec::new();
            
            // Ensure we get exactly the right width
            for (i, ch) in board_line.chars().enumerate() {
                if i >= width { break; }
                if ".$@as".contains(ch) {
                    row.push(ch);
                } else {
                    row.push('.');  // Default to empty space for invalid chars
                }
            }
            
            // Pad row if necessary
            while row.len() < width {
                row.push('.');
            }
            
            board.push(row);
        } else {
            // If no valid board data found, create empty row
            board.push(vec!['.'; width]);
        }
    }
    
    // Ensure we have the right number of rows
    while board.len() < height {
        board.push(vec!['.'; width]);
    }
    
    Ok(GameState::new(board, width, height, player_id))
}

pub fn read_piece_data(input_buffer: &mut String) -> Result<GamePiece, ()> {
    input_buffer.clear();
    if io::stdin().read_line(input_buffer).is_err() { return Err(()); }
    
    let piece_line = input_buffer.trim();
    if !piece_line.starts_with("Piece") { return Err(()); }
    
    let piece_info: Vec<&str> = piece_line.split_whitespace().collect();
    if piece_info.len() < 3 { return Err(()); }
    
    let width: usize = piece_info[1].parse().map_err(|_| ())?;
    let height: usize = piece_info[2].trim_end_matches(':').parse().map_err(|_| ())?;
    
    let mut shape = Vec::new();
    for _ in 0..height {
        input_buffer.clear();
        if io::stdin().read_line(input_buffer).is_err() { return Err(()); }
        let line = input_buffer.trim();
        
        let mut row = Vec::new();
        for (i, ch) in line.chars().enumerate() {
            if i >= width { break; }
            if ".O#*".contains(ch) {
                row.push(ch);
            } else {
                row.push('.');  // Default for invalid chars
            }
        }
        
        // Pad row if necessary
        while row.len() < width {
            row.push('.');
        }
        
        shape.push(row);
    }
    
    // Ensure we have the right number of rows
    while shape.len() < height {
        shape.push(vec!['.'; width]);
    }
    
    Ok(GamePiece::new(shape, width, height))
}
