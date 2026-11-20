fn main() {

    //player variable declaration
    let mut pacman = Player{
        icon: 'C',
        pos_x: 1,
        pos_y: 1,
        lives: 3,
        score: 0,
        is_power_mode: false
    };

    let map: [[char; 5]; 5] = [
        ['#','#','#','#','#'],
        ['#','.','.','.','#'],
        ['#','.','#','.','#'],
        ['#','o','.','.','#'],
        ['#','#','#','#','#']
    ];

    // input movement
    let mut input_key: char = 'd';

    

    println!("RustyMan");
    println!("===============================");
    


    for tick in 1..=3 {
        println!("===============================");

        println!("icon : {} score : {} lives : {}", pacman.icon, pacman.score, pacman.lives);
        println!("position");
        println!("pos_x : {} pos_y : {}", pacman.pos_x , pacman.pos_y);
        println!("Power Mode : {}", pacman.is_power_mode);

        render_map(&map, &mut pacman);

        
        pacman.movement(input_key);
        pacman.check_power_pellet();
    }
}



struct Player{
    icon: char,
    pos_x: usize,
    pos_y: usize,
    lives: u8,
    score: i32,
    is_power_mode: bool
}

impl Player {

    fn movement(&mut self, input_key: char) {
    match input_key{
        'w' => self.pos_y -= 1,
        's' => self.pos_y += 1,
        'a' => self.pos_x -= 1,
        'd' => self.pos_x += 1,
        _ => println!("input not recognized")
        }
    }

    fn check_power_pellet(&mut self) {
    if self.pos_y == 1 && self.pos_x == 2{
            self.is_power_mode = true;
            self.score += 50;
        }
    }
}

fn render_map(map: &[[char; 5]; 5], player: &mut Player) {
    for y in 0..5 {

        for x in 0..5{ 
    
            if player.pos_x == x && player.pos_y == y {
                print!("{}", player.icon);
                if map[x][y] == 'o'{
                    player.is_power_mode = true;
                    player.score += 50;
                } if map[x][y] == '.'{
                    player.score += 10;
                }
            } else {
                print!("{}", map[x][y]);
            }

        }
    println!("");
    }
}
