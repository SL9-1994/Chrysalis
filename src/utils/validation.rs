use std::path::PathBuf;

/// Enum representing the type of a path.
#[derive(Debug)]
pub enum PathType {
    File,
    Directory,
    NotFound,
}

/// Determines whether a given path is a directory or a file.
///
/// # Arguments
///
/// * `path` - A reference to a `PathBuf` representing the path to be checked.
///
/// # Returns
///
/// * `PathType` - The type of the path: `File`, `Directory`, or `NotFound`.
pub fn is_dir_or_file(path: &PathBuf) -> PathType {
    match path {
        path if path.is_dir() => PathType::Directory,
        path if path.is_file() => PathType::File,
        _ => PathType::NotFound,
    }
}
