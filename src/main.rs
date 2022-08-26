use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use gitgud::cli;
use gitgud::git;
use gitgud::git::get_commit_list;
use io::Result;
use std::io;

fn main() -> Result<()> {
    let cli = cli::Cli::default();
    let repo = git::open_repo(cli).unwrap();

    enable_raw_mode()?;
    let mut commit_list = get_commit_list(&repo).unwrap();
    loop {
        commit_list.display();
        let event = read()?;
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        } else {
            commit_list.event(event);
        }
    }
    disable_raw_mode()?;
    Ok(())
}
