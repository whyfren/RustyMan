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
        ['#','.','.','o','#'],
        ['#','.','#','.','#'],
        ['#','.','.','.','#'],
        ['#','#','#','#','#']
    ];
    let mut current_map: [[char; 5]; 5];


    // input movement
    let mut input_key: char = 'd';

    //this is creating map that dynamic, ex the pellet is eaten and gone
    current_map = map;

    println!("RustyMan");
    
    


    for tick in 1..=5 {
        println!(" ");
        println!("===============================");

        render_map(&mut current_map, &mut pacman);
        pacman.movement(input_key, &current_map);

        println!("===============================");

        println!("icon : {} score : {} lives : {}", pacman.icon, pacman.score, pacman.lives);
        println!("position");
        println!("pos_x : {} pos_y : {}", pacman.pos_x , pacman.pos_y);
        println!("Power Mode : {}", pacman.is_power_mode);

        
        // pacman.check_power_pellet();
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

    fn movement(&mut self, input_key: char, map: &[[char; 5]; 5]) {
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

    fn check_power_pellet(&mut self, map: &[[char; 5]; 5], x: &usize, y: &usize) {
        if map[*y][*x] == 'o'{
            self.is_power_mode = true;
            self.score += 50;
        } if map[*y][*x] == '.'{
            self.score += 10;
        } 
    }
}

fn render_map(current_map: &mut [[char; 5]; 5], player: &mut Player) {
    
    for y in 0..5 {

        for x in 0..5{ 
    
            if player.pos_x == x && player.pos_y == y {
                print!("{}", player.icon);
                player.check_power_pellet(current_map, &x, &y);
                current_map[y][x] = ' ';
            } else {
                print!("{}", current_map[y][x]);
            }

        }
    println!("");
    }
}
