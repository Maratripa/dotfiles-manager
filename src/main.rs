#![warn(clippy::pedantic, clippy::perf)]
#![allow(dead_code)]

use clap::{Parser, Subcommand};
use directories::{BaseDirs, ProjectDirs};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    options: Options,
    comp: Option<Vec<Component>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Options {
    base_dir: PathBuf,
}

impl Options {
    fn default() -> Self {
        Self {
            // TODO: work arround unwrap
            base_dir: std::env::current_dir().unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Component {
    name: Option<String>,
    dependencies: Option<Vec<String>>,
    config_dir: Option<String>,
}

impl Config {
    fn default() -> Self {
        Self {
            options: Options::default(),
            comp: None,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // track base repo dir with config file or /home/user by default
    // current dir: std::env::current_dir() -> Result<PathBuf>

    let data_dir = match ProjectDirs::from("", "", "dm") {
        Some(dir) => dir.data_dir().to_path_buf(),
        None => {
            if let Some(base_dir) = BaseDirs::new() {
                base_dir.data_dir().join("dm")
            } else {
                panic!("Could not get base directory")
            }
        }
    };

    match &cli.command {
        Commands::Print => {
            let file_contents = fs::read_to_string("config.toml").unwrap();
            let decoded: Config = toml::from_str(&file_contents).unwrap();
            println!("{decoded:#?}");
        }
        Commands::Add { file } => {
            let config_file = fs::read_to_string(data_dir.join("config.toml"))?;
            let config: Config = toml::from_str(&config_file)?;

            println!("Base directory is: {:#?}", config.options.base_dir);

            let file_contents = fs::read_to_string(file).unwrap();
            println!("File {} added:", file.display());
            println!("{file_contents}");
        }
        Commands::Init => {
            fs::create_dir_all(&data_dir)?;

            assert!((Repository::open(data_dir.join("dotfiles")).is_err()), "Repository already created. If you think this is an error, delete {} and try again", data_dir.display());
            println!("Created directory at {}", data_dir.display());

            let config = Config::default();

            let toml_config = toml::to_string(&config)?;
            let mut config_file = fs::File::create(data_dir.join("config.toml"))?;
            config_file.write_all(toml_config.as_bytes())?;
            println!("Created file 'config.toml' at {}", data_dir.display());

            Repository::init(data_dir.join("dotfiles"))?;
            println!(
                "Created repository 'dotfiles' at {}",
                data_dir.join("dotfiles").display()
            );
        }
    }

    Ok(())
}
