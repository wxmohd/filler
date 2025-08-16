pub mod anfield;
pub mod piece;
pub mod player;
pub mod parser;

pub use anfield::{Anfield, Cell};
pub use piece::Piece;
pub use player::Player;
pub use parser::GameParser;

#[derive(Debug, Clone)]
pub struct GameState {
    pub anfield: Anfield,
    pub current_piece: Option<Piece>,
    pub player1: Player,
    pub player2: Player,
    pub current_player: u8,
    pub turn: u32,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            anfield: Anfield::new(width, height),
            current_piece: None,
            player1: Player::new(1),
            player2: Player::new(2),
            current_player: 1,
            turn: 0,
        }
    }

    pub fn get_current_player(&self) -> &Player {
        if self.current_player == 1 {
            &self.player1
        } else {
            &self.player2
        }
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        if self.current_player == 1 {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    pub fn get_opponent(&self) -> &Player {
        if self.current_player == 1 {
            &self.player2
        } else {
            &self.player1
        }
    }

    pub fn switch_player(&mut self) {
        self.current_player = if self.current_player == 1 { 2 } else { 1 };
        self.turn += 1;
    }

    pub fn update_scores(&mut self) {
        self.player1.update_score(self.anfield.count_player_cells(1));
        self.player2.update_score(self.anfield.count_player_cells(2));
    }

    pub fn place_piece(&mut self, x: usize, y: usize) -> bool {
        if let Some(piece) = &self.current_piece {
            let cells = piece.get_absolute_cells(x, y);
            let cell_type = if self.current_player == 1 {
                Cell::Player1New
            } else {
                Cell::Player2New
            };

            // Place the piece on the anfield
            for (px, py) in &cells {
                if !self.anfield.set_cell(*px, *py, cell_type.clone()) {
                    return false;
                }
            }

            // Update player territory
            self.get_current_player_mut().add_territory(cells);
            self.update_scores();
            
            // Convert new pieces to old pieces for next turn
            self.anfield.update_old_pieces();
            
            true
        } else {
            false
        }
    }
}