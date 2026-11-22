#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

mod ghosts;
mod maps;
mod player;

use ghosts::Ghost;
use maps::{get_map, is_win, MapGrid, MAP_HEIGHT, MAP_WIDTH, MAX_LEVEL};
use player::Player;

#[derive(PartialEq)] enum GameState {
    MainMenu,
    Playing,
    LevelWin,
    GameOver,
    TheEnd,
}

#[wasm_bindgen]
pub struct WebGame {
    pacman: Player,
    ghost: Ghost,
    current_map: MapGrid,
    current_level: u8,
    total_score: i32,
    last_move: char,
    state: GameState,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)] pub fn new() -> WebGame {
        let mut pacman = Player{
        icon: 'C',
        pos_x: 7,
        pos_y: 7,
        lives: 3,
        score: 0,
        is_power_mode: false,
        power_mode_duration: 0,
        last_move: ' '
    };

    // ghost declaration
    let mut ghost: Ghost = Ghost{
        icon: 'G',
        pos_x: 7,
        pos_y: 5,
        live: true
    };
    let mut total_score: i32 = 0;

    let mut current_level: u8 = 1;
    let mut current_map: MapGrid = get_map(current_level);

    let mut win: bool = false;
    let mut game_stop: bool = false;

    let mut last_move: char= ' ';

    let mut warning_massage: &str;

    return WebGame {
        pacman,
        ghost,
        current_map,
        current_level: 1,
        total_score: 0,
        last_move: ' ',
        state: GameState::MainMenu,
    }
    }



    pub fn handle_input(&mut self, key: &str) {
        match self.state {
            GameState::MainMenu => {
                if key == "Enter"{
                    self.state = GameState::Playing;
                } else {

                }
            }
            GameState::Playing => {
                match key {
                    "w" => {self.last_move = 'w'}
                    "a" => {self.last_move = 'a'}
                    "s" => {self.last_move = 's'}
                    "d" => {self.last_move = 'd'}
                    "q" => {self.state = GameState::MainMenu}
                    _ => {}
                }
            }
            GameState::LevelWin => {
                match key {
                    "Enter" => {
                        self.state = GameState::Playing;
                        self.current_level += 1;
                        self.current_map = get_map(self.current_level);
                        self.last_move = ' ';
                        self.pacman.pos_x = 7;
                        self.pacman.pos_y = 7;
                        self.pacman.last_move = ' ';
                        self.pacman.lives = 3;
                        self.pacman.power_mode_duration = 0;
                        self.pacman.is_power_mode = false;
                        self.ghost.pos_x = 7;
                        self.ghost.pos_y = 5;
                        self.ghost.live = true;
                        self.pacman.score = 0;
                    }
                    "q" => {self.state = GameState::MainMenu}
                    _ => {}
                }
            }
            GameState::GameOver => {
                match key {
                    "Enter" => {
                        *self = WebGame::new();
                        self.state = GameState::Playing;
                    }
                    "q" => {self.state = GameState::MainMenu}
                    _ => {}
                }
            }
            GameState::TheEnd => {
                match key {
                    "Enter" => {
                        *self = WebGame::new();
                        self.state = GameState::Playing;
                    }
                    "q" => {self.state = GameState::MainMenu}
                    _ => {}
                }
            }
        }
    }



    pub fn tick(&mut self) {
        if self.state != GameState::Playing{return;}
        // player movement
        if self.last_move != ' ' {
            self.pacman.movement(self.last_move, &self.current_map);
        }
        // player mobement/

        // check pwer pallete
        self.pacman.check_power_pellet(&self.current_map);

        // eating the dot
        self.current_map[self.pacman.pos_y][self.pacman.pos_x] = ' ';

        // checking ghost collision
        self.pacman.check_ghost_collision(&mut self.ghost);
        // ghost movement 
        if self.ghost.live{self.ghost.movement(&self.current_map);} 
        // checking ghost collision (again)
        self.pacman.check_ghost_collision(&mut self.ghost);

        // win condition handling
        // if is_win(&self.current_map) {
        //     self.win = true;
        //     self.game_stop = true;
        // }
        // if self.pacman.lives == 0 {
        //     self.win = false;
        //     self.game_stop = true;
        // }
        // // win handling/


        if is_win(&self.current_map) {
            self.total_score += self.pacman.score;
            if self.current_level == MAX_LEVEL {
                self.state = GameState::TheEnd;
            } else {
                self.state = GameState::LevelWin;
            }
            return;
        }

        if self.pacman.lives == 0 {
            self.total_score += self.pacman.score;
            self.state = GameState::GameOver;
        }


    }





    pub fn render_html(&self) -> String {
        let mut out = String::new();
        
        match self.state {
            GameState::MainMenu => {}
            GameState::Playing => {
                out.push_str("===============================\n");
                out.push_str(&format!("Level : {}\n", self.current_level));
                out.push_str("===============================\n");
                for y in 0..MAP_HEIGHT {
                    for x in 0..MAP_WIDTH {
                        let dx = x.abs_diff(self.pacman.pos_x);
                        let dy = y.abs_diff(self.pacman.pos_y);

                        let in_light = dx * dx + dy * dy <= 10 || self.pacman.is_power_mode;



                        if self.pacman.pos_x == x && self.pacman.pos_y == y {
                            out.push_str(&format!("<span class=\"pacman\">{} </span>", self.pacman.icon));
                        



                        } else if self.ghost.pos_x == x && self.ghost.pos_y == y && self.ghost.live && in_light{
                        
                            if self.pacman.is_power_mode {
                                out.push_str(&format!("<span class=\"scared\">{} </span>", self.ghost.icon));
                            } else {
                                out.push_str(&format!("<span class=\"ghost\">{} </span>", self.ghost.icon));
                            }

                        } else {
                            if in_light{
                                match self.current_map[y][x]{
                                    '#' => out.push_str(&format!("<span class=\"wall\"># </span>",  self.current_map[y][x])),
                                    '.' => out.push_str(&format!("<span class=\"dot\">. </span>",  self.current_map[y][x])),
                                    'o' => out.push_str(&format!("<span class=\"pellet\">o </span>",  self.current_map[y][x])),
                                    _ => out.push_str("  ")
                                }
                            } else {
                                out.push_str("  ");
                            }

                        }


                    }
                    out.push('\n');
                }

                


            }
            GameState::GameOver => {}
            GameState::LevelWin => {}
            GameState::TheEnd => {}
        }


        
    }



}