fn main() {
    let pacman_icon: char = 'C';

    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;
    let mut lives: u8 = 3;
    let mut score: i32 = 0;
    let mut is_power_mode: bool = false;

    println!("RustyMan");
    println!("===============================");
    println!("icon : {} score : {} lives : {}", pacman_icon, score, lives);
    println!("position");
    println!("pos_x : {} pos_y : {}",pos_x , pos_y);

    pos_x += 1;
    score += 10;


    println!("===============================");

    
    println!("icon : {} score : {} lives : {}", pacman_icon, score, lives);
    println!("position");
    println!("pos_x : {} pos_y : {}",pos_x , pos_y);
}
