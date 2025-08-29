use std::io::{self, BufRead};
use filler::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let mut ai = create_ai(AIDifficulty::Medium);
    
    while let Some(line) = lines.next() {
        let line = line?;
        
        // Skip exec lines
        if line.starts_with("$$$") {
            continue;
        }
        
        // Parse Anfield
        if line.starts_with("Anfield") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let height: usize = parts[1].parse().unwrap_or(10);
                let width: usize = parts[2].trim_end_matches(':').parse().unwrap_or(15);
                
                let mut game = GameState::new(width, height);
                
                // Read board state
                for y in 0..height {
                    if let Some(board_line) = lines.next() {
                        let board_line = board_line?;
                        let chars: Vec<char> = board_line.chars().skip(4).collect(); // Skip "000 " prefix
                        
                        for (x, &ch) in chars.iter().enumerate() {
                            if x < width {
                                game.board[y][x] = match ch {
                                    '@' => Cell::Player1Old,
                                    'a' => Cell::Player1New,
                                    '$' => Cell::Player2Old,
                                    's' => Cell::Player2New,
                                    _ => Cell::Empty,
                                };
                            }
                        }
                    }
                }
                
                // Determine current player by looking for new pieces
                let mut current_player = 1;
                for row in &game.board {
                    for cell in row {
                        if matches!(cell, Cell::Player2New) {
                            current_player = 1; // If P2 has new pieces, it's P1's turn
                            break;
                        } else if matches!(cell, Cell::Player1New) {
                            current_player = 2; // If P1 has new pieces, it's P2's turn
                            break;
                        }
                    }
                }
                game.current_player = current_player;
                
                // Parse Piece
                if let Some(piece_line) = lines.next() {
                    let piece_line = piece_line?;
                    if piece_line.starts_with("Piece") {
                        let parts: Vec<&str> = piece_line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            let piece_height: usize = parts[1].parse().unwrap_or(1);
                            let piece_width: usize = parts[2].trim_end_matches(':').parse().unwrap_or(1);
                            
                            let mut shape = Vec::new();
                            
                            // Read piece shape
                            for y in 0..piece_height {
                                if let Some(shape_line) = lines.next() {
                                    let shape_line = shape_line?;
                                    for (x, ch) in shape_line.chars().enumerate() {
                                        if x < piece_width && (ch == '*' || ch == 'O') {
                                            shape.push((x, y));
                                        }
                                    }
                                }
                            }
                            
                            let piece = Piece::new(shape);
                            
                            // Get AI move
                            if let Some((x, y)) = ai.choose_move(&game, &piece) {
                                println!("{} {}", y, x); // Output in row column format
                            } else {
                                println!("0 0"); // Fallback move
                            }
                            
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}
