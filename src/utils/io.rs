use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::{Error, ErrorKind, Read},
    path::Path,
};

/// Reads all files in a directory that have allowed extensions.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory.
/// * `allow_ext` - An array of allowed file extensions.
///
/// # Returns
///
/// A `Result` containing a vector of strings representing the contents of the files.
/// If no matching files are found, an `Error` with kind `NotFound` is returned.
pub fn read_files_in_dir<P: AsRef<Path>>(
    dir_path: P,
    allow_ext: Vec<&str>,
) -> std::io::Result<Vec<String>> {
    let mut contents = Vec::new();

    for entry in read_dir(dir_path)? {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            if allow_ext.iter().any(|&ext| OsStr::new(ext) == extension) {
                let mut file = File::open(entry.path())?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                contents.push(buffer);
            }
        }
    }

    if contents.is_empty() {
        Err(Error::new(ErrorKind::NotFound, "No matching files found"))
    } else {
        Ok(contents)
    }
}

/// Reads the contents of a single file.
///
/// # Arguments
///
/// * `file_path` - The path to the file.
///
/// # Returns
///
/// A `Result` containing a string representing the contents of the file.
pub fn read_single_file(file_path: &Path) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
