use std::{io::{stdout, stdin}, panic::Location};
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    event::{EnableMouseCapture, DisableMouseCapture}, execute, cursor
};
use ratatui::{prelude::CrosstermBackend, Terminal};

mod utils;
mod uri;
mod anilist;
mod pages;
mod nyaa;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        exit();
        let location = info.location().unwrap_or(Location::caller());
        eprintln!("Error: {}, at '{}:{}'",
            info.to_string(),
            location.file(),
            location.line()
        );
        stdin().read_line(&mut String::new()).unwrap();
    }));
    init();
    uri::register();
    if let Some(auth_code) = std::env::args().nth(1) {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        let user = anilist::User::login(auth_code.split("?code=").nth(1).unwrap());
        pages::menu::show(&user, &mut terminal);
    } else {
        open::that("https://anilist.co/api/v2/oauth/authorize?client_id=14244&redirect_uri=ndnd://&response_type=code").unwrap()
    }
    exit()
}

fn init() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture, cursor::Hide).unwrap()
}
fn exit() {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, cursor::Show).unwrap()
}