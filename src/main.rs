use colored::*;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{stdin, Error};
use std::thread;
use std::time;
use std::{collections::HashMap, process::Command};
use tabled::{Style, Table, Tabled};

const BASE_ROUTE: &str = "https://api.github.com";
const AGENT: &str = "foo";


///Delete a GitHub repository
fn delete_repo(repo_name: &str) {
    let user = get_github_credentials().0;
    let token = get_github_credentials().1;

    let url = format!("{BASE_ROUTE}/repos/{user}/{repo_name}");
    
    let client = reqwest::blocking::Client::new();
    
    let res = client
    .delete(url)
    .header(USER_AGENT, AGENT)
    .bearer_auth(&token)
        .send();
        
        match res {
        Ok(_) => {
            let msg = format!("Deleted {repo_name}");
            display_message("ok", &msg, "green");
        }
        Err(_) => {
            let msg = format!("Could not delete {repo_name}");
            display_message("error", &msg, "red");
        }
    };
}

///Get user and token
fn get_github_credentials() -> (String, String) {
    let github_user = env::var("GITHUB_USER").unwrap();
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    (github_user, github_token)
}

///Create a new GitHub repository based on current directory
fn create_repo(name: &str) {
    let token = get_github_credentials().1;
    let client = reqwest::blocking::Client::new();
    let url = format!("{BASE_ROUTE}/user/repos");
    
    let mut map = HashMap::new();
    map.insert("name", name);
    
    let res = client
        .post(url)
        .header(USER_AGENT, AGENT)
        .json(&map)
        .bearer_auth(&token)
        .send();
        
        match res {
            Ok(_) => {
                let msg = format!("Creating {name}...");
                display_message("ok", &msg, "green");
                run_git_commands(&name)
        }
        Err(_) => {
            let msg = format!("Could not create repo for {name}...");
            display_message("error", &msg, "red");
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct Repo {
    name: String,
    html_url: String,
    private: bool,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct RepoRow {
    num: u8,
    name: String,
    url: String,
    created_at: String,
}

///List all GitHub repositories
fn list_repos() {
    let url = format!("{BASE_ROUTE}/user/repos?per_page=100");
    let token = get_github_credentials().1;
    let client = reqwest::blocking::Client::new();
    let mut repo_list: Vec<RepoRow> = Vec::new();
    let mut counter = 1;

    let res: Vec<Repo> = client
        .get(url)
        .bearer_auth(&token)
        .header(USER_AGENT, AGENT)
        .send()
        .unwrap()
        .json()
        .unwrap();
        
        for i in res {
        let repo_row = RepoRow {
            name: i.name,
            url: i.html_url,
            num: counter,
            created_at: i.created_at,
        };
        repo_list.push(repo_row);
        counter += 1;
    }
    
    let table = Table::new(&repo_list).with(Style::modern());
    println!("{table}");
}

///Run a git command
fn run_cmd(commands: &[&str]) -> Result<(), Error> {
    Command::new("git").args(commands).spawn()?;
    thread::sleep(time::Duration::from_secs(3));
    Ok(())
}

///Get user's input
pub fn get_user_input(text: &str) -> String {
    let mut input = String::new();
    println!("{}", text);
    stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

///Run git-related commands
fn run_git_commands(repo_name: &str) {
    let user = get_github_credentials().0;

    run_cmd(&["init"]).unwrap();
    run_cmd(&["add", "-A"]).unwrap();
    run_cmd(&["commit", "-m", "yes"]).unwrap();
    run_cmd(&["branch", "-M", "main"]).unwrap();
    run_cmd(&[
        "remote",
        "add",
        "origin",
        format!("git@github.com:{user}/{repo_name}.git").as_str(),
        ])
    .unwrap();
    
    match run_cmd(&["push", "-u", "origin", "main"]) {
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

///Display message to the user
pub fn display_message(message_type: &str, message: &str, color: &str) {
    let msg = format!("[{}] {}", message_type.to_uppercase(), message).color(color);
    println!("{msg}");
}


fn main() {
    let mut input = String::new();
    
    let title = "AUTOGIT - Iterate with GitHub API \nAuthor: Lorenzo Costa <costa86@zoho.com>\n";
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
