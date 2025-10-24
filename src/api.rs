use reqwest;
use serde_json::Value;
use std::env;
use std::error::Error;

// get token
fn get_token() -> String {
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set")
}

// Purpose:
// get the top repos via GitHub API
// input:
// @language : String of what the language of the repos searched should be in
// created:
// @items : Array of json containing the details of the first 10 repos
pub async fn fetch_top_repos(language: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    const MAX_REPOS: i8 = 10;
    // set up url depending on language
    let url: String = format!(
        "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc&per_page={}",
        language, MAX_REPOS
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1") // User Agent name, 160-hw1
        .header("Authorization", format!("Bearer {}", get_token())) // insert key (key not directly within the files)
        .send() // send request to GitHub API
        .await?; // await, return error if fail

    // get the json from response
    let json: Value = response.json().await?;
    // get the "items" field in the json and return it as an array
    // or return an error if it does not exist
    let items = json["items"]
        .as_array()
        .ok_or("No items found with github call")?
        .clone();

    Ok(items)
}

// Purpose:
// fetch the forks using the GitHub API
// inputs:
// @owner : String of the original repo's owner name
// @repo : String of the original repo's name
// created:
// @forks : Array of json containing information of the 20 most recent forks
pub async fn fetch_forks(owner: &str, repo: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    const MAX_FORKS: i8 = 20;
    // set url based on the original owner and the repo name
    let url = format!(
        "https://api.github.com/repos/{}/{}/forks?per_page={}&sort=newest",
        owner, repo, MAX_FORKS
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;

    // set forks to the list of json;
    // return error if failed
    let forks: Vec<Value> = response.json().await?;
    Ok(forks)
}

// Purpose:
// get the commits from a given repo
// input:
// @owner : String containing the owner name of the repo
// @repo : String containing the repo
// @count : int, the number of commits to look at
// created:
// @commits : Array of json containing information about the commits of the given @repo
pub async fn fetch_commits(
    owner: &str,
    repo: &str,
    count: u32,
) -> Result<Vec<Value>, Box<dyn Error>> {
    // set up url
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits?per_page={}",
        owner, repo, count
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;

    // grab the list of json,
    // return error if failed
    let commits: Vec<Value> = response.json().await?;
    Ok(commits)
}

// Purpose:
// get the commit details from a given repo
// input:
// @owner : String containing the owner name
// @repo : String contaning the repo name
// @sha : String containing the sha used to get specific commit details
// created:
// @commit_detail : a json containing the commit details of a specific commit with @sha
pub async fn fetch_commit_details(
    owner: &str,
    repo: &str,
    sha: &str,
) -> Result<Value, Box<dyn Error>> {
    // set up url
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        owner, repo, sha
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;

    // commit_detail is a json
    let commit_detail: Value = response.json().await?;
    Ok(commit_detail)
}
