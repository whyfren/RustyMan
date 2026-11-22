// libraries
use crossterm::style::Stylize;

use crate::ghosts::Ghost;
use crate::player::Player;

pub const MAP_HEIGHT: usize = 11;
pub const MAP_WIDTH: usize = 15;
pub type MapGrid = [[char; MAP_WIDTH]; MAP_HEIGHT];




pub fn render_map(current_map: &mut MapGrid, player: &mut Player, ghost: &mut Ghost) {
    
    for y in 0..MAP_HEIGHT {

        for x in 0..MAP_WIDTH{ 
    
            if player.pos_x == x && player.pos_y == y {

                print!("{} ", player.icon.yellow().bold());
                player.check_power_pellet(current_map);
                current_map[y][x] = ' ';


            } else if ghost.pos_x == x && ghost.pos_y == y && ghost.live{
            
                if player.is_power_mode {
                    print!("{} ", ghost.icon.cyan().bold());
                } else {
                    print!("{} ", ghost.icon.red().bold());
                }

            } else {
                match current_map[y][x]{
                    '#' => print!("{} ", current_map[y][x].blue().bold()),
                    '.' => print!("{} ", current_map[y][x].white()),
                    'o' => print!("{} ", current_map[y][x].magenta().bold()),
                    _ => print!("  ")
                }
            }

        }
    println!("");
    }
}


pub fn is_win(map: &MapGrid) -> bool {
    for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if map[y][x] == '.' || map[y][x] == 'o' {
                    return false;
                } else {
                }
            }   
    }
    true
}