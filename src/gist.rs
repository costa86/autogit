use crate::*;
use open::that;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Gist {
    pub description: String,
    pub html_url: String,
    pub id: String,
}

fn get_gists() -> Vec<Gist> {
    let url = format!("{BASE_ROUTE}/gists?per_page=100");
    let token = get_github_credentials().1;
    let client = reqwest::blocking::Client::new();

    let res: Vec<Gist> = client
        .get(url)
        .bearer_auth(&token)
        .header(USER_AGENT, AGENT)
        .send()
        .unwrap()
        .json()
        .unwrap();
    res
}

fn delete_gist(gist: &gist::Gist) {
    let (_, token) = get_github_credentials();
    let client = reqwest::blocking::Client::new();

    if get_user_confirmation(
        format!("Are you sure you wish to delete {}", &gist.description).as_str(),
    ) {
        let url = format!("{BASE_ROUTE}/gists/{}", gist.id);

        let res = client
            .delete(url)
            .header(USER_AGENT, AGENT)
            .bearer_auth(&token)
            .send();

        match res {
            Ok(_) => {
                display_message(
                    "ok",
                    format!("{} was deleted", &gist.description).as_str(),
                    "green",
                );
            }
            Err(_) => {
                display_message(
                    "error",
                    format!("{} could not be deleted", &gist.description).as_str(),
                    "red",
                );
            }
        };
    } else {
        display_message(
            "ok",
            format!("{} was NOT deleted", &gist.description).as_str(),
            "green",
        );
    }
}

pub fn display_gists() {
    let gists = get_gists();
    let gist_names: Vec<String> = gists.iter().map(|x| x.description.to_string()).collect();
    let (selected_gist_description, _) = get_user_selection(
        &gist_names,
        format!("Gists: (Quantity: {})", gists.len()).as_str(),
    );

    let (_, index) = get_user_selection(
        &REPO_ACTIONS.to_vec(),
        format!("Action on {}", selected_gist_description).as_str(),
    );
    let selected_gist = gists
        .iter()
        .filter(|x| x.description == selected_gist_description)
        .next()
        .unwrap();

    match index {
        0 => that(&selected_gist.html_url).unwrap(),
        1 => delete_gist(&selected_gist),
        _ => return,
    };
}
