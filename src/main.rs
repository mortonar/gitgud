use crossterm::event::{read, Event, KeyCode};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{terminal, ExecutableCommand};
use gitgud::cli;
use gitgud::git;
use io::Result;
use std::io;
use std::io::stdout;

struct Line {
    text: &'static str,
}

fn main() -> Result<()> {
    // let cli = cli::Cli::default();
    // git::open_repo(cli).unwrap();

    let mut lines = Vec::new();
    lines.push(Line {
        text: "This is the first line",
    });
    lines.push(Line {
        text: "This is the second line",
    });
    let mut selected = 0;
    enable_raw_mode()?;
    loop {
        draw_lines(&lines, selected);
        // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
        let event = read()?;
        if event == Event::Key(KeyCode::Char('j').into()) {
            selected = (selected + 1) % lines.len();
        } else if event == Event::Key(KeyCode::Char('k').into()) {
            selected = selected.saturating_sub(1);
        } else if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }
    disable_raw_mode()?;
    Ok(())
}

fn draw_lines(lines: &Vec<Line>, selected: usize) {
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    print!("selected: {}", &selected);
    for (i, l) in lines.iter().enumerate() {
        if i == selected {
            println!("{}\r", &l.text.on_blue().on_blue());
        } else {
            println!("{}\r", &l.text);
        }
    }
}
