use gitgud::cli;
use gitgud::git;

fn main() {
    let cli = cli::Cli::default();
    git::open_repo(cli).unwrap();
}
