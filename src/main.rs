mod arch;
mod config;
mod config_repository;
mod lock_file;
mod version;
mod version_manager;
mod win_php_client;
mod win_php_domain;
mod zip;

use crate::config::config_menu;
use crate::version::Version;
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
        /// Version to update, default is all versions
        version: Option<String>,
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
            if let Ok(version) = version.parse::<Version>() {
                version_manager::install(version);
            } else {
                println!("`version` argument could not be parsed");
            }
        }
        CliCommands::Remove { version } => {
            if let Ok(version) = version.parse::<Version>() {
                version_manager::remove(version);
            } else {
                println!("`version` argument could not be parsed");
            }
        }
        CliCommands::Update { dry_run, version } => {
            if let Some(version) = version {
                if let Ok(version) = version.parse::<Version>() {
                    version_manager::update(Some(version), dry_run);
                } else {
                    println!("`version` argument could not be parsed");
                }
            } else {
                version_manager::update(None, dry_run);
            }
        }
        CliCommands::Info => {
            version_manager::info();
        }
        CliCommands::Config => {
            config_menu();
        }
    };
}
