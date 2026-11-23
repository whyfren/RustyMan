use std::io::stdout;
use crossterm::{
    cursor::{Hide, MoveTo}, 
    style::Stylize,
    execute, 
};

use crate::player::Player;
use crate::read_input;

pub fn main_menu() -> bool {
    let s: &str = "Start"; 
    let q: &str = "Exit";
    loop{
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();
        println!("===============================");
        println!("           RUSTYMAN            ");
        println!("===============================");
        println!("Press Enter To {}", s.green().bold());
        println!("Press Q To {}", q.red().bold());
        println!("===============================");

        // read user input
        if let Some(c) = read_input() {
            if c == 'q' {return true;}
            if c == '\n' {return false;}
        }
        // read user input/
        
    }
}


pub fn winning(win: bool, player: &Player, total_score: i32) -> bool {
    loop {
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();
        println!("===============================");

        let win_massage: &str;

        if win {
            win_massage = "YOU WIN";
            println!("{}", win_massage.yellow());
            
        } else {
            win_massage = "YOU lOSE";
            println!("{}", win_massage.red());
        }

        println!("YOUR SCORE: {}", player.score);
        println!("TOTAL SCORE: {}", total_score);
        

        println!(" ");
        println!("PRESS q TO EXIT");

        if win {
            println!("PRESS ENTER TO CONTINUE");
        }

        if let Some(c) = read_input() {
            if c == 'q' {return false;}
            if c == '\n' && win {
                    return true;
            }
        }
        println!("===============================");
    }

    
}

pub fn level_continue(current_level: &mut u8) -> bool {
    let s: &str = "Continue"; 
    let q: &str = "Exit";
    loop{
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();
        
        println!("===============================");
        println!("Press Enter To {}", s.green().bold());
        println!("Press Q To {}", q.red().bold());
        println!("===============================");

        // read user input
        if let Some(c) = read_input() {
            if c == 'q' {
                return false;
            }
            if c == '\n' {
                *current_level += 1;
                return true;
            }
        }
        // read user input/
        
    }
}

pub fn end(player: &Player, total_score: i32) -> bool {
    loop {
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();
        println!("===============================");

        let win_massage: &str;

        
        win_massage = "THE END";
        println!("{}", win_massage.yellow());
        println!("YOUR SCORE: {}", player.score);
        println!("TOTAL SCORE: {}", total_score);
        println!(" ");
        println!("PRESS q TO EXIT");

        if let Some(c) = read_input() {
            if c == 'q' {return false;}
        }
        println!("===============================");
    }
}