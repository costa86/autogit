use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::env;
use std::fmt::Display;
use std::io::Error;
use std::process::Command;
use std::thread;
use std::time;
pub mod gist;
pub mod repository;

pub const BASE_ROUTE: &str = "https://api.github.com";
const FIRST_COMMIT_MESSAGE: &str = "first commit";
const BRANCH: &str = "main";
pub const AGENT: &str = "foo";
pub const REPO_ACTIONS: [&str; 3] = ["Visit", "Delete", "Cancel"];
pub const CHOICES: [&str; 4] = [
    "Display repositories",
    "Create a repository based on current directory",
    "Display gists",
    "Exit",
];

///Get user and token
pub fn get_github_credentials() -> (String, String) {
    let github_user = env::var("GITHUB_USER").unwrap();
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    (github_user, github_token)
}

///Run a git command
fn run_cmd(commands: &[&str]) -> Result<(), Error> {
    Command::new("git").args(commands).spawn()?;
    thread::sleep(time::Duration::from_secs(3));
    Ok(())
}

///Run git-related commands
pub fn run_git_commands(repo_name: &str) {
    let user = get_github_credentials().0;

    run_cmd(&["init"]).unwrap();
    run_cmd(&["add", "-A"]).unwrap();
    run_cmd(&["commit", "-m", &FIRST_COMMIT_MESSAGE]).unwrap();
    run_cmd(&["branch", "-M", &BRANCH]).unwrap();
    run_cmd(&[
        "remote",
        "add",
        "origin",
        format!("git@github.com:{user}/{repo_name}.git").as_str(),
    ])
    .unwrap();

    match run_cmd(&["push", "-u", "origin", &BRANCH]) {
        Ok(_) => {
            let msg = format!("Done. Go to https://github.com/{user}/{repo_name} to see.");
            display_message("ok", &msg, "green");
        }
        Err(_) => {
            let msg = format!("Could not create {repo_name}...");
            display_message("error", &msg, "red");
        }
    };
}

pub fn display_message(message_type: &str, message: &str, color: &str) {
    let msg = format!("[{}] {}", message_type.to_uppercase(), message).color(color);
    println!("{msg}");
}

///Get Y/N response
pub fn get_user_confirmation(question: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .default(true)
        .interact()
        .unwrap()
}

///Get text response
pub fn get_user_input(text: &str, default_text: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(text)
        .default(default_text.into())
        .interact_text()
        .unwrap()
}

///Get singe response from choices
pub fn get_user_selection<T>(items: &Vec<T>, title: &str) -> (String, usize)
where
    T: Display,
{
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .with_prompt(title)
        .default(0)
        .interact()
        .unwrap();

    (items.get(selection).unwrap().to_string(), selection)
}

pub fn display_app_intro(color: &str) {
    let title = format!(
        "\n{} - {} \nAuthors: {}\nVersion: {}\nLicense: {}\nCrafted with ❤️ using Rust language\n",
        env!("CARGO_PKG_NAME").to_uppercase(),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_LICENSE")
    )
    .color(color);
    println!("{title}");
}
