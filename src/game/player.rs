#[derive(Debug, Clone)]
pub struct Player {
    pub id: u8,
    pub territory: Vec<(usize, usize)>,
    pub score: u32,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            territory: Vec::new(),
            score: 0,
        }
    }

    pub fn add_territory(&mut self, positions: Vec<(usize, usize)>) {
        for pos in positions {
            if !self.territory.contains(&pos) {
                self.territory.push(pos);
                self.score += 1;
            }
        }
    }

    pub fn has_territory_at(&self, x: usize, y: usize) -> bool {
        self.territory.contains(&(x, y))
    }

    pub fn get_territory_cells(&self) -> &Vec<(usize, usize)> {
        &self.territory
    }

    pub fn update_score(&mut self, new_score: u32) {
        self.score = new_score;
    }

    pub fn clear_territory(&mut self) {
        self.territory.clear();
        self.score = 0;
    }

    pub fn is_adjacent_to_territory(&self, x: usize, y: usize) -> bool {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                
                if self.has_territory_at(new_x, new_y) {
                    return true;
                }
            }
        }
        false
    }
}