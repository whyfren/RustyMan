use crate::maps::MapGrid;
use crate::ghosts::Ghost;

pub struct Player{
    pub icon: char,
    pub pos_x: usize,
    pub pos_y: usize,
    pub lives: u8,
    pub score: i32,
    pub is_power_mode: bool,
    pub power_mode_duration: usize,
    pub last_move: char
}




impl Player {

    pub fn movement(&mut self, input_key: char, map: &MapGrid) {
        let mut next_x = self.pos_x;
        let mut next_y = self.pos_y;
        if self.is_power_mode {
            if self.power_mode_duration == 0 {
                self.is_power_mode = false;
            } else {
                self.power_mode_duration -= 1;
            }
            
        }
        
            match input_key{
                'w' => next_y -= 1,
                's' => next_y += 1,
                'a' => next_x -= 1,
                'd' => next_x += 1,
                _ => println!("input not recognized")
            }
            if map[next_y][next_x] != '#' {
                self.pos_x = next_x;
                self.pos_y = next_y;
                self.last_move = input_key
            } else {
                next_x = self.pos_x;
                next_y = self.pos_y;
                match self.last_move{
                    'w' => next_y -= 1,
                    's' => next_y += 1,
                    'a' => next_x -= 1,
                    'd' => next_x += 1,
                    _ => println!("input not recognized")
                }  
                if map[next_y][next_x] != '#' {
                self.pos_x = next_x;
                self.pos_y = next_y;
            }
            }
        
        
        
        
    }

    pub fn check_power_pellet(&mut self, map: &MapGrid) {
        if map[self.pos_y][self.pos_x] == 'o'{
            self.is_power_mode = true;
            self.power_mode_duration = 30;
            self.score += 50;
        } if map[self.pos_y][self.pos_x] == '.'{
            self.score += 10;
        }
    }

    pub fn check_ghost_collision(&mut self, ghost: &mut Ghost){
        if ghost.live && ghost.pos_x == self.pos_x && ghost.pos_y == self.pos_y{
            if self.is_power_mode {
                        ghost.live = false;
                        ghost.respawn_timer = 1;
                        self.score += 200;
                    } else {
                        self.lives -= 1;
                        self.pos_x = 7;
                        self.pos_y = 7;
                    }

        }
    }
}