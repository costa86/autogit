use autogit::*;
use colored::Colorize;
use std::io::stdin;

fn main() {
    let mut input = String::new();

    let title = format!(
        "\n{} - {} \nAuthors: {}\nVersion: {}\nLicense: {}\nCrafted with ❤️ using Rust language\n",
        env!("CARGO_PKG_NAME").to_uppercase(),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_LICENSE")
    )
    .color("yellow");
    println!("{title}");

    let options_menu = "Available options (case insensitive):
    LR: List repositories
    CR: Create a repository based on current directory
    DR: Delete a repository
    H:  Help
    Q:  Quit";

    loop {
        println!("{options_menu}");
        stdin().read_line(&mut input).unwrap();

        match input.to_uppercase().as_str().trim() {
            "LR" => list_repos(),
            "CR" => create_repo(&get_user_input("Repo name:")),
            "DR" => delete_repo(&get_user_input("Repo name do DELETE:")),
            "H" => display_message(
                "help",
                "Make sure you have 'GITHUB_USER' and 'GITHUB_TOKEN' set as environment vars",
                "blue",
            ),
            "Q" => break,
            _ => {
                display_message("error", "Invalid option", "red");
            }
        }
        input.clear();
    }
}
