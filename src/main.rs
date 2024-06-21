//! CLI tool to rename files and folders in Windows, Linux and MacOS.
//! author: @Amtr4x
//! license: MIT
fn main() {}

fn _display_banner() {
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
    println!("{padding}{}\n", "#".repeat(tool_name.len() + 8));
}
