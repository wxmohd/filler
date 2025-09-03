use std::fmt;
use crate::piece::Piece;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Player1Old,    // @
    Player1New,    // a
    Player2Old,    // $
    Player2New,    // s
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Player1Old => write!(f, "@"),
            Cell::Player1New => write!(f, "a"),
            Cell::Player2Old => write!(f, "$"),
            Cell::Player2New => write!(f, "s"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub current_player: u8,
    pub turn: u32,
    pub game_over: bool,
    pub winner: Option<u8>,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = vec![vec![Cell::Empty; width]; height];
        
        // Set starting positions according to Filler rules
        board[0][0] = Cell::Player1Old;
        board[height - 1][width - 1] = Cell::Player2Old;
        
        Self {
            board,
            width,
            height,
            current_player: 1,
            turn: 1,
            game_over: false,
            winner: None,
        }
    }

    pub fn from_board(board: Vec<Vec<Cell>>) -> Self {
        let height = board.len();
        let width = if height > 0 { board[0].len() } else { 0 };
        
        Self {
            board,
            width,
            height,
            current_player: 1,
            turn: 1,
            game_over: false,
            winner: None,
        }
    }

    pub fn is_valid_move(&self, piece: &Piece, x: usize, y: usize) -> bool {
        let mut overlap_count = 0;
        let player_territory = if self.current_player == 1 {
            [Cell::Player1Old, Cell::Player1New]
        } else {
            [Cell::Player2Old, Cell::Player2New]
        };
        
        let opponent_territory = if self.current_player == 1 {
            [Cell::Player2Old, Cell::Player2New]
        } else {
            [Cell::Player1Old, Cell::Player1New]
        };

        for (px, py) in &piece.shape {
            let abs_x = x + px;
            let abs_y = y + py;
            
            // Check bounds
            if abs_x >= self.width || abs_y >= self.height {
                return false;
            }
            
            let cell = &self.board[abs_y][abs_x];
            
            // Check overlap with own territory
            if player_territory.contains(cell) {
                overlap_count += 1;
            }
            
            // Check overlap with opponent territory
            if opponent_territory.contains(cell) {
                return false;
            }
        }
        
        // Must have exactly one overlap with own territory
        overlap_count == 1
    }

    pub fn place_piece(&mut self, piece: &Piece, x: usize, y: usize) -> bool {
        if !self.is_valid_move(piece, x, y) {
            return false;
        }

        // Convert old pieces to old territory
        self.convert_new_to_old();
        
        // Place new piece
        let new_cell = if self.current_player == 1 {
            Cell::Player1New
        } else {
            Cell::Player2New
        };

        for (px, py) in &piece.shape {
            let abs_x = x + px;
            let abs_y = y + py;
            self.board[abs_y][abs_x] = new_cell.clone();
        }

        true
    }

    fn convert_new_to_old(&mut self) {
        for row in &mut self.board {
            for cell in row {
                match cell {
                    Cell::Player1New => *cell = Cell::Player1Old,
                    Cell::Player2New => *cell = Cell::Player2Old,
                    _ => {}
                }
            }
        }
    }

    pub fn get_valid_moves(&self, piece: &Piece) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_valid_move(piece, x, y) {
                    valid_moves.push((x, y));
                }
            }
        }
        
        valid_moves
    }

    pub fn calculate_scores(&self) -> (u32, u32) {
        let mut p1_score = 0;
        let mut p2_score = 0;
        
        for row in &self.board {
            for cell in row {
                match cell {
                    Cell::Player1Old | Cell::Player1New => p1_score += 1,
                    Cell::Player2Old | Cell::Player2New => p2_score += 1,
                    Cell::Empty => {}
                }
            }
        }
        
        (p1_score, p2_score)
    }

    pub fn switch_player(&mut self) {
        self.current_player = if self.current_player == 1 { 2 } else { 1 };
        self.turn += 1;
    }

    pub fn check_game_over(&mut self, piece: &Piece) -> bool {
        let valid_moves = self.get_valid_moves(piece);
        
        if valid_moves.is_empty() {
            self.game_over = true;
            let (p1_score, p2_score) = self.calculate_scores();
            
            if p1_score > p2_score {
                self.winner = Some(1);
            } else if p2_score > p1_score {
                self.winner = Some(2);
            } else {
                self.winner = None; // Tie
            }
            
            return true;
        }
        
        false
    }

    pub fn display_board(&self) -> String {
        let mut result = String::new();
        
        // Header with column numbers
        result.push_str("   ");
        for x in 0..self.width {
            result.push_str(&format!("{:2}", x % 10));
        }
        result.push('\n');
        
        // Board rows
        for (y, row) in self.board.iter().enumerate() {
            result.push_str(&format!("{:2} ", y));
            for cell in row {
                result.push_str(&format!("{} ", cell));
            }
            result.push('\n');
        }
        
        result
    }
}
