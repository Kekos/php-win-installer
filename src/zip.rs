use crate::zip::ZipError::{IoError, OutDirNotDirectory, ZipLibError};
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::{fs, io};

#[derive(Debug)]
pub enum ZipError {
    IoError(io::Error),
    ZipLibError(zip::result::ZipError),
    OutDirNotDirectory(String),
}

impl<'e> Display for ZipError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IoError(_) => "IO error",
                ZipLibError(_) => "Zip lib error",
                OutDirNotDirectory(_) => "Out path was not a directory",
            }
        )
    }
}

pub fn extract<P: AsRef<Path>>(zip_path: P, out_path: P) -> Result<(), ZipError> {
    let file = fs::File::open(zip_path).map_err(|err| IoError(err))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|err| ZipLibError(err))?;

    let out_path_ref = out_path.as_ref();

    if !out_path_ref.exists() {
        fs::create_dir_all(out_path_ref).map_err(|err| IoError(err))?;
    }

    if !out_path_ref.is_dir() {
        return Err(OutDirNotDirectory(
            out_path_ref.as_os_str().to_string_lossy().into_owned(),
        ));
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_out_path = match file.enclosed_name() {
            Some(path) => out_path_ref.join(path.to_owned()),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&file_out_path).unwrap();
        } else {
            if let Some(p) = file_out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&file_out_path).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(())
}
