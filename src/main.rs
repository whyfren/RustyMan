fn main() {
    //variabel gabisa diganti
    let pacman_icon: char = 'C';

    //variabel game
    let mut pos_x: usize = 1;
    let mut pos_y: usize = 1;
    let mut lives: u8 = 3;
    let mut score: i32 = 0;
    let mut is_power_mode: bool = false;

    // input movement
    let mut input_key: char = 'd';

    println!("RustyMan");
    println!("===============================");
    


    for tick in 1..=9 {
        println!("===============================");

        println!("icon : {} score : {} lives : {}", pacman_icon, score, lives);
        println!("position");
        println!("pos_x : {} pos_y : {}",pos_x , pos_y);
        println!("Power Mode : {}", is_power_mode);

        movement(input_key, &mut pos_x, &mut pos_y);
        score += 10;
        check_power_pellet(pos_x, pos_y, &mut score, &mut is_power_mode);
    }
   
    

    
}


fn movement(input_key: char, x: &mut usize, y: &mut usize) {
    match input_key{
        'w' => *y -= 1,
        's' => *y += 1,
        'a' => *x -= 1,
        'd' => *x += 1,
        _ => println!("input not recognized")
    }

    return;
}

fn check_power_pellet(x: usize, y: usize, score: &mut i32, is_power_mode: &mut bool) {
    if y == 1 && x == 2{
            *is_power_mode = true;
            *score += 50;
        }
}
