

use crate::ghosts::Ghost;
use crate::renderer::{
    Frame,
    Color
};
use crate::player::Player; 

pub fn main_menu() -> Frame {
    let mut frame = Frame::new();

    frame.pushln("===============================", Color::Default, true);
    frame.pushln("           RUSTYMAN            ", Color::Orange, true);
    frame.pushln("===============================", Color::Default, true);
        
    frame.push("Press Enter To ", Color::Default, true);
    frame.pushln("Contnue", Color::Green, true);

    frame.push("Press Q To ", Color::Default, true);
    frame.pushln("Exit", Color::Red, true);

    frame.pushln("===============================", Color::Default, true);

    frame
}


pub fn winning(win: bool, player: &Player, total_score: i32) -> Frame {
    let mut frame = Frame::new();
        frame.pushln("===============================", Color::Default, true);
        let win_massage: &str;

        if win {
            win_massage = "YOU WIN";
            frame.push(win_massage, Color::Yellow, true);
            
        } else {
            win_massage = "YOU lOSE";
            frame.push(win_massage, Color::Red, true);
        }
        frame.new_line();


        frame.push("YOUR SCORE: ", Color::Default, true);
        frame.pushln(&player.score.to_string(), Color::Default, true);
        
        frame.push("TOTAL SCORE: ", Color::Default, true);
        frame.pushln(&total_score.to_string(), Color::Default, true);

        frame.push("Press Q To ", Color::Default, true);
        frame.pushln("Exit", Color::Red, true);

        if win {
            frame.push("Press Enter To ", Color::Default, true);
            frame.pushln("Contnue", Color::Green, true);
        }

        frame.pushln("===============================", Color::Default, true);
    

    frame
}

pub fn level_continue(current_level: & u8) -> Frame {
    let mut frame = Frame::new();
    
        frame.pushln("===============================", Color::Default, true);
        frame.pushformat(format!("         Level {} Cleared \n", current_level), Color::Default, true);
        frame.push("Press Enter To ", Color::Default, true);
        frame.pushln("Contnue", Color::Green, true);
        frame.push("Press Q To ", Color::Default, true);
        frame.pushln("Exit", Color::Red, true);
        frame.pushln("===============================", Color::Default, true);
        
    frame
}

pub fn end(player: &Player, total_score: i32) -> Frame {
    let mut frame = Frame::new();
        frame.pushln("===============================", Color::Default, true);
    
        frame.pushln("The End", Color::Green, true);

        frame.push("YOUR SCORE: ", Color::Default, true);
        frame.pushln(&player.score.to_string(), Color::Default, true);
        
        frame.push("TOTAL SCORE: ", Color::Default, true);
        frame.pushln(&total_score.to_string(), Color::Default, true);

        frame.push("Press Q To ", Color::Default, true);
        frame.pushln("Exit", Color::Red, true);
        frame.pushln("===============================", Color::Default, true);
    frame
}

pub fn header(current_level: & u8) -> Frame {
    let mut frame = Frame::new();
    
        frame.pushln("===============================", Color::Default, true);
        frame.pushformat(format!("          Level : {}  \n", current_level), Color::Default, true);
        frame.pushln("===============================", Color::Default, true);
        
    frame
}

pub fn stats(player: &Player) -> Frame {
    let mut frame = Frame::new();
        frame.pushln("===============================", Color::Default, true);

        frame.push("SCORE: ", Color::Default, true);
        frame.push(&player.score.to_string(), Color::Default, true);

        frame.push("    LIVES: ", Color::Default, true);
        frame.pushln(&player.lives.to_string(), Color::Green, true);
        
        frame.push("Power : ", Color::Default, true);
        frame.pushln(&player.power_mode_duration.to_string(), Color::Yellow, true);
        frame.pushln("===============================", Color::Default, true);
    frame
}

pub fn ghost_detector(player: &Player, ghost: &Ghost) -> Frame {
    let mut frame = Frame::new();
    let dist: usize = ghost.pos_x.abs_diff(player.pos_x) + ghost.pos_y.abs_diff(player.pos_y);
        frame.pushln("Ghost Detector:     ", Color::Default, true);
        if dist <= 4 && ghost.live && !player.is_power_mode {
            frame.pushln("WARNING GHOST CLOSE DETECTED", Color::Red, true);
        } else if dist <= 6 && ghost.live && !player.is_power_mode{
            frame.pushln("WARNING GHOST GETTING CLOSE    ", Color::Yellow, true);
        } else {
            frame.pushln("OK                             ", Color::Green, true);
        }
        frame.pushln("===============================", Color::Default, true);
    frame
}