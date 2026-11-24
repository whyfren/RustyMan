#![cfg(target_arch = "wasm32")]

use crate::game::Game;
use crate::renderer::web_render;

mod game;
mod ui;
mod player;
mod ghosts;
mod maps;
mod renderer;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct WebGame {
    game: Game,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGame {
        WebGame { game: Game::new() }
    }
    pub fn handle_input(&mut self, key: &str){
        self.game.handle_input(key);
    }
    pub fn tick(&mut self) {
        self.game.tick();
    }
    pub fn render_html(&self) -> String{
        web_render(&self.game.render())
    }
}