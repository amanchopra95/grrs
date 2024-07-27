use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<()> {

    let args = Cli::parse();
    // let result = std::fs::read_to_string("test.txt");
    // let content = match result {
    //     Ok(content) => {content},
    //     Err(error) => {return Err(error.into());}
    // };

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
