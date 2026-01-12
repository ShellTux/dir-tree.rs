use clap::Parser;
use dir_tree::dir_tree::DirTree;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(required = true)]
    paths: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    dbg!(&cli);

    for dir in cli.paths {
        let dir_tree = DirTree::from(dir)?;
        println!("{dir_tree:#?}");

        println!("json: {}", serde_json::to_string(&dir_tree)?);

        for file in &dir_tree {
            println!("{}", file.display());
        }

        println!(
            "File count `{}`: {}",
            dir_tree.dir_name().display(),
            dir_tree.file_count()
        );
    }

    Ok(())
}
