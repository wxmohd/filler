use crate::game_state::GameState;
use crate::game_piece::GamePiece;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

pub struct StrategyEngine {
    player_symbols: (char, char), // (stable, last-piece-lowercase)
    enemy_symbols: (char, char),
}

impl StrategyEngine {
    pub fn new(player_id: u8) -> Self {
        let (player_symbols, enemy_symbols) = if player_id == 1 {
            (('@', 'a'), ('$', 's'))
        } else {
            (('$', 's'), ('@', 'a'))
        };
        StrategyEngine { player_symbols, enemy_symbols }
    }

    pub fn find_optimal_move(&self, state: &GameState, piece: &GamePiece) -> (usize, usize) {
        // Phase info
        let total_cells = state.width * state.height;
        let enemy_territory = self.count_enemy_territory(state);
        let occupied = state.my_territory.len() + enemy_territory;
        let progress = (occupied as f64) / (total_cells.max(1) as f64);

        let is_large_map = total_cells > 2000;
        let early_cut = if is_large_map { 0.20 } else { 0.35 };
        let late_cut = 0.75;
        let phase = if progress < early_cut { Phase::Early } else if progress > late_cut { Phase::Late } else { Phase::Mid };

        // Multi-source BFS distances once per turn
        let d_my = self.distance_from_territory(state, &state.my_territory);
        let d_enemy = self.distance_from_enemy(state);

        // Detect “terminator-like” dominance (Voronoi margin test)
        let (margin, empties) = self.voronoi_margin(state, &d_my, &d_enemy);
        // Dynamic thresholds: trigger only when enemy is clearly ahead
        let hard_mode = match phase {
            Phase::Early => margin <= -(empties as i64 / 8).max(8),  // ~12.5% lead
            Phase::Mid   => margin <= -(empties as i64 / 6).max(12), // ~16.7% lead
            Phase::Late  => false,
        };

        // Enumerate candidates
        let mut candidates: Vec<(usize, usize, LexKey)> = Vec::new();
        for row in 0..state.height {
            for col in 0..state.width {
                if self.is_placement_valid(state, piece, col, row) {
                    let key = self.evaluate_move_key(state, piece, col, row, phase, hard_mode, &d_my, &d_enemy);
                    candidates.push((col, row, key));
                }
            }
        }

        if candidates.is_empty() { return (0, 0); }

        // Deterministic sort by lexicographic key; top-left on ties
        candidates.sort_by(|a, b|
            compare_keys(&b.2, &a.2)               // metrics desc
                .then_with(|| a.1.cmp(&b.1))       // row asc
                .then_with(|| a.0.cmp(&b.0))       // col asc
        );

        let (x, y, _) = candidates[0];
        (x, y)
    }

    // ==================== Legality ====================

    fn is_placement_valid(&self, state: &GameState, piece: &GamePiece, start_x: usize, start_y: usize) -> bool {
        let mut overlaps = 0usize;

        for (pr, row) in piece.shape.iter().enumerate() {
            for (pc, &cell) in row.iter().enumerate() {
                if !is_piece_cell(cell) { continue; }
                let x = start_x + pc;
                let y = start_y + pr;
                if x >= state.width || y >= state.height { return false; }

                let b = state.board[y][x];

                // enemy collision not allowed
                if b == self.enemy_symbols.0 || b == self.enemy_symbols.1 { return false; }

                // count own overlap
                if b == self.player_symbols.0 || b == self.player_symbols.1 {
                    overlaps += 1;
                    if overlaps > 1 { return false; }
                }
            }
        }
        overlaps == 1
    }

    // ==================== Scoring ====================

