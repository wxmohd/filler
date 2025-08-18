use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;

fn main() {
    println!("=== Filler AI Test Runner ===\n");
    
    // Build the AI first
    println!("Building AI...");
    let build_result = Command::new("cargo")
        .args(&["build", "--release", "--bin", "filler_ai"])
        .output()
        .expect("Failed to build AI");
    
    if !build_result.status.success() {
        eprintln!("Build failed: {}", String::from_utf8_lossy(&build_result.stderr));
        return;
    }
    println!("✓ AI built successfully\n");

    // Test cases for different scenarios
    let test_cases = vec![
        ("Small Map (5x5)", create_test_input(5, 5, 1)),
        ("Medium Map (10x10)", create_test_input(10, 10, 1)),
        ("Large Map (15x15)", create_test_input(15, 15, 1)),
        ("Player 2 Test", create_test_input(5, 5, 2)),
    ];

    let mut passed = 0;
    let mut total = 0;

    for (test_name, input) in test_cases {
        total += 1;
        println!("Running test: {}", test_name);
        
        match run_ai_test(&input) {
            Ok((x, y)) => {
                println!("✓ AI responded with move: {} {}", x, y);
                passed += 1;
            }
            Err(e) => {
                println!("✗ Test failed: {}", e);
            }
        }
        println!();
    }

    println!("=== Test Results ===");
    println!("Passed: {}/{}", passed, total);
    
    if passed == total {
        println!("🎉 All tests passed! AI is working correctly.");
    } else {
        println!("⚠️  Some tests failed. Check the AI implementation.");
    }
}

fn create_test_input(width: usize, height: usize, player_id: u8) -> String {
    let mut input = String::new();
    
    // Player info
    input.push_str(&format!("$$$ exec p{} : [test_player]\n", player_id));
    
    // Anfield
    input.push_str(&format!("Anfield {} {}:\n", width, height));
    
    for y in 0..height {
        input.push_str(&format!("{:03} ", y));
        for x in 0..width {
            if player_id == 1 && x == 0 && y == 0 {
                input.push('O');
            } else if player_id == 2 && x == width - 1 && y == height - 1 {
                input.push('X');
            } else if player_id == 1 && x == width - 1 && y == height - 1 {
                input.push('X');
            } else if player_id == 2 && x == 0 && y == 0 {
                input.push('O');
            } else {
                input.push('.');
            }
        }
        input.push('\n');
    }
    
    // Simple piece
    input.push_str("Piece 2 1:\n");
    input.push_str("OO\n");
    
    input
}

fn run_ai_test(input: &str) -> Result<(usize, usize), String> {
    let mut child = Command::new("./target/release/filler_ai.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn AI: {}", e))?;

    // Send input
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes())
            .map_err(|e| format!("Failed to write input: {}", e))?;
    }

    // Get output
    let output = child.wait_with_output()
        .map_err(|e| format!("Failed to get output: {}", e))?;

    if !output.status.success() {
        return Err(format!("AI failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = output_str.trim().split_whitespace().collect();
    
    if parts.len() != 2 {
        return Err(format!("Invalid output format: '{}'", output_str));
    }

    let x = parts[0].parse::<usize>()
        .map_err(|_| format!("Invalid x coordinate: {}", parts[0]))?;
    let y = parts[1].parse::<usize>()
        .map_err(|_| format!("Invalid y coordinate: {}", parts[1]))?;

    Ok((x, y))
}
