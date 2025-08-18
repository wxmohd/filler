use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
};
use std::io::{self, stdout, Write};

#[derive(Debug, Clone)]
pub struct GameDisplay {
    pub anfield: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub current_piece: Vec<String>,
    pub player1_score: u32,
    pub player2_score: u32,
    pub current_player: u8,
    pub game_info: String,
}

impl GameDisplay {
    pub fn new() -> Self {
        Self {
            anfield: Vec::new(),
            width: 0,
            height: 0,
            current_piece: Vec::new(),
            player1_score: 0,
            player2_score: 0,
            current_player: 1,
            game_info: String::new(),
        }
    }

    pub fn process_line(&mut self, line: &str) {
        // Parse different types of input lines
        if line.starts_with("$$$") {
            // Player info line: $$$ exec p1 : [robots/bender]
            self.game_info = line.to_string();
            if line.contains("p1") {
                self.current_player = 1;
            } else if line.contains("p2") {
                self.current_player = 2;
            }
        } else if line.starts_with("Anfield") {
            // Anfield header: Anfield 20 15:
            if let Some(dimensions) = self.parse_anfield_header(line) {
                self.width = dimensions.0;
                self.height = dimensions.1;
                self.anfield = vec![vec!['.'; self.width]; self.height];
            }
        } else if line.starts_with("Piece") {
            // Piece header: Piece 4 1:
            self.current_piece.clear();
        } else if line.len() > 4 && line.chars().nth(3) == Some(' ') {
            // Anfield row: 000 ....................
            if let Some(row_num) = line[..3].parse::<usize>().ok() {
                if row_num < self.height {
                    let row_data = &line[4..];
                    for (x, ch) in row_data.chars().enumerate() {
                        if x < self.width {
                            self.anfield[row_num][x] = ch;
                        }
                    }
                }
            }
        } else if !line.trim().is_empty() && !line.starts_with("Anfield") && !line.starts_with("Piece") {
            // Piece pattern line
            self.current_piece.push(line.to_string());
        }

        // Update scores by counting territory
        self.update_scores();
    }

    fn parse_anfield_header(&self, line: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "Anfield" {
            let width = parts[1].parse().ok()?;
            let height = parts[2].trim_end_matches(':').parse().ok()?;
            Some((width, height))
        } else {
            None
        }
    }

    fn update_scores(&mut self) {
        self.player1_score = 0;
        self.player2_score = 0;

        for row in &self.anfield {
            for &cell in row {
                match cell {
                    '@' | 'a' => self.player1_score += 1,
                    '$' | 's' => self.player2_score += 1,
                    _ => {}
                }
            }
        }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = stdout();

        // Clear and render header
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("═══════════════════════════════════════════════════════════════\n"),
            Print("                        FILLER GAME VISUALIZER                  \n"),
            Print("═══════════════════════════════════════════════════════════════\n"),
            ResetColor
        )?;

        // Render game info
        if !self.game_info.is_empty() {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(format!("Game: {}\n", self.game_info)),
                ResetColor
            )?;
        }

        // Render scores
        execute!(
            stdout,
            SetForegroundColor(Color::White),
            Print(format!("Player 1 (@): {} cells  |  Player 2 ($): {} cells\n", 
                self.player1_score, self.player2_score)),
            Print(format!("Current Player: {}\n\n", self.current_player)),
            ResetColor
        )?;

        // Render anfield with column numbers
        if !self.anfield.is_empty() {
            // Column header
            execute!(stdout, Print("    "))?;
            for x in 0..self.width {
                execute!(stdout, Print(format!("{}", x % 10)))?;
            }
            execute!(stdout, Print("\n"))?;

            // Render each row
            for (y, row) in self.anfield.iter().enumerate() {
                execute!(stdout, Print(format!("{:03} ", y)))?;
                
                for &cell in row {
                    match cell {
                        '@' => execute!(stdout, SetForegroundColor(Color::Red), Print("@"), ResetColor)?,
                        'a' => execute!(stdout, SetForegroundColor(Color::Red), SetBackgroundColor(Color::DarkRed), Print("a"), ResetColor)?,
                        '$' => execute!(stdout, SetForegroundColor(Color::Blue), Print("$"), ResetColor)?,
                        's' => execute!(stdout, SetForegroundColor(Color::Blue), SetBackgroundColor(Color::DarkBlue), Print("s"), ResetColor)?,
                        '.' => execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("."), ResetColor)?,
                        _ => execute!(stdout, Print(cell.to_string()))?,
                    }
                }
                execute!(stdout, Print("\n"))?;
            }
        }

        // Render current piece
        if !self.current_piece.is_empty() {
            execute!(
                stdout,
                Print("\nCurrent Piece:\n"),
                SetForegroundColor(Color::Green)
            )?;
            for piece_line in &self.current_piece {
                execute!(stdout, Print(format!("{}\n", piece_line)))?;
            }
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;
        Ok(())
    }
}