    fn evaluate_move_key(
        &self,
        state: &GameState,
        piece: &GamePiece,
        start_x: usize,
        start_y: usize,
        phase: Phase,
        hard_mode: bool,
        d_my: &Vec<Vec<i32>>,
        d_enemy: &Vec<Vec<i32>>,
    ) -> LexKey {
        let mut new_claims: Vec<(usize, usize)> = Vec::new();

        for (pr, row) in piece.shape.iter().enumerate() {
            for (pc, &cell) in row.iter().enumerate() {
                if !is_piece_cell(cell) { continue; }
                let x = start_x + pc;
                let y = start_y + pr;
                if state.board[y][x] == '.' {
                    new_claims.push((x, y));
                }
            }
        }

        // 1) Cells captured (primary everywhere)
        let k1_cells_captured: i64 = new_claims.len() as i64;

        // 2) Voronoi gain (larger radius in hard mode)
        let voronoi_r = if hard_mode { 6 } else { 4 };
        let k2_voronoi_gain: i64 = self.local_voronoi_gain(state, &new_claims, d_my, d_enemy, voronoi_r) as i64;

        // 3) Enemy blocking (adjacent enemy contacts)
        let k3_enemy_block: i64 = self.enemy_adjacent_count(state, &new_claims) as i64;

        // 4) Cohesion (closer to our frontier = better)
        let k4_cohesion: i64 = self.cohesion_score_from_distances(&new_claims, d_my);

        // 5) Context: Early/Mid → edge/corner; Late → local empties
        let k5_context: i64 = match phase {
            Phase::Early | Phase::Mid => self.corner_edge_pressure(state, &new_claims),
            Phase::Late               => self.local_empty_potential(state, &new_claims) as i64,
        };

        // ---- Hard-mode aggressors (only when triggered) ----
        let mut k6_invasion: i64 = 0; // how many new claims were enemy Voronoi
        let mut k7_squeeze: i64  = 0; // local enemy mobility reduction

        if hard_mode && !new_claims.is_empty() {
            k6_invasion = self.invasion_score(&new_claims, d_my, d_enemy) as i64;
            k7_squeeze  = self.squeeze_score(state, &new_claims, d_enemy, 4) as i64;
        }

        // Phase ordering (lexicographic). In hard mode, push control terms up.
        let (k2_primary, k3_secondary, k4_tertiary, k5_quaternary, k6_quinary, k7_senary) = if hard_mode {
            match phase {
                Phase::Early => (k2_voronoi_gain, k6_invasion, k3_enemy_block, k1_cells_captured, k7_squeeze, k4_cohesion),
                Phase::Mid   => (k2_voronoi_gain, k6_invasion, k7_squeeze, k3_enemy_block, k4_cohesion, k1_cells_captured),
                Phase::Late  => (k2_voronoi_gain, k4_cohesion, k3_enemy_block, k5_context,      k6_invasion, k7_squeeze),
            }
        } else {
            match phase {
                Phase::Early => (k2_voronoi_gain, k5_context,   k3_enemy_block, k4_cohesion, 0, 0),
                Phase::Mid   => (k2_voronoi_gain, k3_enemy_block, k4_cohesion,  k5_context,  0, 0),
                Phase::Late  => (k2_voronoi_gain, k4_cohesion,   k3_enemy_block, k5_context, 0, 0),
            }
        };

        // Stable tie-breakers: prefer top-left
        let t_row = -(start_y as i64);
        let t_col = -(start_x as i64);

        LexKey {
            k1_cells_captured,
            k2_primary,
            k3_secondary,
            k4_tertiary,
            k5_quaternary: k5_quaternary,
            k6_quinary,
            k7_senary,
            t_row,
            t_col,
        }
    }

    fn count_enemy_territory(&self, state: &GameState) -> usize {
        let mut count = 0usize;
        for row in &state.board {
            for &c in row {
                if c == self.enemy_symbols.0 || c == self.enemy_symbols.1 { count += 1; }
            }
        }
        count
    }

    // ==================== Distances & Voronoi ====================

    fn distance_from_territory(&self, state: &GameState, territory: &Vec<(usize, usize)>) -> Vec<Vec<i32>> {
        let w = state.width;
        let h = state.height;
        let inf = 1_000_000i32;
        let mut dist = vec![vec![inf; w]; h];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        for &(tx, ty) in territory {
            for (nx, ny) in four_neighbors(tx, ty, w, h) {
                if state.board[ny][nx] == '.' && dist[ny][nx] == inf {
                    dist[ny][nx] = 1;
                    q.push_back((nx, ny));
                }
            }
        }
        while let Some((x, y)) = q.pop_front() {
            let d = dist[y][x];
            for (nx, ny) in four_neighbors(x, y, w, h) {
                if state.board[ny][nx] != '.' { continue; }
                if dist[ny][nx] > d + 1 {
                    dist[ny][nx] = d + 1;
                    q.push_back((nx, ny));
                }
            }
        }
        dist
    }

