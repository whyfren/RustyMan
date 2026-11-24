// libraries
use std::io::stdout;
use std::time::Duration;
use crossterm::{
    cursor::{Hide, MoveTo, Show}, 
    event::{self, Event, KeyCode, KeyEventKind},
    execute, 
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};

// from other file
mod ghosts;
mod maps;
mod player;
mod ui;
mod renderer;
mod game;

use ghosts::Ghost;
use renderer::{
    terminal_render
};
use maps::{
    MapGrid,
    MAX_LEVEL,
    render_map,
    is_win,
    get_map
};
use player::{
    Player
};
use ui::{
    main_menu,
    winning,
    level_continue
};
use game::{
    Game,
    GameState
};




fn main() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide).unwrap();
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    let mut game = Game::new();
    let mut last_state: GameState = GameState::MainMenu;
    
    loop {
        execute!(stdout(), MoveTo(0, 0), Hide).unwrap();

        if last_state != game.state {
            execute!(stdout(), Clear(ClearType::All)).unwrap();
            last_state = game.state;
        }
        

        terminal_render(&game.render());
        if let Some(c) = read_input() {
            if c == '\n' {
                execute!(stdout(), Clear(ClearType::All)).unwrap();
                game.handle_input("Enter");
            } else if c == 'q' {
                execute!(stdout(), Clear(ClearType::All)).unwrap();
                game.handle_input("q");
            } else {game.handle_input(&c.to_string());}
        }
        game.tick();

        if game.should_quit {break;}
    }

    execute!(stdout(), Clear(ClearType::All)).unwrap();
    execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    disable_raw_mode().unwrap();
}

pub fn read_input() -> Option<char> {
    // its for handling input so fucking long
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => return Some(c),
                        KeyCode::Enter => return Some('\n'),
                        _ => {}
                    }
                }
            }
        }
        None
}