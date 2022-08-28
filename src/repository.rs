use crate::*;
use open::that;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
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

pub fn get_repositories() -> Vec<repository::Repo> {
    let url = format!("{BASE_ROUTE}/user/repos?per_page=100");
    let token = get_github_credentials().1;
    let client = reqwest::blocking::Client::new();

    let res: Vec<repository::Repo> = client
        .get(url)
        .bearer_auth(&token)
        .header(USER_AGENT, AGENT)
        .send()
        .unwrap()
        .json()
        .unwrap();
    res
}

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
