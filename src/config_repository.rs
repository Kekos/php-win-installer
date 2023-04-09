use crate::config::Config;
use home::home_dir;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub struct ConfigRepository {
    pub config: Config,
}

impl ConfigRepository {
    pub fn read() -> ConfigRepository {
        let config_data = match fs::read_to_string(get_config_path()) {
            Ok(data) => data,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => String::from(""),
                error => panic!("Could not open config file: {}", error),
            },
        };

        let config: Config =
            toml::from_str(config_data.as_str()).expect("Could not parse config file");

        ConfigRepository { config }
    }

    pub fn write(config_repo: &ConfigRepository) {
        let config_data =
            toml::to_string(&config_repo.config).expect("Could not convert config to TOML");

        if let Err(error) = fs::write(get_config_path(), config_data) {
            panic!("Could not write config file: {}", error);
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut path = home_dir().expect("Could not detect your home directory");

    path.push(".pwin.toml");

    path
}
