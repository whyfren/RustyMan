use std::io::stdout;
use crossterm::{
    cursor::{Hide, MoveTo, Show}, 
    event::{self, Event, KeyCode, KeyEventKind}, 
    style::Stylize,
    execute, 
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};

use crate::read_input;

pub fn main_menu() -> bool {
    let mut s: &str = "Start"; 
    let mut q: &str = "Exit";
    loop{
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();
        
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