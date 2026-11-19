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

        match input_key{
        'w' => pos_y -= 1,
        's' => pos_y += 1,
        'a' => pos_x -= 1, 
        'd' => pos_x += 1,
        _ => println!("input not recognized")
    }

    score += 10;
    if pos_y == 1 && pos_x == 2{
        is_power_mode = true;
        score += 50;
    }
    

    
    }
    // movement(input_key, pos_x, pos_y);
    

    
}


// fn movement(input_key: char, mut x: usize, mut y: usize) {
//     match input_key{
//         'w' => y -= 1,
//         's' => y += 1,
//         'a' => x -= 1,
//         'd' => x += 1,
//         _ => println!("input not recognized")
//     }

//     return;
// }
