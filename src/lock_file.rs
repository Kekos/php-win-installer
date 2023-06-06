use crate::arch::Arch;
use crate::config::ThreadSafety;
use crate::version::Version;
use home::home_dir;
use log::{debug, trace};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct LockFile {
    versions: Vec<LockedVersion>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct LockedVersion {
    pub version: Version,
    pub thread_safety: ThreadSafety,
    pub arch: Arch,
}

impl LockFile {
    pub fn has_version(&self, version: &Version) -> bool {
        self.versions
            .iter()
            .find(|v| v.version.match_major_minor(version))
            .is_some()
    }

    pub fn add_version(&mut self, version: LockedVersion) {
        self.versions.push(version);
    }

    pub fn remove_version(&mut self, version: Version) {
        self.versions
            .retain(|v| v.version.match_major_minor(&version))
    }

    pub fn versions_iter(&self) -> std::slice::Iter<'_, LockedVersion> {
        self.versions.iter()
    }
}

pub trait LockFileRepository {
    fn new() -> Self;
    fn read(&self) -> LockFile;
    fn write(&self, lock: &LockFile);
}

pub struct FileLockFileRepository {}

impl LockFileRepository for FileLockFileRepository {
    fn new() -> Self {
        Self {}
    }

    fn read(&self) -> LockFile {
        let config_data = match fs::read_to_string(get_lock_path()) {
            Ok(data) => data,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => String::from("versions = []"),
                error => panic!("Could not open lock file: {}", error),
            },
        };

        let lock: LockFile =
            toml::from_str(config_data.as_str()).expect("Could not parse lock file");

        lock
    }

    fn write(&self, lock: &LockFile) {
        trace!("Begin writing lock file");

        let lock_data = toml::to_string(&lock).expect("Could not convert lock to TOML");

        trace!("End writing lock file");

        if let Err(error) = fs::write(get_lock_path(), lock_data) {
            panic!("Could not write lock file: {}", error);
        }
    }
}

fn get_lock_path() -> PathBuf {
    let mut path = home_dir().expect("Could not detect your home directory");

    path.push(".pwin.lock");

    debug!("Looking for lock file at path \"{}\"", path.display());

    path
}

#[cfg(test)]
pub struct MemoryLockFileRepository {
    lock_file: String,
}

impl LockFileRepository for MemoryLockFileRepository {
    fn new() -> Self {
        Self {
            lock_file: String::from("versions = []"),
        }
    }

    fn read(&self) -> LockFile {
        let lock: LockFile =
            toml::from_str(self.lock_file.as_str()).expect("Could not parse lock file");

        lock
    }

    fn write(&self, _lock: &LockFile) {}
}
