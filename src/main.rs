mod install;
mod toml;
mod system;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "rice8y",
    version = "0.1.0",
    about = "CLI tool to install Typst packages from GitHub or local path",
    long_about = "This tool allows you to install, list, and remove Typst packages locally. It supports fetching packages from GitHub repositories or local directories."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        #[arg(help = "Specify the GitHub URL or local directory path of the Typst package")]
        source: String,
        #[arg(short, long, help = "Show detailed output during installation")]
        verbose: bool,
    },
    Clean {
        #[arg(help = "Name of the installed Typst package")]
        name: String,
        #[arg(help = "Version of the package to remove")]
        version: String,
    },
    List {
        #[arg(help = "Show the list of all installed Typst packages")]
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { source, verbose } => install::install(&source, verbose)?,
        Commands::Clean { name, version } => install::clean(&name, &version)?,
        Commands::List { verbose } => {
            if verbose {
                println!("Listing installed packages in verbose mode...");
            }
            install::list()?
        }
    }

    Ok(())
}
