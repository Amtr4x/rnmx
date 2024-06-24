//! RNM tool definitions and test
//! author: @Amtr4x
use std::{
    fs,
    io::{self},
};

pub struct Asset(String);

impl Asset {
    pub fn new(path: String) -> Self {
        assert!(!path.is_empty());
        Self(path)
    }

    /// Rename the provided asset. First checks for a file and
    /// if not a file found checks for a directory.
    ///
    /// ## Arguments
    /// * `&self` - self asset object reference.
    /// * `new_path` - the new named asset, should not be empty.
    ///
    /// ## Panics
    /// If an empty string name is provided function will panic.
    /// If you try to rename a directory with file path or viceversa.
    ///
    /// ## Example
    /// ```rust
    /// use rnm::Asset;
    ///
    /// // Simulate creating a new asset instance for a file
    /// let file_path = Asset::new(String::from("hello.txt"));
    ///
    /// file_path.rename(String::from("bye.txt"));
    ///
    ///```
    pub fn rename(&self, new_path: String) {
        if !self.0.ends_with('/') && !new_path.ends_with('/') {
            rename_file(&self.0, &new_path).unwrap_or_else(|err| {
                eprintln!("{err}");
            });
        } else {
            rename_dir(&self.0, &new_path).unwrap_or_else(|err| {
                eprintln!("{err}");
            });
        }
    }
}

/// Rename a file, ensuring that the current name and the new name
/// are not typed as a directory path.
///
/// ## Arguments
/// * `path: &String` - current path of the file.
/// * `new_path: &String` - new name for the file.
///
/// ## Returns
/// * `io:Result<()>` - an `Ok` after the file renaming and a console message indicating the success,
/// if there is an error it will return it.
fn rename_file(path: &String, new_path: &String) -> io::Result<()> {
    if !path.ends_with('/') && !new_path.ends_with('/') {
        fs::rename(path, new_path)?;
        println!("File renamed successfuly");
    }
    Ok(())
}

/// Rename a folder, ensuring that the current name and the new name
/// are written as a directory path.
///
/// ## Arguments
/// * `path: &String` - current path of the directory.
/// * `new_path: &String` - new name for the directory.
///
/// ## Returns
/// * `io::Result<()>` - an `Ok` after the folder renaming and a console message indicating the success,
/// if there is an error it will return it.
fn rename_dir(path: &String, new_path: &String) -> io::Result<()> {
    if path.ends_with('/') && new_path.ends_with('/') {
        fs::rename(path, new_path)?;
        println!("Directory renamed successfuly");
    }
    Ok(())
}
