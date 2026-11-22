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
mod ui;

use ghosts::Ghost;
use maps::{
    MapGrid,
    render_map,
    is_win,
    get_map
};
use player::{
    Player
};
use ui::{
    main_menu
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
        is_power_mode: false,
        power_mode_duration: 0,
        last_move: ' '
    };

    // ghost declaration
    let mut ghost: Ghost = Ghost{
        icon: 'G',
        pos_x: 7,
        pos_y: 5,
        live: true
    };


    let mut current_level: u8 = 1;
    let mut current_map: MapGrid = get_map(current_level);

    // let map: MapGrid = [
    //     ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
    //     ['#','o','.','.','.','.','.','#','.','.','.','.','.','o','#'],
    //     ['#','.','#','#','.','#','.','#','.','#','.','#','#','.','#'],
    //     ['#','.','.','.','.','#','.','.','.','#','.','.','.','.','#'],
    //     ['#','.','#','#','.','#','#',' ','#','#','.','#','#','.','#'],
    //     ['#','.','.','.','.','#',' ',' ',' ','#','.','.','.','.','#'],
    //     ['#','.','#','#','.','#','#','#','#','#','.','#','#','.','#'],
    //     ['#','.','.','.','.','.','.',' ','.','.','.','.','.','.','#'],
    //     ['#','.','#','#','.','#','#','#','#','#','.','#','#','.','#'],
    //     ['#','o','.','.','.','.','.','#','.','.','.','.','.','o','#'],
    //     ['#','#','#','#','#','#','#','#','#','#','#','#','#','#','#'],
    // ];

    // let mut current_map: MapGrid;

    
    let mut win: bool = false;

    let mut last_move: char= ' ';

    let mut warning_massage: &str;


    // input movement
    let mut input_key: char = 'd';

    println!("RustyMan");

    if main_menu() {
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
        disable_raw_mode().unwrap();
        return;
    }

    execute!(stdout(), Clear(ClearType::All)).unwrap();
    
    loop {
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();

        println!("===============================");

        // render map
        render_map(&mut current_map, &mut pacman, &mut ghost);
        // render map/
        
        // stats
        println!("===============================");
        println!("player : {} score : {} lives : {}", pacman.icon, pacman.score, pacman.lives);
        println!("position");
        println!("pos_x : {} pos_y : {}", pacman.pos_x , pacman.pos_y);
        println!("Power Mode : {}", pacman.is_power_mode);
        // stats/

        // this whole block in bottom is just for warning massage
        println!("===============================");
        let dist: usize = ghost.pos_x.abs_diff(pacman.pos_x) + ghost.pos_y.abs_diff(pacman.pos_y);
        println!("Ghost Detector: ");
        if dist <= 4 && ghost.live {
            warning_massage = "WARNING GHOST CLOSE DETECTED";
            println!("{}", warning_massage.red().bold());
        } else if dist <= 6 && ghost.live {
            warning_massage = "WARNING GHOST GETTING CLOSE";
            println!("{}    ", warning_massage.yellow().bold());
        } else {
            warning_massage = "OK";
            println!("{}                         ", warning_massage.green().bold());
        }
        println!("===============================");
        // warning massage/

        // win condition handling
        if is_win(&current_map) {
            win = true;
            break
        }
        if pacman.lives == 0 {
            break;
        }
        // win handling/

        // read user input
        if let Some(c) = read_input() {
            if c == 'q' {break;}
            match c {
                'w' | 'a' | 's' | 'd' => last_move = c,
                _ => {}
            }
        }
        // read user input/
        
        // player movement
        if last_move != ' ' {
            pacman.movement(last_move, &current_map);
        }
        // player mobement/

        // ghost collision check
        pacman.check_ghost_collision(&mut ghost);
        // ghost movement 
        if ghost.live{ghost.movement(&current_map);} 
        pacman.check_ghost_collision(&mut ghost);

        

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

        println!("YOUR SCORE: {}", pacman.score);
        

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





    





pub fn read_input() -> Option<char> {
    // its for handling input so fucking long
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => return Some(c),
                        KeyCode::Enter => return Some('\n'),
                        _ => {}
                    }
                }
            }
        }
        None
}

