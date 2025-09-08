pub struct GamePiece {
    pub shape: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl GamePiece {
    pub fn new(shape: Vec<Vec<char>>, width: usize, height: usize) -> Self {
        GamePiece { shape, width, height }
    }
    
    pub fn get_active_cells(&self) -> Vec<(usize, usize)> {
        let mut active_cells = Vec::new();
        
        for (row_idx, row) in self.shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == 'O' || cell == '#' {
                    active_cells.push((col_idx, row_idx));
                }
            }
        }
        
        active_cells
    }
}
