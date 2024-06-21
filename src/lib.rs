//! RNM tool definitions and test
//! author: @Amtr4x

use std::io::Result;

pub struct Asset<'a> {
    asset_type: &'a str,
    path: String,
}

impl<'a> Asset<'a> {
    // asset_type can be identified by the path structure checking for / at the last char of the path
    pub fn new(path: String) -> Self {
        Self {
            asset_type: "",
            path,
        }
    }

    pub fn rename(&self, new_path: String) -> Result<u8> {
        Ok(0)
    }
}
