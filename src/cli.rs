use clap::Parser;

/// A git repository browsing tool
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// repository to browse
    pub repo: String
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

impl Cli {
}