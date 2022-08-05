use crate::cli::Cli;
use git2::{Commit, Error, Repository};

pub fn open_repo(cli: Cli) -> Result<(), Error> {
    let repo = Repository::open(&cli.repo)?;

    let state = repo.state();
    println!("{}({:?}):", &cli.repo, &state);
    println!();

    let head = get_head_commit(&repo)?;
    let mut stack = Vec::new();
    stack.push(head);
    for _i in 0..=10 {
        if let Some(c) = stack.pop() {
            display_commit_info(&c);
            stack.push(c.parent(0)?);
        }
    }
    // let mut revwalk = match repo.revwalk() {
    //     Ok(revwalk) => revwalk,
    //     Err(e) => panic!("failed to get revwalk: {}", e)
    // };
    // revwalk.set_sorting(Sort::NONE).expect("set revwalk sorting failed");
    // revwalk.push_head().expect("revwalk push HEAD failed");
    // if let Some(Ok(oid)) = revwalk.next() {
    //     let commit = match repo.find_commit(oid) {
    //         Ok(commit) => commit,
    //         Err(e) => panic!("couldn't find commit: {}", e)
    //     };
    //     display_commit_info(&commit);
    // }
    Ok(())
}

pub fn get_head_commit(repo: &Repository) -> Result<Commit, Error> {
    repo.head()?.peel_to_commit()
}

pub fn display_commit_info(commit: &Commit) {
    println!("-----------------------------------------------------------------------------------");
    println!("Author: {}", commit.author());
    println!("{}", commit.message().expect("no commit!"));

    // if let Ok(tree) = commit.tree() {
    //     for tree_entry in tree.iter() {
    //         if let Some(ot) = tree_entry.kind() {
    //             if ot == ObjectType::Blob {
    //                 println!("{}:", tree_entry.name().unwrap());
    //                 let object = tree_entry.borrow().to_object(&repo).unwrap();
    //                 let content = String::from_utf8_lossy(object.as_blob().unwrap().content());
    //                 println!("blob: {}", &content);
    //             }
    //         }
    //     }
    // }
    println!("-----------------------------------------------------------------------------------");
}
