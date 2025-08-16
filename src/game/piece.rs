#[derive(Debug, Clone)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub shape: Vec<Vec<bool>>, // true = piece cell, false = empty
}

impl Piece {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            shape: vec![vec![false; width]; height],
        }
    }

    pub fn from_pattern(pattern: Vec<String>) -> Self {
        let height = pattern.len();
        let width = pattern.get(0).map(|s| s.len()).unwrap_or(0);
        
        let mut shape = vec![vec![false; width]; height];
        
        for (y, row) in pattern.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if x < width && y < height {
                    shape[y][x] = ch != '.';
                }
            }
        }

        Self {
            width,
            height,
            shape,
        }
    }

    pub fn get_piece_cells(&self) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.shape[y][x] {
                    cells.push((x, y));
                }
            }
        }
        cells
    }

    pub fn has_cell_at(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.shape[y][x]
        } else {
            false
        }
    }

    pub fn get_absolute_cells(&self, offset_x: usize, offset_y: usize) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        for (rel_x, rel_y) in self.get_piece_cells() {
            cells.push((offset_x + rel_x, offset_y + rel_y));
        }
        cells
    }

    pub fn count_cells(&self) -> usize {
        self.get_piece_cells().len()
    }
}