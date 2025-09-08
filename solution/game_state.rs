pub struct GameState {
    pub board: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub player_id: u8,
    pub my_territory: Vec<(usize, usize)>,
}

impl GameState {
    pub fn new(board: Vec<Vec<char>>, width: usize, height: usize, player_id: u8) -> Self {
        let territory_chars = if player_id == 1 { ['@', 'a'] } else { ['$', 's'] };
        let mut my_territory = Vec::new();
        
        for (y, row) in board.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if territory_chars.contains(&cell) {
                    my_territory.push((x, y));
                }
            }
        }
        
        GameState {
            board,
            width,
            height,
            player_id,
            my_territory,
        }
    }
}
