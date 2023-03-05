#![warn(clippy::pedantic, clippy::perf)]
#![allow(dead_code)]

use clap::{Args, Parser, Subcommand};
use directories::ProjectDirs;
use git2::Repository;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

// dm

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Read configuration file
    Print,
    Add {
        file: PathBuf,
    },
    Init,
}

#[derive(Debug, Deserialize)]
struct Config {
    comp: Vec<Component>,
}

#[derive(Debug, Deserialize)]
struct Component {
    name: Option<String>,
    dependencies: Option<Vec<String>>,
    config_dir: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Print => {
            let file_contents = fs::read_to_string("config.toml").unwrap();
            let decoded: Config = toml::from_str(&file_contents).unwrap();
            println!("{decoded:#?}");
        }
        Commands::Add { file } => {
            let file_contents = fs::read_to_string(file).unwrap();
            println!("File {} added:", file.display());
            println!("{file_contents}");
        }
        Commands::Init => {
            if let Some(proj_dirs) = ProjectDirs::from("", "", "dm") {
                let data_dir = proj_dirs.data_dir();

                fs::create_dir_all(data_dir)?;

                assert!((Repository::open(data_dir.join("dotfiles")).is_err()), "Repository already created. If you think this is an error, delete {} and try again", data_dir.display());

                println!("Created directory at {}", data_dir.display());

                Repository::init(data_dir.join("dotfiles"))?;
                println!(
                    "Created repository 'dotfiles' at {}",
                    data_dir.join("dotfiles").display()
                );
            }
        }
    }

    Ok(())
}
