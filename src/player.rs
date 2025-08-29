use std::io::{self, Write};
use crate::{GameState, Piece};

pub trait Player {
    fn get_move(&mut self, game: &GameState, piece: &Piece) -> Option<(usize, usize)>;
    fn get_name(&self) -> &str;
    fn is_human(&self) -> bool;
}

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    fn display_game_info(&self, game: &GameState, piece: &Piece) {
        println!("\n{}", "=".repeat(60));
        println!("Turn {} - {} ({}) to move", 
                 game.turn, 
                 self.name, 
                 if game.current_player == 1 { "@" } else { "$" });
        
        let (p1_score, p2_score) = game.calculate_scores();
        println!("Scores: Player 1: {} | Player 2: {}", p1_score, p2_score);
        println!("{}", "=".repeat(60));
        
        // Display board
        println!("\nCurrent Board:");
        print!("{}", game.display_board());
        
        // Display piece
        println!("Your piece to place:");
        print!("{}", piece.display());
        
        // Display valid moves count
        let valid_moves = game.get_valid_moves(piece);
        println!("Valid moves available: {}", valid_moves.len());
        
        if valid_moves.len() <= 10 {
            println!("Valid positions: {:?}", valid_moves);
        }
        
        println!("\nRules reminder:");
        println!("• Place piece so EXACTLY ONE cell overlaps your territory");
        println!("• Cannot overlap opponent territory");
        println!("• Piece must fit within board boundaries");
        println!("• Enter coordinates as: row column (e.g., '5 3')");
    }

    fn get_user_input(&self) -> io::Result<String> {
        print!("\nEnter move (row column) or 'help' for assistance: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn show_help(&self, game: &GameState, piece: &Piece) {
        println!("\n--- HELP ---");
        println!("Commands:");
        println!("• <row> <column> - Place piece at position (e.g., '5 3')");
        println!("• 'help' - Show this help");
        println!("• 'hint' - Show a suggested move");
        println!("• 'valid' - Show all valid moves");
        println!("• 'scores' - Show current scores");
        println!("• 'quit' - Quit the game");
        
        let valid_moves = game.get_valid_moves(piece);
        if !valid_moves.is_empty() {
            println!("\nExample valid move: {} {}", valid_moves[0].1, valid_moves[0].0);
        }
    }

    fn show_hint(&self, game: &GameState, piece: &Piece) {
        let valid_moves = game.get_valid_moves(piece);
        if valid_moves.is_empty() {
            println!("No valid moves available!");
            return;
        }

        // Simple heuristic: prefer center positions and larger coverage
        let mut best_move = valid_moves[0];
        let mut best_score = self.evaluate_move_simple(game, piece, best_move.0, best_move.1);
        
        for &(x, y) in &valid_moves[1..] {
            let score = self.evaluate_move_simple(game, piece, x, y);
            if score > best_score {
                best_score = score;
                best_move = (x, y);
            }
        }
        
        println!("💡 Suggested move: {} {} (score: {})", best_move.1, best_move.0, best_score);
    }

    fn evaluate_move_simple(&self, game: &GameState, piece: &Piece, x: usize, y: usize) -> i32 {
        let mut score = 0;
        
        // Prefer larger pieces
        score += piece.shape.len() as i32 * 10;
        
        // Prefer center positions
        let center_x = game.width / 2;
        let center_y = game.height / 2;
        let distance_to_center = ((x as i32 - center_x as i32).abs() + (y as i32 - center_y as i32).abs()) as i32;
        score -= distance_to_center;
        
        score
    }

    fn show_valid_moves(&self, game: &GameState, piece: &Piece) {
        let valid_moves = game.get_valid_moves(piece);
        println!("All valid moves ({} total):", valid_moves.len());
        
        for (i, &(x, y)) in valid_moves.iter().enumerate() {
            print!("({}, {}) ", y, x);
            if (i + 1) % 8 == 0 {
                println!();
            }
        }
        if valid_moves.len() % 8 != 0 {
            println!();
        }
    }

    fn show_scores(&self, game: &GameState) {
        let (p1_score, p2_score) = game.calculate_scores();
        println!("Current Scores:");
        println!("Player 1 (@): {}", p1_score);
        println!("Player 2 ($): {}", p2_score);
        
        let total_cells = game.width * game.height;
        let empty_cells = total_cells - (p1_score + p2_score) as usize;
        println!("Empty cells: {}", empty_cells);
        
        let percentage_p1 = (p1_score as f32 / total_cells as f32) * 100.0;
        let percentage_p2 = (p2_score as f32 / total_cells as f32) * 100.0;
        println!("Territory: P1: {:.1}% | P2: {:.1}%", percentage_p1, percentage_p2);
    }

    fn parse_coordinates(&self, input: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }
        
        match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            (Ok(row), Ok(col)) => Some((col, row)), // Convert to (x, y)
            _ => None,
        }
    }
}

impl Player for HumanPlayer {
    fn get_move(&mut self, game: &GameState, piece: &Piece) -> Option<(usize, usize)> {
        loop {
            self.display_game_info(game, piece);
            
            match self.get_user_input() {
                Ok(input) => {
                    let input_lower = input.to_lowercase();
                    
                    match input_lower.as_str() {
                        "help" | "h" => {
                            self.show_help(game, piece);
                            continue;
                        }
                        "hint" => {
                            self.show_hint(game, piece);
                            continue;
                        }
                        "valid" | "v" => {
                            self.show_valid_moves(game, piece);
                            continue;
                        }
                        "scores" | "score" | "s" => {
                            self.show_scores(game);
                            continue;
                        }
                        "quit" | "q" | "exit" => {
                            println!("Thanks for playing!");
                            return None;
                        }
                        _ => {
                            if let Some((x, y)) = self.parse_coordinates(&input) {
                                if x < game.width && y < game.height {
                                    if game.is_valid_move(piece, x, y) {
                                        println!("✅ Valid move: ({}, {})", y, x);
                                        return Some((x, y));
                                    } else {
                                        println!("❌ Invalid move! Check the rules and try again.");
                                        println!("   Hint: Exactly one piece cell must overlap your territory.");
                                    }
                                } else {
                                    println!("❌ Coordinates out of bounds! Board size: {}x{}", game.height, game.width);
                                }
                            } else {
                                println!("❌ Invalid input format! Use: row column (e.g., '5 3')");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error reading input: {}", e);
                    return None;
                }
            }
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_human(&self) -> bool {
        true
    }
}

pub struct AIPlayerWrapper {
    ai: Box<dyn crate::AIPlayer>,
}

impl AIPlayerWrapper {
    pub fn new(ai: Box<dyn crate::AIPlayer>) -> Self {
        Self { ai }
    }
}

impl Player for AIPlayerWrapper {
    fn get_move(&mut self, game: &GameState, piece: &Piece) -> Option<(usize, usize)> {
        println!("\n{} is thinking...", self.ai.get_name());
        
        let start_time = std::time::Instant::now();
        let result = self.ai.choose_move(game, piece);
        let elapsed = start_time.elapsed();
        
        match result {
            Some((x, y)) => {
                println!("🤖 {} chose: ({}, {}) [took {:.2}ms]", 
                         self.ai.get_name(), y, x, elapsed.as_millis());
                Some((x, y))
            }
            None => {
                println!("🤖 {} found no valid moves", self.ai.get_name());
                None
            }
        }
    }

    fn get_name(&self) -> &str {
        self.ai.get_name()
    }

    fn is_human(&self) -> bool {
        false
    }
}