    fn distance_from_enemy(&self, state: &GameState) -> Vec<Vec<i32>> {
        let w = state.width;
        let h = state.height;
        let inf = 1_000_000i32;
        let mut dist = vec![vec![inf; w]; h];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        for y in 0..h {
            for x in 0..w {
                let c = state.board[y][x];
                if c == self.enemy_symbols.0 || c == self.enemy_symbols.1 {
                    for (nx, ny) in four_neighbors(x, y, w, h) {
                        if state.board[ny][nx] == '.' && dist[ny][nx] == inf {
                            dist[ny][nx] = 1;
                            q.push_back((nx, ny));
                        }
                    }
                }
            }
        }
        while let Some((x, y)) = q.pop_front() {
            let d = dist[y][x];
            for (nx, ny) in four_neighbors(x, y, w, h) {
                if state.board[ny][nx] != '.' { continue; }
                if dist[ny][nx] > d + 1 {
                    dist[ny][nx] = d + 1;
                    q.push_back((nx, ny));
                }
            }
        }
        dist
    }

    fn voronoi_margin(&self, state: &GameState, d_my: &Vec<Vec<i32>>, d_enemy: &Vec<Vec<i32>>) -> (i64, usize) {
        let mut myc = 0usize;
        let mut enc = 0usize;
        let mut empties = 0usize;
        for y in 0..state.height {
            for x in 0..state.width {
                if state.board[y][x] != '.' { continue; }
                empties += 1;
                let dm = d_my[y][x];
                let de = d_enemy[y][x];
                if dm < de { myc += 1; }
                else if de < dm { enc += 1; }
            }
        }
        (myc as i64 - enc as i64, empties)
    }

    fn local_voronoi_gain(
        &self,
        state: &GameState,
        new_claims: &[(usize, usize)],
        d_my: &Vec<Vec<i32>>,
        d_enemy: &Vec<Vec<i32>>,
        radius: i32,
    ) -> i32 {
        if new_claims.is_empty() { return 0; }
        let w = state.width as i32;
        let h = state.height as i32;
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut gain = 0i32;

        for &(cx, cy) in new_claims {
            let cx = cx as i32; let cy = cy as i32;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = cx + dx; let ny = cy + dy;
                    if nx < 0 || ny < 0 || nx >= w || ny >= h { continue; }
                    let nxu = nx as usize; let nyu = ny as usize;
                    if state.board[nyu][nxu] != '.' { continue; }
                    if !seen.insert((nxu, nyu)) { continue; }

                    let before_my = d_my[nyu][nxu];
                    let enemy_d   = d_enemy[nyu][nxu];

                    let from_claim = (dx.abs() + dy.abs()) as i32; // distance via our new seed
                    let after_my   = before_my.min(from_claim);

                    let before_was_mine = before_my < enemy_d;
                    let after_is_mine   = after_my < enemy_d;
                    if !before_was_mine && after_is_mine { gain += 1; }
                }
            }
        }
        gain
    }

    // Cells we claim that were previously enemy Voronoi
    fn invasion_score(&self, new_claims: &[(usize, usize)], d_my: &Vec<Vec<i32>>, d_enemy: &Vec<Vec<i32>>) -> i32 {
        let mut inv = 0i32;
        for &(x, y) in new_claims {
            let dm = d_my[y][x];
            let de = d_enemy[y][x];
            if de <= dm { inv += 1; } // planting inside enemy-favored area
        }
        inv
    }

    // Local “squeeze”: enemy-favored empties whose 4-neighbor degree drops after we occupy new_claims.
    fn squeeze_score(
        &self,
        state: &GameState,
        new_claims: &[(usize, usize)],
        d_enemy: &Vec<Vec<i32>>,
        radius: i32,
    ) -> i32 {
        if new_claims.is_empty() { return 0; }
        let w = state.width as i32;
        let h = state.height as i32;
        let new_set: HashSet<(usize, usize)> = new_claims.iter().cloned().collect();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut score = 0i32;

        for &(cx, cy) in new_claims {
            let cx = cx as i32; let cy = cy as i32;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = cx + dx; let ny = cy + dy;
                    if nx < 0 || ny < 0 || nx >= w || ny >= h { continue; }
                    let nxu = nx as usize; let nyu = ny as usize;
                    if state.board[nyu][nxu] != '.' { continue; }
                    if !seen.insert((nxu, nyu)) { continue; }

                    let enemy_favored = d_enemy[nyu][nxu] < 1_000_000;
                    if !enemy_favored { continue; }

                    let mut deg_before = 0;
                    let mut deg_after  = 0;
                    for (qx, qy) in four_neighbors(nxu, nyu, state.width, state.height) {
                        if state.board[qy][qx] == '.' {
                            deg_before += 1;
                            if !new_set.contains(&(qx, qy)) {
                                deg_after += 1;
                            }
                        }
                    }
                    if deg_after < deg_before {
                        score += (deg_before - deg_after) as i32;
                    }
                }
            }
        }
        score
    }

    // ==================== Simpler signals ====================

    fn enemy_adjacent_count(&self, state: &GameState, new_claims: &[(usize, usize)]) -> i32 {
        let mut count = 0i32;
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for &(x, y) in new_claims {
            for (nx, ny) in four_neighbors(x, y, state.width, state.height) {
                if !seen.insert((nx, ny)) { continue; }
                let c = state.board[ny][nx];
                if c == self.enemy_symbols.0 || c == self.enemy_symbols.1 { count += 1; }
            }
        }
        count
    }

    fn cohesion_score_from_distances(&self, new_claims: &[(usize, usize)], d_my: &Vec<Vec<i32>>) -> i64 {
        if new_claims.is_empty() { return 0; }
        let mut best = i64::MIN;
        for &(x, y) in new_claims {
            let d = d_my[y][x];
            let score = 1_000i64.saturating_sub(d as i64); // closer → larger
            if score > best { best = score; }
        }
        best
    }

    fn corner_edge_pressure(&self, state: &GameState, new_claims: &[(usize, usize)]) -> i64 {
        if new_claims.is_empty() { return 0; }
        let w = state.width;
        let h = state.height;
        let mut touches_edge = 0i64;
        let mut best_corner: i64 = i64::MIN;
        for &(x, y) in new_claims {
            if x == 0 || y == 0 || x + 1 == w || y + 1 == h { touches_edge = 1; }
            let d0 = manhattan(x, y, 0, 0);
            let d1 = manhattan(x, y, 0, h - 1);
            let d2 = manhattan(x, y, w - 1, 0);
            let d3 = manhattan(x, y, w - 1, h - 1);
            let min_d = d0.min(d1).min(d2).min(d3) as i64;
            let base = (w as i64 + h as i64).saturating_sub(min_d);
            if base > best_corner { best_corner = base; }
        }
        touches_edge * 1000 + best_corner
    }

    fn local_empty_potential(&self, state: &GameState, new_claims: &[(usize, usize)]) -> i32 {
        let mut s: HashSet<(usize, usize)> = HashSet::new();
        for &(x, y) in new_claims {
            for (nx, ny) in eight_neighbors(x, y, state.width, state.height) {
                if state.board[ny][nx] == '.' { s.insert((nx, ny)); }
            }
        }
        s.len() as i32
    }
}

