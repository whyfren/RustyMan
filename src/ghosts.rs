use rand;

pub struct Ghost{
    pub icon: char,
    pub pos_x: usize,
    pub pos_y: usize,
    pub live: bool
}
impl Ghost {
    pub fn movement (&mut self, map: &MapGrid){

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