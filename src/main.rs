mod arch;
mod config;
mod config_repository;
mod lock_file;
mod version;
mod win_php_client;
mod win_php_domain;
mod zip;

use crate::config::config_menu;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Install a version of PHP
    Install { version: String },
    /// Remove a version of PHP
    Remove { version: String },
    /// Updates all PHP versions
    Update {
        #[arg(long)]
        /// Outputs the operations without performing the actions
        dry_run: bool,
    },
    /// Lists info about all installed PHP versions
    Info,
    /// Configures this tool
    Config,
}

fn main() {
    let args = CliArgs::parse();

    match args.command {
        CliCommands::Install { version } => {
            println!("Install `{}`!", version);
        }
        CliCommands::Remove { version } => {
            println!("Remove `{}`!", version);
        }
        CliCommands::Update { dry_run } => {
            println!("Update! Dry run? {}", dry_run);
        }
        CliCommands::Info => {
            println!("Info!");
        }
        CliCommands::Config => {
            config_menu();
        }
    };
}
