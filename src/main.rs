use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
// reqwest for using the API, aka https
// serde to parse json

#[derive(Debug, Deserialize)]
struct Owner {
    login: String,
    id: u64,
    html_url: String,
    site_admin: bool,
}

#[derive(Debug, Deserialize)]
struct Issue {
    title: String,
    body: String,
    state: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct Repo {
    name: String,
    owner: Owner,
    html_url: String,
    forks_count: u64,
    language: String,
    open_issues_count: u64,
    #[serde(skip)]
    sha: Vec<Commit>,   
}

#[derive(Debug, Deserialize)]
struct Details {
    date: String,
    name: String,
    email: String,
}

// #[derive(Debug, Deserialize)]
// struct AdditionalRepo {
//     forks: Vec<Repo>,        
//     commit: Vec<Commit>, 
//     issues: Vec<Issue>,
//     commit_count: u64,   
// }

#[derive(Debug, serde::Deserialize)]
struct Commit {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    items: Vec<Repo>,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let repo_url = "https://api.github.com/search/repositories?q=language:rust&sort=stars&order=desc&per_page=10";

    // refer to https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#http-method 
    // for GitHub API
    // refer to https://docs.rs/reqwest/latest/reqwest/index.html 
    // for the reqwest documentation
    let mut repo_res = client
        .get(repo_url) // GET the url Client::get()
        .header(USER_AGENT, "rust-client") // [User-Agent] Show valid User-Agent 
        .header(ACCEPT, "application/vnd.github+json") // [Accept] required by the github API
        .header("X-GitHub-Api-Version", "2022-11-28") // [X-GitHub-API-Version] specify a version of the REST API
        .send()? // returns Reponse object 
        .json::<SearchResult>()?; // parse JSON into the struct Repo

    for repo in &mut repo_res.items {
        let commits_url = format!(
            "https://api.github.com/repos/{}/{}/commits?per_page=50",
            repo.owner.login, repo.name
        );
        let commit_res = client
            .get(commits_url)
            .header(USER_AGENT, "rust-client")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()?
            .json::<Vec<Commit>>()?;

        repo.sha = commit_res
    }


     // temporary check
    for repo in repo_res.items { 
        println!("Repo name: {}", repo.name);
        println!("Owner login: {}", repo.owner.login);
        println!("Owner id: {}", repo.owner.id);
        println!("Owner html url: {}", repo.owner.html_url);
        println!("Owner site admin: {}", repo.owner.site_admin);
        println!("Repo html url: {}", repo.html_url);
        println!("Repo fork count: {}", repo.forks_count);
        println!("Repo language: {}", repo.language);
        println!("Repo open issues count: {}", repo.open_issues_count);
        for items in repo.sha {
            println!("sha: {}", items.sha);
        }
        println!();
    }

    Ok(())
}
