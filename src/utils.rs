use std::io::{self, Write};
use crate::{GameState, Player, HumanPlayer, AIPlayerWrapper, AIDifficulty, create_ai, GameVisualizer, GameReplay};

#[derive(Debug, Clone)]
pub enum GameMode {
    HumanVsAI,
    AIVsAI,
    HumanVsHuman,
}

pub struct GameConfig {
    pub mode: GameMode,
    pub board_width: usize,
    pub board_height: usize,
    pub ai_difficulty: AIDifficulty,
    pub show_animations: bool,
    pub player1_name: String,
    pub player2_name: String,
    pub enable_replay: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            mode: GameMode::HumanVsAI,
            board_width: 15,
            board_height: 10,
            ai_difficulty: AIDifficulty::Medium,
            show_animations: true,
            player1_name: "Player 1".to_string(),
            player2_name: "AI".to_string(),
            enable_replay: false,
        }
    }
}

pub fn setup_game() -> GameConfig {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    FILLER GAME SETUP                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let mut config = GameConfig::default();

    // Game mode selection
    println!("Select game mode:");
    println!("1. Human vs AI");
    println!("2. AI vs AI");
    println!("3. Human vs Human");
    
    let mode = loop {
        print!("Enter choice (1-3): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => break GameMode::HumanVsAI,
            "2" => break GameMode::AIVsAI,
            "3" => break GameMode::HumanVsHuman,
            _ => println!("Invalid choice, please enter 1, 2, or 3"),
        }
    };
    
    config.mode = mode;

    // Board size selection
    println!("\nSelect board size:");
    println!("1. Small (10x8)");
    println!("2. Medium (15x10) [Default]");
    println!("3. Large (20x14)");
    println!("4. Extra Large (30x20)");
    println!("5. Custom");
    
    let (width, height) = loop {
        print!("Enter choice (1-5): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => break (10, 8),
            "2" | "" => break (15, 10),
            "3" => break (20, 14),
            "4" => break (30, 20),
            "5" => {
                print!("Enter width: ");
                io::stdout().flush().unwrap();
                let mut w_input = String::new();
                io::stdin().read_line(&mut w_input).unwrap();
                
                print!("Enter height: ");
                io::stdout().flush().unwrap();
                let mut h_input = String::new();
                io::stdin().read_line(&mut h_input).unwrap();
                
                match (w_input.trim().parse::<usize>(), h_input.trim().parse::<usize>()) {
                    (Ok(w), Ok(h)) if w >= 5 && h >= 5 && w <= 50 && h <= 50 => break (w, h),
                    _ => {
                        println!("Invalid size! Width and height must be between 5 and 50.");
                        continue;
                    }
                }
            }
            _ => println!("Invalid choice, please enter 1-5"),
        }
    };
    
    config.board_width = width;
    config.board_height = height;

    // AI difficulty (if applicable)
    if matches!(config.mode, GameMode::HumanVsAI | GameMode::AIVsAI) {
        println!("\nSelect AI difficulty:");
        println!("1. Easy (Random moves)");
        println!("2. Medium (Greedy strategy) [Default]");
        println!("3. Hard (Minimax depth 3)");
        println!("4. Expert (Minimax depth 5)");
        
        let difficulty = loop {
            print!("Enter choice (1-4): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim() {
                "1" => break AIDifficulty::Easy,
                "2" | "" => break AIDifficulty::Medium,
                "3" => break AIDifficulty::Hard,
                "4" => break AIDifficulty::Expert,
                _ => println!("Invalid choice, please enter 1-4"),
            }
        };
        
        config.ai_difficulty = difficulty;
    }

    // Player names
    match config.mode {
        GameMode::HumanVsAI => {
            print!("Enter your name (default: Player 1): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let name = input.trim();
            if !name.is_empty() {
                config.player1_name = name.to_string();
            }
            config.player2_name = format!("AI ({:?})", config.ai_difficulty);
        }
        GameMode::HumanVsHuman => {
            print!("Enter Player 1 name (default: Player 1): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let name = input.trim();
            if !name.is_empty() {
                config.player1_name = name.to_string();
            }
            
            print!("Enter Player 2 name (default: Player 2): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let name = input.trim();
            if !name.is_empty() {
                config.player2_name = name.to_string();
            } else {
                config.player2_name = "Player 2".to_string();
            }
        }
        GameMode::AIVsAI => {
            config.player1_name = format!("AI 1 ({:?})", config.ai_difficulty);
            config.player2_name = format!("AI 2 ({:?})", config.ai_difficulty);
        }
    }

    // Animation settings
    print!("Enable animations? (y/N): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    config.show_animations = matches!(input.trim().to_lowercase().as_str(), "y" | "yes");

    // Replay settings
    print!("Enable game replay recording? (y/N): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    config.enable_replay = matches!(input.trim().to_lowercase().as_str(), "y" | "yes");

    println!("\n{}", "═".repeat(60));
    println!("Game Configuration:");
    println!("Mode: {:?}", config.mode);
    println!("Board: {}x{}", config.board_width, config.board_height);
    println!("Player 1: {}", config.player1_name);
    println!("Player 2: {}", config.player2_name);
    if matches!(config.mode, GameMode::HumanVsAI | GameMode::AIVsAI) {
        println!("AI Difficulty: {:?}", config.ai_difficulty);
    }
    println!("Animations: {}", if config.show_animations { "Enabled" } else { "Disabled" });
    println!("Replay: {}", if config.enable_replay { "Enabled" } else { "Disabled" });
    println!("{}", "═".repeat(60));

    print!("\nPress Enter to start the game...");
    io::stdout().flush().unwrap();
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();

    config
}

pub fn run_game(config: GameConfig) -> io::Result<()> {
    let mut game = GameState::new(config.board_width, config.board_height);
    let visualizer = GameVisualizer::with_settings(
        config.show_animations,
        500,
        true,
    );
    
    let mut replay = if config.enable_replay {
        Some(GameReplay::new())
    } else {
        None
    };

    // Create players based on game mode
    let mut player1: Box<dyn Player> = match config.mode {
        GameMode::HumanVsAI | GameMode::HumanVsHuman => {
            Box::new(HumanPlayer::new(config.player1_name.clone()))
        }
        GameMode::AIVsAI => {
            Box::new(AIPlayerWrapper::new(create_ai(config.ai_difficulty.clone())))
        }
    };

    let mut player2: Box<dyn Player> = match config.mode {
        GameMode::HumanVsAI | GameMode::AIVsAI => {
            Box::new(AIPlayerWrapper::new(create_ai(config.ai_difficulty.clone())))
        }
        GameMode::HumanVsHuman => {
            Box::new(HumanPlayer::new(config.player2_name.clone()))
        }
    };

    let mut piece_gen = crate::Piece::generate_sequence(42); // Fixed seed for reproducibility
    
    visualizer.display_game_header();
    
    // Main game loop
    loop {
        let current_piece = piece_gen.next();
        
        // Check if current player has valid moves
        if game.check_game_over(&current_piece) {
            break;
        }

        visualizer.display_game_state(&game, &current_piece, &config.player1_name, &config.player2_name);

        // Get current player
        let current_player = if game.current_player == 1 {
            &mut player1
        } else {
            &mut player2
        };

        // Get move from player
        let move_result = current_player.get_move(&game, &current_piece);
        
        match move_result {
            Some((x, y)) => {
                let success = game.place_piece(&current_piece, x, y);
                visualizer.display_move_result(&game, x, y, current_player.get_name(), success);
                
                if success {
                    // Record move for replay
                    if let Some(ref mut replay) = replay {
                        replay.add_move(game.current_player, current_piece.clone(), Some((x, y)), game.clone());
                    }
                    
                    game.switch_player();
                } else {
                    println!("Invalid move! Try again.");
                    continue;
                }
            }
            None => {
                visualizer.display_no_valid_moves(current_player.get_name());
                
                // Record no-move for replay
                if let Some(ref mut replay) = replay {
                    replay.add_move(game.current_player, current_piece.clone(), None, game.clone());
                }
                
                game.switch_player();
            }
        }

        // Add delay for AI vs AI games
        if matches!(config.mode, GameMode::AIVsAI) && !current_player.is_human() {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    // Display game over
    visualizer.display_game_over(&game, &config.player1_name, &config.player2_name);

    // Offer replay
    if let Some(mut replay) = replay {
        print!("\nWould you like to watch the game replay? (y/N): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            replay.play_replay(&visualizer);
        }
    }

    Ok(())
}

pub fn display_main_menu() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                         FILLER GAME                          ║");
    println!("║                     Welcome to Filler!                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("🎮 A strategic territory control game");
    println!("📋 Rules:");
    println!("   • Take turns placing Tetris-like pieces");
    println!("   • Each piece must overlap exactly ONE of your cells");
    println!("   • Cannot overlap opponent territory");
    println!("   • Player with most territory wins!");
    println!();
    println!("🎯 Game Features:");
    println!("   • Multiple AI difficulty levels");
    println!("   • Human vs AI, AI vs AI, and Human vs Human modes");
    println!("   • Customizable board sizes");
    println!("   • Game replay system");
    println!("   • Interactive help and hints");
    println!();
}
