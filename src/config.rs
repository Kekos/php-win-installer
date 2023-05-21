use crate::config_repository::ConfigRepository;
use dialoguer::console::Term;
use dialoguer::Select;
use log::error;
use serde_derive::{Deserialize, Serialize};
use std::io::stdin;
use std::string::String;

#[derive(Serialize, Deserialize)]
pub struct Config {
    path: Option<String>,
    thread_safety: Option<ThreadSafety>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum ThreadSafety {
    Safe,
    NonSafe,
}

impl ThreadSafety {
    pub fn to_php_ident(&self) -> String {
        String::from(match self {
            Self::Safe => "ts",
            Self::NonSafe => "nts",
        })
    }
}

impl ToString for ThreadSafety {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Safe => "TS",
            Self::NonSafe => "NTS",
        })
    }
}

impl Config {
    pub fn path(&self) -> &str {
        if let Some(str) = &self.path {
            return str;
        }

        "C:\\"
    }

    pub fn thread_safety(&self) -> &ThreadSafety {
        if let Some(val) = &self.thread_safety {
            return val;
        }

        &ThreadSafety::NonSafe
    }
}

pub fn config_menu() {
    let term = Term::stdout();

    let menu = vec![
        "View configuration",
        "Set install path",
        "Select thread safety mode",
        "Quit",
    ];

    while let Some(chosen) = Select::new().items(&menu).interact_opt().unwrap() {
        term.clear_screen().unwrap();

        println!("Configuration");
        println!("Select one by using arrow keys and then ENTER");

        match menu[chosen] {
            "View configuration" => view(&term),
            "Set install path" => set_install_path(&term),
            "Select thread safety mode" => select_thread_safety(&term),
            _ => break,
        }
    }
}

fn view(term: &Term) {
    term.clear_screen().unwrap();

    let config_repo = ConfigRepository::read();

    println!("Install path: {}", config_repo.config.path());
    println!(
        "Thread safety mode: {}",
        match config_repo.config.thread_safety() {
            ThreadSafety::Safe => "Safe (TS)",
            ThreadSafety::NonSafe => "None (NTS)",
        }
    );
}

fn set_install_path(term: &Term) {
    term.clear_screen().unwrap();

    println!("Enter your wanted installation base path for all your PHP installations:");
    let path = try_read_line();

    let mut config_repo = ConfigRepository::read();

    if path == "" {
        config_repo.config.path = None;
    } else {
        config_repo.config.path = Some(path);
    }

    ConfigRepository::write(&config_repo);

    println!("Configuration saved!");
}

fn select_thread_safety(term: &Term) {
    let mut config_repo = ConfigRepository::read();
    let mut keep_selecting = true;

    while keep_selecting {
        term.clear_screen().unwrap();

        println!("Select thread safety mode by using arrow keys and then ENTER:");
        let modes = vec![
            "Thread Safe (used with Apache Webserver)",
            "Non Thread Safe (used with IIS or CLI)",
        ];

        let selected_mode = Select::new().items(&modes).interact();

        match selected_mode {
            Ok(0) => {
                config_repo.config.thread_safety = Some(ThreadSafety::Safe);
                keep_selecting = false;
            }
            Ok(1) => {
                config_repo.config.thread_safety = Some(ThreadSafety::NonSafe);
                keep_selecting = false;
            }
            _ => {
                error!("Not a valid choice")
            }
        }
    }

    ConfigRepository::write(&config_repo);

    println!("Configuration saved!");
}

fn try_read_line() -> String {
    let mut input_string = String::new();

    stdin()
        .read_line(&mut input_string)
        .expect("Could not read line");

    input_string.trim().to_string()
}
