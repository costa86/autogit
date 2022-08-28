use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use open::that;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Display;
use std::io::Error;
use std::thread;
use std::time;
use std::{collections::HashMap, process::Command};

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    name: String,
    html_url: String,
}

const BASE_ROUTE: &str = "https://api.github.com";
const FIRST_COMMIT_MESSAGE: &str = "first commit";
const BRANCH: &str = "main";
const AGENT: &str = "foo";
pub const CHOICES: [&str; 3] = [
    "Display repositories",
    "Create a repository based on current directory",
    "Exit",
];
pub const REPO_ACTIONS: [&str; 3] = ["Visit", "Delete", "Cancel"];

pub fn display_repositories() {
    let repositories = get_repositories();
    let repo_names = repositories
        .iter()
        .map(|x| format!("{}", &x.name))
        .collect::<Vec<String>>();

    let (selected_repo_name, _) = get_user_selection(
        &repo_names,
        format!("Repository (quantity: {})", repositories.len()).as_str(),
    );
    let (_, index) = get_user_selection(
        &REPO_ACTIONS.to_vec(),
        format!("Action on {}", selected_repo_name).as_str(),
    );

    let selected_repo = repositories
        .iter()
        .filter(|x| x.name == selected_repo_name)
        .next()
        .unwrap();

    match index {
        0 => that(&selected_repo.html_url).unwrap(),
        1 => delete_repository(&selected_repo.name),
        _ => return,
    };
}

///Delete a GitHub repository
fn delete_repository(repo_name: &str) {
    let (user, token) = get_github_credentials();
    let client = reqwest::blocking::Client::new();

    if get_user_confirmation(format!("Are you sure you wish to delete {}", &repo_name).as_str()) {
        let url = format!("{BASE_ROUTE}/repos/{user}/{repo_name}");

        let res = client
            .delete(url)
            .header(USER_AGENT, AGENT)
            .bearer_auth(&token)
            .send();

        match res {
            Ok(_) => {
                display_message("ok", format!("{repo_name} was deleted").as_str(), "green");
            }
            Err(_) => {
                display_message(
                    "error",
                    format!("{repo_name} could not be deleted").as_str(),
                    "red",
                );
            }
        };
    } else {
        display_message(
            "ok",
            format!("{repo_name} was NOT deleted").as_str(),
            "green",
        );
    }
}

///Get user and token
fn get_github_credentials() -> (String, String) {
    let github_user = env::var("GITHUB_USER").unwrap();
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    (github_user, github_token)
}

///Create a new GitHub repository based on current directory
pub fn create_repository() {
    let (user, token) = get_github_credentials();
    let name = get_user_input("Repo name:", &user);
    let client = reqwest::blocking::Client::new();
    let url = format!("{BASE_ROUTE}/user/repos");

    let mut map = HashMap::new();
    map.insert("name", &name);

    let res = client
        .post(url)
        .header(USER_AGENT, AGENT)
        .json(&map)
        .bearer_auth(&token)
        .send();

    match res {
        Ok(_) => {
            display_message(
                "ok",
                format!("Creating repository {}...", &name).as_str(),
                "green",
            );
            run_git_commands(&name)
        }
        Err(_) => {
            display_message(
                "error",
                format!("Could not create repository {}...", &name).as_str(),
                "red",
            );
        }
    }
}

fn get_repositories() -> Vec<Repo> {
    let url = format!("{BASE_ROUTE}/user/repos?per_page=100");
    let token = get_github_credentials().1;
    let client = reqwest::blocking::Client::new();

    let res: Vec<Repo> = client
        .get(url)
        .bearer_auth(&token)
        .header(USER_AGENT, AGENT)
        .send()
        .unwrap()
        .json()
        .unwrap();
    res
}

///Run a git command
fn run_cmd(commands: &[&str]) -> Result<(), Error> {
    Command::new("git").args(commands).spawn()?;
    thread::sleep(time::Duration::from_secs(3));
    Ok(())
}

///Run git-related commands
fn run_git_commands(repo_name: &str) {
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

fn display_message(message_type: &str, message: &str, color: &str) {
    let msg = format!("[{}] {}", message_type.to_uppercase(), message).color(color);
    println!("{msg}");
}

///Get Y/N response
fn get_user_confirmation(question: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .default(true)
        .interact()
        .unwrap()
}

///Get text response
fn get_user_input(text: &str, default_text: &str) -> String {
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
