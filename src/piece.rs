use rand::Rng;

#[derive(Debug, Clone)]
pub struct Piece {
    pub shape: Vec<(usize, usize)>,
    pub width: usize,
    pub height: usize,
}

impl Piece {
    pub fn new(shape: Vec<(usize, usize)>) -> Self {
        let width = shape.iter().map(|(x, _)| *x).max().unwrap_or(0) + 1;
        let height = shape.iter().map(|(_, y)| *y).max().unwrap_or(0) + 1;
        
        Self { shape, width, height }
    }

    pub fn display(&self) -> String {
        let mut result = String::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                if self.shape.contains(&(x, y)) {
                    result.push('█');
                } else {
                    result.push('.');
                }
                result.push(' ');
            }
            result.push('\n');
        }
        
        result
    }

    // Tetris-like piece shapes
    pub fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        let piece_type = rng.gen_range(0..19);
        
        match piece_type {
            // Single dot
            0 => Self::new(vec![(0, 0)]),
            
            // Line pieces
            1 => Self::new(vec![(0, 0), (1, 0)]), // 2x1 horizontal
            2 => Self::new(vec![(0, 0), (0, 1)]), // 1x2 vertical
            3 => Self::new(vec![(0, 0), (1, 0), (2, 0)]), // 3x1 horizontal
            4 => Self::new(vec![(0, 0), (0, 1), (0, 2)]), // 1x3 vertical
            5 => Self::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]), // 4x1 horizontal
            6 => Self::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]), // 1x4 vertical
            
            // Square pieces
            7 => Self::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]), // 2x2 square
            8 => Self::new(vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]), // 3x2 rectangle
            
            // L-shaped pieces
            9 => Self::new(vec![(0, 0), (0, 1), (0, 2), (1, 2)]), // L
            10 => Self::new(vec![(0, 0), (1, 0), (2, 0), (0, 1)]), // L rotated
            11 => Self::new(vec![(1, 0), (1, 1), (1, 2), (0, 2)]), // L mirrored
            12 => Self::new(vec![(0, 0), (0, 1), (1, 1), (2, 1)]), // L rotated mirrored
            
            // T-shaped pieces
            13 => Self::new(vec![(0, 0), (1, 0), (2, 0), (1, 1)]), // T
            14 => Self::new(vec![(0, 0), (0, 1), (0, 2), (1, 1)]), // T rotated
            15 => Self::new(vec![(1, 0), (0, 1), (1, 1), (2, 1)]), // T upside down
            16 => Self::new(vec![(1, 0), (1, 1), (1, 2), (0, 1)]), // T rotated left
            
            // Z-shaped pieces
            17 => Self::new(vec![(0, 0), (1, 0), (1, 1), (2, 1)]), // Z
            _ => Self::new(vec![(1, 0), (2, 0), (0, 1), (1, 1)]), // Z mirrored
        }
    }

    pub fn generate_sequence(seed: u64) -> PieceGenerator {
        PieceGenerator::new(seed)
    }
}

pub struct PieceGenerator {
    rng: rand::rngs::StdRng,
}

impl PieceGenerator {
    pub fn new(seed: u64) -> Self {
        use rand::SeedableRng;
        Self {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    pub fn next(&mut self) -> Piece {
        let piece_type = self.rng.gen_range(0..19);
        
        match piece_type {
            0 => Piece::new(vec![(0, 0)]),
            1 => Piece::new(vec![(0, 0), (1, 0)]),
            2 => Piece::new(vec![(0, 0), (0, 1)]),
            3 => Piece::new(vec![(0, 0), (1, 0), (2, 0)]),
            4 => Piece::new(vec![(0, 0), (0, 1), (0, 2)]),
            5 => Piece::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            6 => Piece::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            7 => Piece::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
            8 => Piece::new(vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]),
            9 => Piece::new(vec![(0, 0), (0, 1), (0, 2), (1, 2)]),
            10 => Piece::new(vec![(0, 0), (1, 0), (2, 0), (0, 1)]),
            11 => Piece::new(vec![(1, 0), (1, 1), (1, 2), (0, 2)]),
            12 => Piece::new(vec![(0, 0), (0, 1), (1, 1), (2, 1)]),
            13 => Piece::new(vec![(0, 0), (1, 0), (2, 0), (1, 1)]),
            14 => Piece::new(vec![(0, 0), (0, 1), (0, 2), (1, 1)]),
            15 => Piece::new(vec![(1, 0), (0, 1), (1, 1), (2, 1)]),
            16 => Piece::new(vec![(1, 0), (1, 1), (1, 2), (0, 1)]),
            17 => Piece::new(vec![(0, 0), (1, 0), (1, 1), (2, 1)]),
            _ => Piece::new(vec![(1, 0), (2, 0), (0, 1), (1, 1)]),
        }
    }
}