/* ===================== Helpers & types ===================== */

#[derive(Clone, Copy)]
enum Phase { Early, Mid, Late }

#[derive(Debug, Clone)]
struct LexKey {
    k1_cells_captured: i64,
    k2_primary: i64,
    k3_secondary: i64,
    k4_tertiary: i64,
    k5_quaternary: i64,
    k6_quinary: i64,
    k7_senary: i64,
    t_row: i64, // -(row)
    t_col: i64, // -(col)
}

fn compare_keys(a: &LexKey, b: &LexKey) -> Ordering {
    a.k1_cells_captured.cmp(&b.k1_cells_captured)
        .then_with(|| a.k2_primary.cmp(&b.k2_primary))
        .then_with(|| a.k3_secondary.cmp(&b.k3_secondary))
        .then_with(|| a.k4_tertiary.cmp(&b.k4_tertiary))
        .then_with(|| a.k5_quaternary.cmp(&b.k5_quaternary))
        .then_with(|| a.k6_quinary.cmp(&b.k6_quinary))
        .then_with(|| a.k7_senary.cmp(&b.k7_senary))
        .then_with(|| a.t_row.cmp(&b.t_row))
        .then_with(|| a.t_col.cmp(&b.t_col))
}

#[inline]
fn is_piece_cell(c: char) -> bool { matches!(c, 'O' | '#' | '*') }

#[inline]
fn manhattan(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

#[inline]
fn four_neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    DIRS.into_iter().filter_map(move |(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < w as i32 && ny < h as i32 {
            Some((nx as usize, ny as usize))
        } else { None }
    })
}

#[inline]
fn eight_neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    const DIRS8: [(i32, i32); 8] = [
        (1, 0), (-1, 0), (0, 1), (0, -1),
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];
    DIRS8.into_iter().filter_map(move |(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < w as i32 && ny < h as i32 {
            Some((nx as usize, ny as usize))
        } else { None }
    })
}
