use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let pattern = &args.pattern;

    // let progress = indicatif::ProgressBar::new(100);
    // for i in 0..100{
    //     // progress.println(format!("[+] finished #{}", i));
    //     progress.inc(1);
    // }    

    genes::find_matches(&content, pattern, &mut std::io::stdout());

    Ok(())
}
