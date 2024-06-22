//! RNM tool definitions and test
//! author: @Amtr4x
use std::{
    fs,
    io::{self},
    path::Path,
};

pub struct Asset(String);

impl Asset {
    pub fn new(path: String) -> Self {
        Self(path)
    }

    pub fn rename(&self, new_path: String) {
        let path: &Path = Path::new(&self.0);

        if Path::is_file(path) {
            rename_file(&self.0, new_path).unwrap_or_else(|err| {
                eprintln!("{err}");
            });
        } else {
            rename_dir(&self.0, new_path).unwrap_or_else(|err| {
                eprintln!("{err}");
            });
        }
    }
}

fn rename_file(path: &String, new_path: String) -> io::Result<()> {
    assert!(Path::is_file(Path::new(&path)) && !new_path.ends_with('/'));
    fs::rename(path, new_path)?;
    Ok(())
}

fn rename_dir(path: &String, new_path: String) -> io::Result<()> {
    assert!(Path::is_dir(Path::new(&path)) && new_path.ends_with('/'));
    fs::rename(path, new_path)?;
    Ok(())
}
