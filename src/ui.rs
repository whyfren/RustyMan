

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
        frame.pushformat(format!("           Level {} Cleared \n", current_level), Color::Default, true);
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