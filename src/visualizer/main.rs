use clap::{Arg, Command};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, stdout, BufRead, BufReader, Write};
use std::process::{Command as ProcessCommand, Stdio};
use std::fs::File;
use std::time::Duration;

mod display;
use display::GameDisplay;

fn main() -> io::Result<()> {
    let matches = Command::new("Filler Visualizer")
        .version("0.1.0")
        .about("Terminal-based visualizer for the Filler game")
        .arg(
            Arg::new("replay")
                .short('r')
                .long("replay")
                .value_name("FILE")
                .help("Replay mode: load a saved game log")
        )
        .arg(
            Arg::new("live")
                .short('l')
                .long("live")
                .help("Live mode: run game engine directly")
        )
        .arg(
            Arg::new("map")
                .short('m')
                .long("map")
                .value_name("MAP")
                .help("Map file to use")
                .default_value("maps/map01")
        )
        .arg(
            Arg::new("player1")
                .short('1')
                .long("player1")
                .value_name("P1")
                .help("Player 1 executable")
                .default_value("solution/filler_ai")
        )
        .arg(
            Arg::new("player2")
                .short('2')
                .long("player2")
                .value_name("P2")
                .help("Player 2 executable")
                .default_value("robots/bender")
        )
        .get_matches();

    if let Some(replay_file) = matches.get_one::<String>("replay") {
        run_replay_mode(replay_file)?;
    } else if matches.get_flag("live") {
        let map = matches.get_one::<String>("map").unwrap();
        let player1 = matches.get_one::<String>("player1").unwrap();
        let player2 = matches.get_one::<String>("player2").unwrap();
        run_live_mode(map, player1, player2)?;
    } else {
        println!("Filler Game Visualizer");
        println!("Usage:");
        println!("  --live -m <map> -1 <player1> -2 <player2>  : Live mode");
        println!("  --replay <file>                            : Replay mode");
        println!();
        println!("Controls (in replay mode):");
        println!("  Space: play/pause");
        println!("  Left/Right: step backward/forward");
        println!("  R: restart");
        println!("  Q: quit");
    }

    Ok(())
}

fn run_live_mode(map: &str, player1: &str, player2: &str) -> io::Result<()> {
    println!("Starting live game: {} vs {}", player1, player2);
    println!("Map: {}", map);
    println!("Press Ctrl+C to stop");

    // Start the game engine
    let mut child = ProcessCommand::new("./game_engine")
        .args(&["-f", map, "-p1", player1, "-p2", player2])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        let mut display = GameDisplay::new();
        
        for line in reader.lines() {
            let line = line?;
            display.process_line(&line);
            display.render()?;
            std::thread::sleep(Duration::from_millis(500));
        }
    }

    child.wait()?;
    Ok(())
}

fn run_replay_mode(replay_file: &str) -> io::Result<()> {
    let file = File::open(replay_file)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

    if lines.is_empty() {
        println!("Empty replay file");
        return Ok(());
    }

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    let mut display = GameDisplay::new();
    let mut current_line = 0;
    let mut playing = false;
    let mut last_update = std::time::Instant::now();

    loop {
        // Handle input
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char(' ') => playing = !playing,
                    KeyCode::Left => {
                        if current_line > 0 {
                            current_line -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if current_line < lines.len() - 1 {
                            current_line += 1;
                        }
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        current_line = 0;
                        display = GameDisplay::new();
                    }
                    _ => {}
                }
            }
        }

        // Auto-advance if playing
        if playing && last_update.elapsed() > Duration::from_millis(1000) {
            if current_line < lines.len() - 1 {
                current_line += 1;
            } else {
                playing = false;
            }
            last_update = std::time::Instant::now();
        }

        // Process current line
        if current_line < lines.len() {
            display.process_line(&lines[current_line]);
        }

        // Render
        execute!(stdout, cursor::MoveTo(0, 0))?;
        display.render()?;
        
        // Show controls
        execute!(
            stdout,
            cursor::MoveTo(0, 25),
            SetForegroundColor(Color::Yellow),
            Print(format!("Line: {}/{} | {} | Space: play/pause | ←→: step | R: restart | Q: quit", 
                current_line + 1, lines.len(), if playing { "PLAYING" } else { "PAUSED" })),
            ResetColor
        )?;

        stdout.flush()?;
    }

    execute!(stdout, cursor::Show, ResetColor)?;
    terminal::disable_raw_mode()?;
    Ok(())
}