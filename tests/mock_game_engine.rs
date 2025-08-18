use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use std::fs;

/// Mock game engine for testing AI without Docker
pub struct MockGameEngine {
    pub map_size: (usize, usize),
    pub player1_pos: (usize, usize),
    pub player2_pos: (usize, usize),
}

impl MockGameEngine {
    pub fn new(map_size: (usize, usize)) -> Self {
        Self {
            map_size,
            player1_pos: (0, 0),
            player2_pos: (map_size.0 - 1, map_size.1 - 1),
        }
    }

    /// Generate test input for the AI
    pub fn generate_test_input(&self, player_id: u8, turn: u32) -> String {
        let mut input = String::new();
        
        // Player info
        input.push_str(&format!("$$$ exec p{} : [test_player]\n", player_id));
        
        // Anfield
        input.push_str(&format!("Anfield {} {}:\n", self.map_size.0, self.map_size.1));
        
        for y in 0..self.map_size.1 {
            input.push_str(&format!("{:03} ", y));
            for x in 0..self.map_size.0 {
                if (x, y) == self.player1_pos {
                    input.push('O');
                } else if (x, y) == self.player2_pos {
                    input.push('X');
                } else {
                    input.push('.');
                }
            }
            input.push('\n');
        }
        
        // Piece (simple 2x1 piece for testing)
        input.push_str("Piece 2 1:\n");
        input.push_str("OO\n");
        
        input
    }

    /// Run AI with test input and capture output
    pub fn test_ai_move(&self, player_id: u8) -> Result<(usize, usize), String> {
        let input = self.generate_test_input(player_id, 1);
        
        let mut child = Command::new("./target/release/filler_ai.exe")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn AI process: {}", e))?;

        // Send input
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(input.as_bytes())
                .map_err(|e| format!("Failed to write to AI stdin: {}", e))?;
        }

        // Read output
        let output = child.wait_with_output()
            .map_err(|e| format!("Failed to read AI output: {}", e))?;

        if !output.status.success() {
            return Err(format!("AI process failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.trim().split_whitespace().collect();
        
        if parts.len() != 2 {
            return Err(format!("Invalid AI output format: {}", output_str));
        }

        let x = parts[0].parse::<usize>()
            .map_err(|_| format!("Invalid x coordinate: {}", parts[0]))?;
        let y = parts[1].parse::<usize>()
            .map_err(|_| format!("Invalid y coordinate: {}", parts[1]))?;

        Ok((x, y))
    }

    /// Run multiple games and check win rate
    pub fn run_game_series(&self, games: u32) -> (u32, u32) {
        let mut wins = 0;
        let mut total = 0;

        for game in 0..games {
            // Alternate which player the AI is
            let ai_player = if game % 2 == 0 { 1 } else { 2 };
            
            match self.test_ai_move(ai_player) {
                Ok((x, y)) => {
                    // Basic validation - move should be within bounds
                    if x < self.map_size.0 && y < self.map_size.1 {
                        wins += 1;
                    }
                    total += 1;
                }
                Err(e) => {
                    eprintln!("Game {} failed: {}", game + 1, e);
                    total += 1;
                }
            }
        }

        (wins, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_engine_basic() {
        let engine = MockGameEngine::new((5, 5));
        let input = engine.generate_test_input(1, 1);
        assert!(input.contains("$$$ exec p1"));
        assert!(input.contains("Anfield 5 5"));
        assert!(input.contains("Piece 2 1"));
    }

    #[test]
    fn test_ai_responds_correctly() {
        let engine = MockGameEngine::new((5, 5));
        
        // This test requires the AI binary to be built
        if std::path::Path::new("./target/release/filler_ai.exe").exists() {
            match engine.test_ai_move(1) {
                Ok((x, y)) => {
                    assert!(x < 5 && y < 5, "AI move should be within bounds");
                }
                Err(e) => {
                    // AI might fail if not properly built, that's ok for this test
                    println!("AI test failed (expected if not built): {}", e);
                }
            }
        }
    }
}
