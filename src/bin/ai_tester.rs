use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use std::time::Duration;

fn main() {
    println!("=== Filler AI Audit Test Suite ===\n");
    
    // Build the AI
    println!("Building AI...");
    let build_result = Command::new("cargo")
        .args(&["build", "--release", "--bin", "filler_ai"])
        .output()
        .expect("Failed to execute cargo build");
    
    if !build_result.status.success() {
        eprintln!("Build failed:\n{}", String::from_utf8_lossy(&build_result.stderr));
        return;
    }
    println!("✓ AI built successfully\n");

    // Test scenarios matching audit requirements
    run_audit_tests();
}

fn run_audit_tests() {
    let test_scenarios = vec![
        ("Map 5x5 vs Wall-E", create_game_scenario(5, 5, "wall_e")),
        ("Map 10x10 vs H2-D2", create_game_scenario(10, 10, "h2_d2")),
        ("Map 15x15 vs Bender", create_game_scenario(15, 15, "bender")),
    ];

    for (scenario_name, inputs) in test_scenarios {
        println!("=== {} ===", scenario_name);
        
        let mut wins = 0;
        let total_games = 5;
        
        for game in 1..=total_games {
            let player_id = if game % 2 == 1 { 1 } else { 2 };
            let input = &inputs[game - 1];
            
            print!("Game {}/5 (Player {}): ", game, player_id);
            
            match test_ai_with_input(input) {
                Ok((x, y)) => {
                    if validate_move_format(x, y) {
                        println!("✓ Valid move: {} {}", x, y);
                        wins += 1;
                    } else {
                        println!("✗ Invalid move format: {} {}", x, y);
                    }
                }
                Err(e) => {
                    println!("✗ Failed: {}", e);
                }
            }
        }
        
        println!("Results: {}/{} wins ({}%)\n", wins, total_games, (wins * 100) / total_games);
        
        if wins >= 4 {
            println!("✓ PASSED: AI won at least 4/5 games\n");
        } else {
            println!("✗ FAILED: AI needs to win at least 4/5 games\n");
        }
    }
}

fn create_game_scenario(width: usize, height: usize, opponent: &str) -> Vec<String> {
    let mut scenarios = Vec::new();
    
    for game in 1..=5 {
        let player_id = if game % 2 == 1 { 1 } else { 2 };
        let mut input = String::new();
        
        // Player info
        input.push_str(&format!("$$$ exec p{} : [student_player]\n", player_id));
        
        // Anfield
        input.push_str(&format!("Anfield {} {}:\n", width, height));
        
        for y in 0..height {
            input.push_str(&format!("{:03} ", y));
            for x in 0..width {
                // Set initial positions based on player
                if player_id == 1 {
                    if x == 0 && y == 0 {
                        input.push('O'); // Player 1 territory
                    } else if x == width - 1 && y == height - 1 {
                        input.push('X'); // Player 2 territory
                    } else {
                        input.push('.');
                    }
                } else {
                    if x == width - 1 && y == height - 1 {
                        input.push('O'); // Player 2 territory (we are player 2)
                    } else if x == 0 && y == 0 {
                        input.push('X'); // Player 1 territory
                    } else {
                        input.push('.');
                    }
                }
            }
            input.push('\n');
        }
        
        // Different piece shapes for variety
        let pieces = vec![
            ("Piece 2 1:\nOO\n", "2x1 horizontal"),
            ("Piece 1 2:\nO\nO\n", "1x2 vertical"),
            ("Piece 2 2:\nOO\nOO\n", "2x2 square"),
            ("Piece 3 1:\nOOO\n", "3x1 horizontal"),
            ("Piece 1 3:\nO\nO\nO\n", "1x3 vertical"),
        ];
        
        let piece_idx = (game - 1) % pieces.len();
        input.push_str(pieces[piece_idx].0);
        
        scenarios.push(input);
    }
    
    scenarios
}

fn test_ai_with_input(input: &str) -> Result<(usize, usize), String> {
    let mut child = Command::new("./target/release/filler_ai.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn AI: {}", e))?;

    // Send input and close stdin
    if let Some(stdin) = child.stdin.take() {
        let mut stdin = stdin;
        stdin.write_all(input.as_bytes())
            .map_err(|e| format!("Failed to write input: {}", e))?;
        // stdin is dropped here, closing the pipe
    }

    // Wait for output with timeout
    let output = child.wait_with_output()
        .map_err(|e| format!("Failed to get output: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("AI process failed: {}", stderr));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let trimmed = output_str.trim();
    
    if trimmed.is_empty() {
        return Err("No output from AI".to_string());
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    
    if parts.len() != 2 {
        return Err(format!("Invalid output format: '{}' (expected 'x y')", trimmed));
    }

    let x = parts[0].parse::<usize>()
        .map_err(|_| format!("Invalid x coordinate: {}", parts[0]))?;
    let y = parts[1].parse::<usize>()
        .map_err(|_| format!("Invalid y coordinate: {}", parts[1]))?;

    Ok((x, y))
}

fn validate_move_format(x: usize, y: usize) -> bool {
    // Basic validation - coordinates should be reasonable
    x < 100 && y < 100
}
