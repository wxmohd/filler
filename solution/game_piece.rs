// Game piece representation used across the engine and AI
// Note: Several modules access these fields directly, so they must be public.
#[derive(Debug, Clone)]
pub struct GamePiece {
    pub shape: Vec<Vec<char>>, // raw piece grid ('.' empty, 'O'/'#'/'*' active)
    pub width: usize,
    pub height: usize,
}

impl GamePiece {
    pub fn new(shape: Vec<Vec<char>>, width: usize, height: usize) -> Self {
        GamePiece { shape, width, height }
    }

    /// Return coordinates of active cells within the piece-local grid
    pub fn get_active_cells(&self) -> Vec<(usize, usize)> {
        let mut active = Vec::new();
        for (r, row) in self.shape.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if is_active_cell(ch) { active.push((c, r)); }
            }
        }
        active
    }
}

#[inline]
pub fn is_active_cell(c: char) -> bool { matches!(c, 'O' | '#' | '*') }
