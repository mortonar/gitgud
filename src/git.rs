use crate::cli::Cli;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use crossterm::event::{Event, KeyCode};
use crossterm::style::Stylize;
use crossterm::{terminal, ExecutableCommand};
use git2::{Commit, Error, Repository};
use std::io::stdout;

/// Abstraction wrapper over git2::Repository
pub struct Repo {
    repository: Repository,
}

pub fn open_repo(cli: Cli) -> Result<Repo, Error> {
    let repo = Repository::open(&cli.repo)?;
    Ok(Repo { repository: repo })
}

pub fn get_head_commit(repo: &Repository) -> Result<Commit, Error> {
    repo.head()?.peel_to_commit()
}

pub struct CommitList {
    selected: usize,
    commits: Vec<GGCommit>,
}

impl CommitList {
    pub fn display(&self) {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        for (i, c) in self.commits.iter().enumerate() {
            let line = format!(
                "[{}] {} | {}\r",
                &c.time.clone().green(),
                &c.author.clone().yellow(),
                &c.message.clone().white()
            );
            if i == self.selected {
                println!("{}", &line.on_blue());
            } else {
                println!("{}", &line);
            }
        }
    }

    pub fn event(&mut self, event: Event) {
        if event == Event::Key(KeyCode::Char('j').into()) {
            self.selected = (self.selected + 1) % self.commits.len();
        } else if event == Event::Key(KeyCode::Char('k').into()) {
            self.selected = self.selected.saturating_sub(1);
        }
    }
}

struct GGCommit {
    author: String,
    message: String,
    time: String,
}

pub fn get_commit_list(repo: &Repo) -> Result<CommitList, Error> {
    let head = get_head_commit(&repo.repository)?;
    let mut stack = Vec::new();
    let mut commit_list = Vec::new();
    stack.push(head);
    for _i in 0..=10 {
        if let Some(c) = stack.pop() {
            commit_list.push(GGCommit {
                author: format!("{}", c.author()),
                message: format!("{}", c.summary().expect("no commit!")),
                time: time_to_string(c.time().seconds()),
            });
            stack.push(c.parent(0)?);
        }
    }
    Ok(CommitList {
        selected: 0,
        commits: commit_list,
    })
}

pub fn time_to_string(secs: i64) -> String {
    let time = DateTime::<Local>::from(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(secs, 0),
        Utc,
    ));
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}
