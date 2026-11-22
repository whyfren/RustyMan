use std::io::stdout;
use std::time::Duration;
use crossterm::{
    cursor::{Hide, MoveTo, Show}, 
    event::{self, Event, KeyCode, KeyEventKind}, 
    style::Stylize,
    execute, 
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};
use rand;

const MAP_HEIGHT: usize = 11;
const MAP_WIDTH: usize = 15;
type MapGrid = [[char; MAP_WIDTH]; MAP_HEIGHT];

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

    
    let mut win: bool = false;

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
            pacman.movement(c, &current_map);
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



struct Player{
    icon: char,
    pos_x: usize,
    pos_y: usize,
    lives: u8,
    score: i32,
    is_power_mode: bool
}

struct Ghost{
    icon: char,
    pos_x: usize,
    pos_y: usize,
    live: bool
}



impl Player {

    fn movement(&mut self, input_key: char, map: &MapGrid) {
        let mut next_x = self.pos_x;
        let mut next_y = self.pos_y;
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
        }
    }

    fn check_power_pellet(&mut self, map: &MapGrid) {
        if map[self.pos_y][self.pos_x] == 'o'{
            self.is_power_mode = true;
            self.score += 50;
        } if map[self.pos_y][self.pos_x] == '.'{
            self.score += 10;
        } 
    }

    fn check_ghost_collision(&mut self, ghost: &mut Ghost){
        if ghost.live && ghost.pos_x == self.pos_x && ghost.pos_y == self.pos_y{
            if self.is_power_mode {
                        ghost.live = false;
                        self.score += 200;
                    } else {
                        self.lives -= 1;
                        self.pos_x = 7;
                        self.pos_y = 7;
                    }

        }
    }
}

impl Ghost {
    fn movement (&mut self, map: &MapGrid){

        let dir = rand::random_range(0..4);

        let mut next_x = self.pos_x;
        let mut next_y = self.pos_y;

        match dir{
            0 => next_y -= 1,
            1 => next_y += 1,
            2 => next_x -= 1,
            3 => next_x += 1,
            _ => println!("input not recognized")
        }

        if map[next_y][next_x] != '#' {
            self.pos_x = next_x;
            self.pos_y = next_y;
        }
    }
}

fn render_map(current_map: &mut MapGrid, player: &mut Player, ghost: &mut Ghost) {
    
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

fn is_win(map: &MapGrid) -> bool {
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