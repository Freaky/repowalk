use std::env;
use std::io::Write;

use git2::{Oid, Repository};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args_os().nth(1).expect("Repository as first argument");
    let persist = env::args_os()
        .nth(2)
        .expect("Last seen file as second argument");

    let repo = Repository::open(path)?;

    let mut walker = repo.revwalk()?;
    let mut tips = vec![];

    // Record the current tips and push to the walker
    for tip in repo.references_glob("refs/heads/*")? {
        let tip = tip?.target().expect("ref should have a target");
        tips.push(tip.to_string());
        walker.push(tip)?;
    }

    // Load previously seen tips, if any
    if let Ok(prev) = std::fs::read_to_string(&persist) {
        for line in prev.lines() {
            walker.hide(Oid::from_str(&line)?)?;
        }
    }

    // Walk
    for oid in walker.into_iter() {
        println!("{}", oid?);
    }

    // Save the current tips for next time
    let mut out = std::fs::File::create(&persist)?;
    for tip in tips {
        writeln!(&mut out, "{}", tip)?;
    }
    out.sync_all()?;

    Ok(())
}
