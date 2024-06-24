//! Parse the request of the user and perform actions based on the provided request. This module is the front end of the app.
//! author: @Amtr4x
use std::env;

use rnm::Asset;

fn display_banner() {
    let tool_name: &str = "RENAME TOOL";
    let padding: String = " ".repeat(4);

    println!("{padding}{}", "#".repeat(tool_name.len() + 8));
    println!(
        "{padding}{}{}{}",
        "#".repeat(2),
        " ".repeat(tool_name.len() + 4),
        "#".repeat(2)
    );
    println!("{padding}{}  {tool_name}  {}", "#".repeat(2), "#".repeat(2));
    println!(
        "{padding}{}{}{}",
        "#".repeat(2),
        " ".repeat(tool_name.len() + 4),
        "#".repeat(2)
    );
    println!("{padding}{}", "#".repeat(tool_name.len() + 8));
}

fn display_menu() {
    let menu = "
    Description:
    Rename a file or folder in the specified path.
    
    Syntax:    
    rnm -flag current_archive renamed_archive
    rnm -flag # to query extra info

    Flags:
    -p, --path Specify the path to be renamed (Mandatory).

    -h, --help  Show this help menu and exit.

    -i, --info  Show tool info, repo link, author...
    
    Examples:
    # renaming a directory
    rnm -p my_folder/ my_renamed_folder/

    # renaming an archive using verbose arguments
    rnm --path my_archive.ext my_renamed_archived.ext
    ";
    display_banner();
    println!("{menu}");
}

fn display_app_info() {
    // TODO update donation info and version
    let info = "author: Leandrys Osorio (Amtr4x)\nLicense: MIT\nrelease-version: 1.0b\n\n for donations read the README.md info at: https://github.com/Amtr4x/rnm?tab=readme-ov-file#can-i-make-a-thank-you-donation";
    println!("{info}");
}

fn parse_request<'a>(args: Vec<String>) -> (Option<String>, Option<String>, Option<String>) {
    if args.len() == 4 {
        (
            Some(args.get(1).unwrap().to_string()),
            Some(args.get(2).unwrap().to_string()),
            Some(args.get(3).unwrap().to_string()),
        )
    } else {
        (Some(args.get(1).unwrap().to_string()), None, None)
    }
}

pub fn setup() {
    let args = env::args().collect();
    let (flag, current_path, new_path) = parse_request(args);
    match flag.unwrap().to_lowercase().as_str() {
        "-h" | "--help" => display_menu(),
        "-i" | "--info" => display_app_info(),
        "-p" | "--path" => {
            let asset = Asset::new(current_path.unwrap());
            asset.rename(new_path.unwrap());
        }
        _ => {
            eprintln!("Error: Unknown flag.");
            display_menu();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    #[test]
    fn rename_a_file() {
        let new_file = "hello.txt";
        let renamed_file = "bye.txt";
        let asset = Asset::new(new_file.to_string());

        // Create a testing directory with a file to rename
        fs::File::create(&new_file).unwrap();

        asset.rename(renamed_file.to_string());
        assert!(Path::new(renamed_file).exists());

        //clean the directory and the renamed file.
        fs::remove_file(renamed_file).unwrap();
    }
}
