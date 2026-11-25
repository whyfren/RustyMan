//libraries
#[cfg(not(target_arch = "wasm32"))] use rand;

// fromm other file
use crate::maps::MapGrid;
use crate::player::{Player};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum GhostType {
    Chaser,
    Patrol,
}


pub struct Ghost{
    pub icon: char,
    pub pos_x: usize,
    pub pos_y: usize,
    pub live: bool,
    pub last_dir: Direction,
    pub ghost_type: GhostType,
    pub respawn_timer: u8,

}

impl Direction {
    pub fn opposite(&mut self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Ghost {
    pub fn movement (&mut self, map: &MapGrid, player: &mut Player){

        



        let all_dirs = [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left
        ];

        let mut valid_moves: Vec<(Direction, usize, usize)> = Vec::new();

        let mut candidate_moves: Vec<(Direction, usize, usize, usize)> = Vec::new();

        
        let chosen_move: Option<&(Direction, usize, usize, usize)>;

        let opp = self.last_dir.opposite();

        for dir in all_dirs {
            let mut next_x = self.pos_x;
            let mut next_y = self.pos_y;

            match dir {
                Direction::Down => {next_y += 1}
                Direction::Up => {next_y -= 1}
                Direction::Left => {next_x -= 1}
                Direction::Right => {next_x += 1}
            }

            if map[next_y][next_x] != '#' && dir != opp {
                let dist = next_x.abs_diff(player.pos_x) + next_y.abs_diff(player.pos_y);
                candidate_moves.push((dir, next_x, next_y, dist));
            }
        }

        if candidate_moves.len() > 1 {
            let opp = self.last_dir.opposite();
            candidate_moves.retain(|item| item.0 != opp);
        }


        if player.is_power_mode {
            chosen_move = candidate_moves.iter().max_by_key(|item| item.3);
        } else {
            if self.ghost_type == GhostType::Chaser {
                chosen_move = candidate_moves.iter().min_by_key(|item| item.3);
            } else if self.ghost_type == GhostType::Patrol {
                let len = candidate_moves.len();

                #[cfg(not(target_arch = "wasm32"))]
                let rand: usize = if len > 0 { rand::random_range(0..len) } else { 0 };

                #[cfg(target_arch = "wasm32")]
                let rand: usize = (js_sys::Math::random()*len as f64) as usize;

                chosen_move = candidate_moves.get(rand)
            } else {
                chosen_move = None;
            }
        }

        if let Some(&(chosen_dir, nx, ny, _dist)) = chosen_move {
            self.pos_x = nx;
            self.pos_y = ny;
            self.last_dir = chosen_dir;
        }
    }
}