use std::io::{self, BufRead, Write};
use super::{Anfield, Piece, Cell};

pub struct GameParser;

impl GameParser {
    /// Parse player info line: $$$ exec p1 : [robots/bender]
    pub fn parse_player_info(line: &str) -> Option<u8> {
        if line.starts_with("$$$ exec p") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(player_part) = parts.get(2) {
                if let Some(num_char) = player_part.chars().nth(1) {
                    return num_char.to_digit(10).map(|n| n as u8);
                }
            }
        }
        None
    }

    /// Parse anfield header: Anfield 20 15:
    pub fn parse_anfield_header(line: &str) -> Option<(usize, usize)> {
        if line.starts_with("Anfield ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let width = parts[1].parse().ok()?;
                let height = parts[2].trim_end_matches(':').parse().ok()?;
                return Some((width, height));
            }
        }
        None
    }

    /// Parse anfield row: 000 ....................
    pub fn parse_anfield_row(line: &str) -> Vec<Cell> {
        let mut cells = Vec::new();
        // Skip line number prefix (e.g., "000 ")
        let content = if line.len() > 4 && line.chars().nth(3) == Some(' ') {
            &line[4..]
        } else {
            line
        };

        for ch in content.chars() {
            cells.push(Cell::from_char(ch));
        }
        cells
    }

    /// Parse piece header: Piece 4 1:
    pub fn parse_piece_header(line: &str) -> Option<(usize, usize)> {
        if line.starts_with("Piece ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let width = parts[1].parse().ok()?;
                let height = parts[2].trim_end_matches(':').parse().ok()?;
                return Some((width, height));
            }
        }
        None
    }

    /// Parse complete anfield from input lines
    pub fn parse_anfield(lines: &[String]) -> Option<Anfield> {
        if lines.is_empty() {
            return None;
        }

        // Parse header
        let (width, height) = Self::parse_anfield_header(&lines[0])?;
        let mut anfield = Anfield::new(width, height);

        // Parse grid rows
        for (y, line) in lines[1..].iter().enumerate() {
            if y >= height {
                break;
            }
            let row_cells = Self::parse_anfield_row(line);
            for (x, cell) in row_cells.iter().enumerate() {
                if x < width {
                    anfield.set_cell(x, y, cell.clone());
                }
            }
        }

        Some(anfield)
    }

    /// Parse piece from input lines
    pub fn parse_piece(lines: &[String]) -> Option<Piece> {
        if lines.is_empty() {
            return None;
        }

        // Parse header
        let (width, height) = Self::parse_piece_header(&lines[0])?;
        
        // Parse pattern
        let mut pattern = Vec::new();
        for line in lines[1..].iter().take(height) {
            pattern.push(line.clone());
        }

        if pattern.len() == height {
            Some(Piece::from_pattern(pattern))
        } else {
            None
        }
    }

    /// Output move coordinates
    pub fn output_move(x: usize, y: usize) {
        println!("{} {}", x, y);
        io::stdout().flush().unwrap();
    }

    /// Read input from stdin line by line for real-time game engine interaction
    pub fn read_game_input() -> io::Result<(Option<u8>, Option<Anfield>, Option<Piece>)> {
        let stdin = io::stdin();
        let mut lines = Vec::new();
        
        // Read input line by line until we have a complete game state
        for line in stdin.lock().lines() {
            let line = line?;
            lines.push(line);
            
            // Check if we have all components needed
            let has_player = lines.iter().any(|l| l.starts_with("$$$ exec p"));
            let has_anfield = lines.iter().any(|l| l.starts_with("Anfield "));
            let has_piece = lines.iter().any(|l| l.starts_with("Piece "));
            
            // If we have all components, process them
            if has_player && has_anfield && has_piece {
                break;
            }
        }

        let mut player_id = None;
        let mut anfield = None;
        let mut piece = None;
        
        let mut i = 0;
        while i < lines.len() {
            let line = &lines[i];
            
            // Parse player info
            if line.starts_with("$$$ exec p") {
                player_id = Self::parse_player_info(line);
                i += 1;
            }
            // Parse anfield
            else if line.starts_with("Anfield ") {
                // Collect anfield lines
                let mut anfield_lines = vec![line.clone()];
                i += 1;
                
                // Read grid rows
                while i < lines.len() && !lines[i].starts_with("Piece ") {
                    anfield_lines.push(lines[i].clone());
                    i += 1;
                }
                
                anfield = Self::parse_anfield(&anfield_lines);
            }
            // Parse piece
            else if line.starts_with("Piece ") {
                // Get piece dimensions from header
                let (piece_width, piece_height) = Self::parse_piece_header(line).unwrap_or((0, 0));
                
                // Collect piece lines
                let mut piece_lines = vec![line.clone()];
                i += 1;
                
                // Read exactly piece_height lines for the piece pattern
                for _ in 0..piece_height {
                    if i < lines.len() {
                        piece_lines.push(lines[i].clone());
                        i += 1;
                    }
                }
                
                piece = Self::parse_piece(&piece_lines);
                break; // We have everything we need
            }
            else {
                i += 1;
            }
        }

        Ok((player_id, anfield, piece))
    }
}