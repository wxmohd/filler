use std::io::{self, Write};
use std::time::{Duration, Instant};
use crate::{GameState, Piece};

pub struct GameVisualizer {
    show_animations: bool,
    animation_delay: Duration,
    clear_screen: bool,
}

impl GameVisualizer {
    pub fn new() -> Self {
        Self {
            show_animations: true,
            animation_delay: Duration::from_millis(500),
            clear_screen: true,
        }
    }

    pub fn with_settings(show_animations: bool, animation_delay_ms: u64, clear_screen: bool) -> Self {
        Self {
            show_animations,
            animation_delay: Duration::from_millis(animation_delay_ms),
            clear_screen,
        }
    }

    pub fn display_game_header(&self) {
        if self.clear_screen {
            self.clear_screen_cmd();
        }
        
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║                         FILLER GAME                          ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
    }

    pub fn display_game_state(&self, game: &GameState, piece: &Piece, player1_name: &str, player2_name: &str) {
        let (p1_score, p2_score) = game.calculate_scores();
        
        println!("Turn: {} | Current Player: {} ({})", 
                 game.turn,
                 if game.current_player == 1 { player1_name } else { player2_name },
                 if game.current_player == 1 { "@" } else { "$" });
        
        println!("Scores: {} {} | {} {}", 
                 player1_name, p1_score, 
                 player2_name, p2_score);
        
        // Progress bar
        let total_cells = game.width * game.height;
        let filled_cells = (p1_score + p2_score) as usize;
        let progress = (filled_cells as f32 / total_cells as f32 * 40.0) as usize;
        
        print!("Progress: [");
        for i in 0..40 {
            if i < progress {
                print!("█");
            } else {
                print!("░");
            }
        }
        println!("] {:.1}%", (filled_cells as f32 / total_cells as f32) * 100.0);
        
        println!("{}", "─".repeat(60));
        
        // Display board
        self.display_board_with_piece(game, Some(piece));
        
        println!("{}", "─".repeat(60));
    }

