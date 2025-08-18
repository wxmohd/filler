use std::env;
use std::fs;
use std::io::{self, Write, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
struct GameEngine {
    map_file: String,
    player1: String,
    player2: String,
    quiet: bool,
    timeout: u64,
    seed: Option<u64>,
}

#[derive(Debug, Clone)]
struct Anfield {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
    player1_start: (usize, usize),
    player2_start: (usize, usize),
}

#[derive(Debug, Clone)]
struct Piece {
    width: usize,
    height: usize,
    pattern: Vec<String>,
}

impl GameEngine {
    fn new() -> Self {
        Self {
            map_file: String::new(),
            player1: String::new(),
            player2: String::new(),
            quiet: false,
            timeout: 10,
            seed: None,
        }
    }

    fn parse_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        let mut i = 1;
        
        while i < args.len() {
            match args[i].as_str() {
                "-f" | "-file" => {
                    if i + 1 < args.len() {
                        self.map_file = args[i + 1].clone();
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "-p1" | "-player1" => {
                    if i + 1 < args.len() {
                        self.player1 = args[i + 1].clone();
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "-p2" | "-player2" => {
                    if i + 1 < args.len() {
                        self.player2 = args[i + 1].clone();
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "-q" | "-quiet" => {
                    self.quiet = true;
                    i += 1;
                }
                "-t" | "-time" => {
                    if i + 1 < args.len() {
                        self.timeout = args[i + 1].parse().unwrap_or(10);
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "-s" | "-seed" => {
                    if i + 1 < args.len() {
                        self.seed = args[i + 1].parse().ok();
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                _ => i += 1,
            }
        }
    }

    fn load_map(&self) -> io::Result<Anfield> {
        let content = fs::read_to_string(&self.map_file)?;
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid map format"));
        }

        let dimensions: Vec<usize> = lines[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();
            
        if dimensions.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid map dimensions"));
        }

        let height = dimensions[0];
        let width = dimensions[1];
        let mut grid = vec![vec!['.'; width]; height];
        let mut player1_start = (0, 0);
        let mut player2_start = (width - 1, height - 1);

        // If map has more lines, parse the initial layout
        if lines.len() > 1 {
            for (y, line) in lines[1..].iter().enumerate() {
                if y >= height { break; }
                for (x, ch) in line.chars().enumerate() {
                    if x >= width { break; }
                    grid[y][x] = ch;
                    if ch == '@' {
                        player1_start = (x, y);
                    } else if ch == '$' {
                        player2_start = (x, y);
                    }
                }
            }
        } else {
            // Default starting positions
            grid[player1_start.1][player1_start.0] = '@';
            grid[player2_start.1][player2_start.0] = '$';
        }

        Ok(Anfield {
            width,
            height,
            grid,
            player1_start,
            player2_start,
        })
    }

    fn generate_random_piece(&self) -> Piece {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(1..=5);
        let height = rng.gen_range(1..=4);
        
        let mut pattern = Vec::new();
        for _ in 0..height {
            let mut row = String::new();
            for _ in 0..width {
                if rng.gen_bool(0.6) {
                    row.push('O');
                } else {
                    row.push('.');
                }
            }
            pattern.push(row);
        }

        // Ensure at least one cell is filled
        if !pattern.iter().any(|row| row.contains('O')) {
            let mut first_row = pattern[0].chars().collect::<Vec<char>>();
            first_row[0] = 'O';
            pattern[0] = first_row.into_iter().collect();
        }

        Piece {
            width,
            height,
            pattern,
        }
    }

    fn send_game_state_to_player(&self, player_path: &str, player_id: u8, anfield: &Anfield, piece: &Piece) -> io::Result<(usize, usize)> {
        let mut child = Command::new(player_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(stdin) = child.stdin.as_mut() {
            // Send player info
            writeln!(stdin, "$$$ exec p{} : [{}]", player_id, player_path)?;
            
            // Send anfield
            writeln!(stdin, "Anfield {} {}:", anfield.width, anfield.height)?;
            for (y, row) in anfield.grid.iter().enumerate() {
                write!(stdin, "{:03} ", y)?;
                for &cell in row {
                    write!(stdin, "{}", cell)?;
                }
                writeln!(stdin)?;
            }
            
            // Send piece
            writeln!(stdin, "Piece {} {}:", piece.width, piece.height)?;
            for row in &piece.pattern {
                writeln!(stdin, "{}", row)?;
            }
        }

        // Read response
        let output = child.wait_with_output()?;
        let response = String::from_utf8_lossy(&output.stdout);
        let coords: Vec<&str> = response.trim().split_whitespace().collect();
        
        if coords.len() >= 2 {
            let x = coords[0].parse().unwrap_or(0);
            let y = coords[1].parse().unwrap_or(0);
            Ok((x, y))
        } else {
            Ok((0, 0))
        }
    }

    fn is_valid_move(&self, anfield: &Anfield, piece: &Piece, x: usize, y: usize, player_id: u8) -> bool {
        // Check bounds
        for (py, row) in piece.pattern.iter().enumerate() {
            for (px, ch) in row.chars().enumerate() {
                if ch == 'O' {
                    let abs_x = x + px;
                    let abs_y = y + py;
                    
                    if abs_x >= anfield.width || abs_y >= anfield.height {
                        return false;
                    }
                    
                    let cell = anfield.grid[abs_y][abs_x];
                    // Cannot overlap opponent
                    if (player_id == 1 && (cell == '$' || cell == 's')) ||
                       (player_id == 2 && (cell == '@' || cell == 'a')) {
                        return false;
                    }
                }
            }
        }

        // Check overlap requirement (must overlap exactly one cell with own territory)
        let mut overlap_count = 0;
        for (py, row) in piece.pattern.iter().enumerate() {
            for (px, ch) in row.chars().enumerate() {
                if ch == 'O' {
                    let abs_x = x + px;
                    let abs_y = y + py;
                    let cell = anfield.grid[abs_y][abs_x];
                    
                    if (player_id == 1 && (cell == '@' || cell == 'a')) ||
                       (player_id == 2 && (cell == '$' || cell == 's')) {
                        overlap_count += 1;
                    }
                }
            }
        }

        overlap_count == 1
    }

    fn place_piece(&self, anfield: &mut Anfield, piece: &Piece, x: usize, y: usize, player_id: u8) -> bool {
        if !self.is_valid_move(anfield, piece, x, y, player_id) {
            return false;
        }

        // Convert old pieces to old symbols
        for row in &mut anfield.grid {
            for cell in row {
                if *cell == 'a' { *cell = '@'; }
                if *cell == 's' { *cell = '$'; }
            }
        }

        // Place new piece
        let new_symbol = if player_id == 1 { 'a' } else { 's' };
        for (py, row) in piece.pattern.iter().enumerate() {
            for (px, ch) in row.chars().enumerate() {
                if ch == 'O' {
                    let abs_x = x + px;
                    let abs_y = y + py;
                    anfield.grid[abs_y][abs_x] = new_symbol;
                }
            }
        }

        true
    }

    fn run_game(&self) -> io::Result<()> {
        let mut anfield = self.load_map()?;
        let mut turn = 0;
        let max_turns = 1000;

        if !self.quiet {
            println!("Starting game: {} vs {}", self.player1, self.player2);
            println!("Map: {}", self.map_file);
        }

        while turn < max_turns {
            let current_player = if turn % 2 == 0 { 1 } else { 2 };
            let player_path = if current_player == 1 { &self.player1 } else { &self.player2 };
            
            let piece = self.generate_random_piece();
            
            if !self.quiet {
                println!("\n--- Turn {} (Player {}) ---", turn + 1, current_player);
            }

            match self.send_game_state_to_player(player_path, current_player, &anfield, &piece) {
                Ok((x, y)) => {
                    if self.place_piece(&mut anfield, &piece, x, y, current_player) {
                        if !self.quiet {
                            println!("Player {} placed piece at ({}, {})", current_player, x, y);
                        }
                    } else {
                        if !self.quiet {
                            println!("Player {} made invalid move at ({}, {})", current_player, x, y);
                        }
                        break;
                    }
                }
                Err(e) => {
                    if !self.quiet {
                        println!("Player {} error: {}", current_player, e);
                    }
                    break;
                }
            }

            turn += 1;
        }

        // Calculate final scores
        let mut p1_score = 0;
        let mut p2_score = 0;
        for row in &anfield.grid {
            for &cell in row {
                match cell {
                    '@' | 'a' => p1_score += 1,
                    '$' | 's' => p2_score += 1,
                    _ => {}
                }
            }
        }

        if !self.quiet {
            println!("\n=== GAME OVER ===");
            println!("Player 1 score: {}", p1_score);
            println!("Player 2 score: {}", p2_score);
            println!("Winner: Player {}", if p1_score > p2_score { 1 } else { 2 });
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut engine = GameEngine::new();
    engine.parse_args();
    
    if engine.map_file.is_empty() || engine.player1.is_empty() || engine.player2.is_empty() {
        eprintln!("Usage: game_engine -f <map> -p1 <player1> -p2 <player2>");
        std::process::exit(1);
    }

    engine.run_game()
}
