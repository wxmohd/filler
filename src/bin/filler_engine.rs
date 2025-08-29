use std::env;
use std::io::{self, Write, BufRead, BufReader};
use std::process::{Command, Stdio};
use filler::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} [options]", args[0]);
        println!("Options:");
        println!("  -f <map>     Use map file");
        println!("  -p1 <player> Player 1 executable");
        println!("  -p2 <player> Player 2 executable");
        println!("  -h           Human vs AI mode");
        return Ok(());
    }

    let mut map_file = None;
    let mut player1 = None;
    let mut player2 = None;
    let mut human_mode = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                if i + 1 < args.len() {
                    map_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-p1" => {
                if i + 1 < args.len() {
                    player1 = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-p2" => {
                if i + 1 < args.len() {
                    player2 = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-h" => {
                human_mode = true;
                i += 1;
            }
            _ => i += 1,
        }
    }

    // Default settings
    let board_size = if let Some(_) = map_file {
        // Parse map file if provided
        (15, 10) // Default for now
    } else {
        (15, 10)
    };

    run_terminal_game(board_size.0, board_size.1, player1, player2, human_mode)
}

fn run_terminal_game(
    width: usize, 
    height: usize, 
    player1_cmd: Option<String>, 
    player2_cmd: Option<String>,
    human_mode: bool
) -> io::Result<()> {
    let mut game = GameState::new(width, height);
    let mut piece_generator = PieceGenerator::new(42);
    
    // Initialize players
    let mut p1_process = if let Some(ref cmd) = player1_cmd {
        Some(spawn_player_process(cmd)?)
    } else {
        None
    };
    
    let mut p2_process = if let Some(ref cmd) = player2_cmd {
        Some(spawn_player_process(cmd)?)
    } else {
        None
    };

    println!("$$$ exec p1 : [{}]", 
             player1_cmd.as_deref().unwrap_or("human"));
    println!("$$$ exec p2 : [{}]", 
             player2_cmd.as_deref().unwrap_or("AI"));

    // Main game loop
    loop {
        let current_piece = piece_generator.next();
        
        // Check if game is over
        if game.check_game_over(&current_piece) {
            break;
        }

        // Display current state
        display_anfield(&game);
        display_piece(&current_piece);

        // Get move from current player
        let move_result = if game.current_player == 1 {
            if human_mode || p1_process.is_none() {
                get_human_move(&game, &current_piece)
            } else {
                get_bot_move(p1_process.as_mut().unwrap(), &game, &current_piece, 1)
            }
        } else {
            if p2_process.is_none() {
                // Default AI if no player 2 specified
                let mut ai = create_ai(AIDifficulty::Medium);
                ai.choose_move(&game, &current_piece)
            } else {
                get_bot_move(p2_process.as_mut().unwrap(), &game, &current_piece, 2)
            }
        };

        match move_result {
            Some((x, y)) => {
                if game.is_valid_move(&current_piece, x, y) {
                    game.place_piece(&current_piece, x, y);
                    println!("Player {} placed piece at ({}, {})", game.current_player, y, x);
                    game.switch_player();
                } else {
                    println!("Invalid move by player {}", game.current_player);
                    break;
                }
            }
            None => {
                println!("Player {} has no valid moves", game.current_player);
                break;
            }
        }
    }

    // Game over
    display_anfield(&game);
    let (p1_score, p2_score) = game.calculate_scores();
    
    println!("== O fin de la partie ==");
    println!("Player 1: {} points", p1_score);
    println!("Player 2: {} points", p2_score);
    
    if p1_score > p2_score {
        println!("Player 1 WINS!");
    } else if p2_score > p1_score {
        println!("Player 2 WINS!");
    } else {
        println!("TIE!");
    }

    Ok(())
}

fn display_anfield(game: &GameState) {
    println!("Anfield {} {}:", game.height, game.width);
    
    for (y, row) in game.board.iter().enumerate() {
        print!("{:03} ", y);
        for cell in row {
            let ch = match cell {
                Cell::Empty => '.',
                Cell::Player1Old => '@',
                Cell::Player1New => 'a',
                Cell::Player2Old => '$',
                Cell::Player2New => 's',
            };
            print!("{}", ch);
        }
        println!();
    }
}

fn display_piece(piece: &Piece) {
    println!("Piece {} {}:", piece.height, piece.width);
    
    for y in 0..piece.height {
        for x in 0..piece.width {
            if piece.shape.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_human_move(game: &GameState, piece: &Piece) -> Option<(usize, usize)> {
    loop {
        print!("Enter move (row column): ");
        io::stdout().flush().ok();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        
        let coords: Vec<&str> = input.trim().split_whitespace().collect();
        if coords.len() == 2 {
            if let (Ok(row), Ok(col)) = (coords[0].parse::<usize>(), coords[1].parse::<usize>()) {
                if game.is_valid_move(piece, col, row) {
                    return Some((col, row));
                } else {
                    println!("Invalid move, try again.");
                }
            } else {
                println!("Invalid input format.");
            }
        } else {
            println!("Invalid input format. Use: row column");
        }
    }
}

fn spawn_player_process(cmd: &str) -> io::Result<std::process::Child> {
    Command::new(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

fn get_bot_move(
    process: &mut std::process::Child, 
    game: &GameState, 
    piece: &Piece, 
    player: u8
) -> Option<(usize, usize)> {
    // Send game state to bot
    if let Some(stdin) = process.stdin.as_mut() {
        // Send player info
        writeln!(stdin, "$$$ exec p{} : [bot]", player).ok()?;
        
        // Send anfield
        writeln!(stdin, "Anfield {} {}:", game.height, game.width).ok()?;
        for (y, row) in game.board.iter().enumerate() {
            write!(stdin, "{:03} ", y).ok()?;
            for cell in row {
                let ch = match cell {
                    Cell::Empty => '.',
                    Cell::Player1Old => '@',
                    Cell::Player1New => 'a',
                    Cell::Player2Old => '$',
                    Cell::Player2New => 's',
                };
                write!(stdin, "{}", ch).ok()?;
            }
            writeln!(stdin).ok()?;
        }
        
        // Send piece
        writeln!(stdin, "Piece {} {}:", piece.height, piece.width).ok()?;
        for y in 0..piece.height {
            for x in 0..piece.width {
                if piece.shape.contains(&(x, y)) {
                    write!(stdin, "*").ok()?;
                } else {
                    write!(stdin, ".").ok()?;
                }
            }
            writeln!(stdin).ok()?;
        }
        
        stdin.flush().ok()?;
    }

    // Read response from bot
    if let Some(stdout) = process.stdout.as_mut() {
        let mut reader = BufReader::new(stdout);
        let mut response = String::new();
        
        if reader.read_line(&mut response).is_ok() {
            let coords: Vec<&str> = response.trim().split_whitespace().collect();
            if coords.len() == 2 {
                if let (Ok(row), Ok(col)) = (coords[0].parse::<usize>(), coords[1].parse::<usize>()) {
                    return Some((col, row));
                }
            }
        }
    }
    
    None
}
