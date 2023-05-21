use log::{debug, error, trace};
use std::fs;
use std::path::Path;

use crate::arch::Arch;
use crate::config_repository::ConfigRepository;
use crate::lock_file;
use crate::lock_file::LockedVersion;
use crate::version::Version;
use crate::win_php_client::WinPhpClient;
use crate::zip::extract as zip_extract;

pub fn install(version: Version) {
    let mut lock = lock_file::read();
    let config = ConfigRepository::read();
    let config_path = Path::new(config.config.path());
    let ts = config.config.thread_safety();
    let arch = Arch::get();

    debug!(
        "Thread safety {}, Arch {}",
        ts.to_string(),
        arch.to_string()
    );

    if lock.has_version(&version) {
        println!("Version {} already installed", version.to_string());

        return;
    }

    let client = WinPhpClient::new();
    let releases = client
        .get_releases()
        .expect("Could not read windows.php.net");

    let release = releases
        .versions
        .get(version.to_string().as_str())
        .expect("Version not found");

    let build = release
        .builds
        .iter()
        .find(|b| b.0.contains(&ts.to_php_ident()) && b.0.contains(&arch.to_string()));

    if let None = build {
        println!(
            "No matching release for thread safety `{}` and arch `{}`",
            ts.to_string(),
            arch.to_string()
        );

        return;
    }

    let zip_filename = build.unwrap().1.zip.path.as_str();
    let zip_filepath = config_path.join(&zip_filename);

    debug!("{}", zip_filepath.to_str().unwrap());

    if !config_path.exists() {
        trace!("Will create config path \"{}\"", config_path.display());
        fs::create_dir_all(&config_path).expect("Could not create storage directory");
    }

    let mut file = fs::File::create(&zip_filepath).expect("Failed creating local ZIP file");
    client
        .download_zip(zip_filename, &mut file)
        .expect("Failed download ZIP");

    let release_path = config_path.join(version.to_string());

    zip_extract(&zip_filepath, &release_path).expect("Failed unzip");

    if let Err(err) = fs::remove_file(zip_filepath) {
        error!("Could not remove ZIP file: {}", err);
    }

    lock.add_version(LockedVersion {
        version,
        thread_safety: (*ts).clone(),
        arch,
    });

    lock_file::write(&lock);
}

pub fn remove(version: Version) {
    let mut lock = lock_file::read();
    let config = ConfigRepository::read();
    let config_path = Path::new(config.config.path());

    if !lock.has_version(&version) {
        println!("Version {} not installed", version.to_string());

        return;
    }

    let release_path = config_path.join(version.to_string());

    fs::remove_dir_all(release_path).expect("Failed to delete version folder");

    lock.remove_version(version);

    lock_file::write(&lock);
}

pub fn update(version: Option<Version>, dry_run: bool) {}

pub fn info() {
    let mut lock = lock_file::read();

    for version in lock.versions_iter() {
        println!(
            "{} {} {}",
            version.version.to_string(),
            version.arch.to_string(),
            version.thread_safety.to_string()
        );
    }
}
