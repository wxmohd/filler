use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct GameState {
    pub grid: Vec<Vec<char>>,
    pub piece: Vec<Vec<char>>,
    pub player_num: u8,
    pub width: usize,
    pub height: usize,
    pub territory: Vec<(usize, usize)>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            grid: Vec::new(),
            piece: Vec::new(),
            player_num: 0,
            width: 0,
            height: 0,
            territory: Vec::new(),
        }
    }

    pub fn read_player_assignment(&mut self) -> io::Result<()> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        self.player_num = if input.contains("p1") { 1 } else { 2 };
        Ok(())
    }

    pub fn read_game_state(&mut self) -> io::Result<bool> {
        let mut input = String::new();
        
        // Read anfield header
        input.clear();
        if io::stdin().read_line(&mut input).is_err() { return Ok(false); }
        if !input.trim().starts_with("Anfield") { return Ok(false); }
        
        // Parse dimensions
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        self.width = parts[1].parse().unwrap_or(40);
        self.height = parts[2].trim_end_matches(':').parse().unwrap_or(30);
        
        // Skip coordinate header
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        // Read grid
        self.grid.clear();
        for _ in 0..self.height {
            input.clear();
            io::stdin().read_line(&mut input)?;
            let line = input.trim();
            if let Some(start) = line.find(|c: char| ".$@as".contains(c)) {
                let row: Vec<char> = line[start..].chars().take(self.width).collect();
                self.grid.push(row);
            }
        }
        
        // Read piece header
        input.clear();
        if io::stdin().read_line(&mut input).is_err() { return Ok(false); }
        if !input.trim().starts_with("Piece") { return Ok(false); }
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let piece_width: usize = parts[1].parse().unwrap_or(1);
        let piece_height: usize = parts[2].trim_end_matches(':').parse().unwrap_or(1);
        
        // Read piece
        self.piece.clear();
        for _ in 0..piece_height {
            input.clear();
            io::stdin().read_line(&mut input)?;
            let row: Vec<char> = input.trim().chars().take(piece_width).collect();
            self.piece.push(row);
        }
        
        // Update territory
        self.update_territory();
        
        Ok(true)
    }

    pub fn update_territory(&mut self) {
        let our_symbols = if self.player_num == 1 { ['@', 'a'] } else { ['$', 's'] };
        self.territory.clear();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if our_symbols.contains(&self.grid[y][x]) {
                    self.territory.push((x, y));
                }
            }
        }
    }

    pub fn output_move(&self, x: usize, y: usize) -> io::Result<()> {
        println!("{} {}", x, y);
        io::stdout().flush()
    }
}