    pub fn display_board_with_piece(&self, game: &GameState, piece: Option<&Piece>) {
        // Header with column numbers
        print!("   ");
        for x in 0..game.width {
            print!("{:2}", x % 10);
        }
        println!();
        
        // Board rows
        for (y, row) in game.board.iter().enumerate() {
            print!("{:2} ", y);
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
        
        // Display current piece if provided
        if let Some(p) = piece {
            println!("\nCurrent Piece:");
            print!("{}", p.display());
        }
    }

    pub fn display_move_result(&self, game: &GameState, x: usize, y: usize, player_name: &str, success: bool) {
        if success {
            println!("✅ {} placed piece at ({}, {})", player_name, y, x);
            
            if self.show_animations {
                self.animate_piece_placement(game, x, y);
            }
        } else {
            println!("❌ {} attempted invalid move at ({}, {})", player_name, y, x);
        }
    }

    pub fn display_game_over(&self, game: &GameState, player1_name: &str, player2_name: &str) {
        println!("\n{}", "═".repeat(60));
        println!("                        GAME OVER                        ");
        println!("{}", "═".repeat(60));
        
        let (p1_score, p2_score) = game.calculate_scores();
        
        println!("Final Scores:");
        println!("{}: {} points", player1_name, p1_score);
        println!("{}: {} points", player2_name, p2_score);
        
        let total_cells = game.width * game.height;
        let p1_percentage = (p1_score as f32 / total_cells as f32) * 100.0;
        let p2_percentage = (p2_score as f32 / total_cells as f32) * 100.0;
        
        println!("\nTerritory Control:");
        println!("{}: {:.1}%", player1_name, p1_percentage);
        println!("{}: {:.1}%", player2_name, p2_percentage);
        
        match game.winner {
            Some(1) => {
                println!("\n🎉 {} WINS! 🎉", player1_name);
                self.display_victory_animation();
            }
            Some(2) => {
                println!("\n🎉 {} WINS! 🎉", player2_name);
                self.display_victory_animation();
            }
            None => {
                println!("\n🤝 IT'S A TIE! 🤝");
            }
            _ => {
                println!("\n🤝 IT'S A TIE! 🤝");
            }
        }
        
        println!("\nFinal Board:");
        self.display_board_with_piece(game, None);
    }

    pub fn display_no_valid_moves(&self, player_name: &str) {
        println!("⚠️  {} has no valid moves available!", player_name);
    }

    pub fn display_thinking(&self, player_name: &str) {
        if self.show_animations {
            print!("🤔 {} is thinking", player_name);
            io::stdout().flush().unwrap();
            
            for _ in 0..3 {
                std::thread::sleep(Duration::from_millis(300));
                print!(".");
                io::stdout().flush().unwrap();
            }
            println!();
        } else {
            println!("🤔 {} is thinking...", player_name);
        }
    }

    pub fn wait_for_input(&self, message: &str) {
        print!("{}", message);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn animate_piece_placement(&self, game: &GameState, x: usize, y: usize) {
        // Simple animation showing the piece being placed
        for frame in 0..3 {
            if self.clear_screen {
                self.clear_screen_cmd();
            }
            
            println!("Placing piece... Frame {}/3", frame + 1);
            
            // Show board with animation
            print!("   ");
            for col in 0..game.width {
                print!("{:2}", col % 10);
            }
            println!();
            
            for (row, board_row) in game.board.iter().enumerate() {
                print!("{:2} ", row);
                for (col, cell) in board_row.iter().enumerate() {
                    if col == x && row == y && frame % 2 == 0 {
                        print!("* "); // Blinking effect
                    } else {
                        print!("{} ", cell);
                    }
                }
                println!();
            }
            
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    fn display_victory_animation(&self) {
        if !self.show_animations {
            return;
        }
        
        let fireworks = ["🎆", "🎇", "✨", "🌟", "💫"];
        
        for _ in 0..5 {
            print!("   ");
            for _ in 0..10 {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let firework = fireworks[rng.gen_range(0..fireworks.len())];
                print!("{} ", firework);
            }
            println!();
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    fn clear_screen_cmd(&self) {
        // Cross-platform screen clearing
        if cfg!(target_os = "windows") {
            std::process::Command::new("cls").status().ok();
        } else {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
        }
    }
}

pub struct GameReplay {
    moves: Vec<ReplayMove>,
    current_move: usize,
}

#[derive(Debug, Clone)]
pub struct ReplayMove {
    pub player: u8,
    pub piece: Piece,
    pub position: Option<(usize, usize)>,
    pub game_state: GameState,
    pub timestamp: Instant,
}

impl GameReplay {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            current_move: 0,
        }
    }

    pub fn add_move(&mut self, player: u8, piece: Piece, position: Option<(usize, usize)>, game_state: GameState) {
        self.moves.push(ReplayMove {
            player,
            piece,
            position,
            game_state,
            timestamp: Instant::now(),
        });
    }

    pub fn play_replay(&mut self, visualizer: &GameVisualizer) {
        println!("🎬 Starting game replay ({} moves)", self.moves.len());
        println!("Controls: Space=play/pause, ←/→=step, R=restart, Q=quit");
        
        self.current_move = 0;
        let mut paused = true;
        
        loop {
            if self.current_move >= self.moves.len() {
                println!("📽️ Replay finished!");
                break;
            }
            
            let replay_move = &self.moves[self.current_move];
            
            visualizer.display_game_header();
            println!("Replay Move: {}/{}", self.current_move + 1, self.moves.len());
            println!("Player: {}", replay_move.player);
            
            if let Some((x, y)) = replay_move.position {
                println!("Move: ({}, {})", y, x);
            } else {
                println!("Move: No valid moves");
            }
            
            visualizer.display_board_with_piece(&replay_move.game_state, Some(&replay_move.piece));
            
            if paused {
                println!("\n⏸️  PAUSED - Press Space to continue");
            } else {
                println!("\n▶️  PLAYING - Press Space to pause");
                std::thread::sleep(Duration::from_millis(1000));
                self.current_move += 1;
                continue;
            }
            
            // Handle input
            match self.get_replay_input() {
                ReplayCommand::PlayPause => paused = !paused,
                ReplayCommand::StepForward => {
                    self.current_move = (self.current_move + 1).min(self.moves.len() - 1);
                }
                ReplayCommand::StepBackward => {
                    self.current_move = self.current_move.saturating_sub(1);
                }
                ReplayCommand::Restart => {
                    self.current_move = 0;
                }
                ReplayCommand::Quit => break,
            }
        }
    }

    fn get_replay_input(&self) -> ReplayCommand {
        use std::io::Read;
        
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap_or_default();
        
        match buffer[0] {
            b' ' => ReplayCommand::PlayPause,
            b'a' | b'h' => ReplayCommand::StepBackward,
            b'd' | b'l' => ReplayCommand::StepForward,
            b'r' => ReplayCommand::Restart,
            b'q' => ReplayCommand::Quit,
            _ => ReplayCommand::PlayPause,
        }
    }
}

enum ReplayCommand {
    PlayPause,
    StepForward,
    StepBackward,
    Restart,
    Quit,
}
