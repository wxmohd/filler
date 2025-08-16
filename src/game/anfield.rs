#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Player1Old,     // '@'
    Player1New,     // 'a'
    Player2Old,     // '$'
    Player2New,     // 's'
}

impl Cell {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '.' => Cell::Empty,
            '@' => Cell::Player1Old,
            'a' => Cell::Player1New,
            '$' => Cell::Player2Old,
            's' => Cell::Player2New,
            _ => Cell::Empty,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Player1Old => '@',
            Cell::Player1New => 'a',
            Cell::Player2Old => '$',
            Cell::Player2New => 's',
        }
    }

    pub fn is_player(&self, player_id: u8) -> bool {
        match (self, player_id) {
            (Cell::Player1Old | Cell::Player1New, 1) => true,
            (Cell::Player2Old | Cell::Player2New, 2) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Anfield {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
}

impl Anfield {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![Cell::Empty; width]; height],
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.grid.get(y)?.get(x)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) -> bool {
        if let Some(row) = self.grid.get_mut(y) {
            if let Some(target_cell) = row.get_mut(x) {
                *target_cell = cell;
                return true;
            }
        }
        false
    }

    pub fn is_valid_position(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_player_territory(&self, player_id: u8) -> Vec<(usize, usize)> {
        let mut territory = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get_cell(x, y) {
                    if cell.is_player(player_id) {
                        territory.push((x, y));
                    }
                }
            }
        }
        territory
    }

    pub fn count_player_cells(&self, player_id: u8) -> u32 {
        let mut count = 0;
        for row in &self.grid {
            for cell in row {
                if cell.is_player(player_id) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn update_old_pieces(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                match cell {
                    Cell::Player1New => *cell = Cell::Player1Old,
                    Cell::Player2New => *cell = Cell::Player2Old,
                    _ => {}
                }
            }
        }
    }
}