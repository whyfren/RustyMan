// libraries
use std::io::stdout;
use std::time::Duration;
use crossterm::{
    cursor::{Hide, MoveTo, Show}, 
    event::{self, Event, KeyCode, KeyEventKind}, 
    style::Stylize,
    execute, 
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};

// from other file
mod ghosts;
mod maps;
mod player;

use ghosts::Ghost;
use maps::{
    MapGrid,
    render_map,
    is_win
};
use player::{
    Player
};




fn main() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide).unwrap();

    //player variable declaration
    let mut pacman = Player{
        icon: 'C',
        pos_x: 7,
        pos_y: 7,
        lives: 3,
        score: 0,
        is_power_mode: false
    };

    let mut ghost: Ghost = Ghost{
        icon: 'G',
        pos_x: 7,
        pos_y: 5,
        live: true
    };

    let map: MapGrid = [
        ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
        ['#','o','.','.','.','.','.','#','.','.','.','.','.','o','#'],
        ['#','.','#','#','.','#','.','#','.','#','.','#','#','.','#'],
        ['#','.','.','.','.','#','.','.','.','#','.','.','.','.','#'],
        ['#','.','#','#','.','#','#',' ','#','#','.','#','#','.','#'],
        ['#','.','.','.','.','#',' ',' ',' ','#','.','.','.','.','#'],
        ['#','.','#','#','.','#','#','#','#','#','.','#','#','.','#'],
        ['#','.','.','.','.','.','.',' ','.','.','.','.','.','.','#'],
        ['#','.','#','#','.','#','#','#','#','#','.','#','#','.','#'],
        ['#','o','.','.','.','.','.','#','.','.','.','.','.','o','#'],
        ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
    ];
    let mut current_map: MapGrid;

    
    let mut win: bool = false;

    let mut last_move: char= ' ';


    // input movement
    let mut input_key: char = 'd';

    //this is creating map that dynamic, ex the pellet is eaten and gone
    current_map = map;

    println!("RustyMan");
    
    loop {
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();

        println!("===============================");

        render_map(&mut current_map, &mut pacman, &mut ghost);

        if let Some(c) = read_input() {
            if c == 'q' {break;}
            match c {
                'w' => last_move = 'w',
                'a' => last_move = 'a',
                's' => last_move = 's',
                'd' => last_move = 'd',
                _ => last_move,
            }

            pacman.movement(last_move, &current_map);
        }

        pacman.check_ghost_collision(&mut ghost);
        if ghost.live{ghost.movement(&map);} 

        println!("===============================");

        println!("icon : {} score : {} lives : {}", pacman.icon, pacman.score, pacman.lives);
        println!("position");
        println!("pos_x : {} pos_y : {}", pacman.pos_x , pacman.pos_y);
        println!("Power Mode : {}", pacman.is_power_mode);

        if is_win(&current_map) {
            win = true;
            break
        }
        if pacman.lives == 0 {
            break;
        }
        // pacman.check_power_pellet();
    }

    execute!(stdout(), Clear(ClearType::All)).unwrap();

    loop {
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();
        println!("===============================");

        let win_massage: &str;

        if win {
            win_massage = "YOU WIN";
            println!("{}", win_massage.yellow())
        } else {
            win_massage = "YOU lOSE";
            println!("{}", win_massage.red())
        }
        

        println!(" ");
        println!("PRESS q TO EXIT");

        if let Some(c) = read_input() {
            if c == 'q' {break;}
        }
        println!("===============================");
    }
    execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    disable_raw_mode().unwrap();
}









fn read_input() -> Option<char> {
    // its for handling input so fucking long
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    if let KeyCode::Char(c) = key_event.code {
                        return Some(c);
                    }
                }
            }
        }
        None
}

