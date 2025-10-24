use reqwest;
use serde_json::Value;
use std::env;
use std::error::Error;

// get token -- export GITHUB_TOKEN="" in terminal
fn get_token() -> String {
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set")
}

// Purpose:
// get the top 10 repos via GitHub API
// input:
// @language : String of what the language of the repos searched should be in
// ouput:
// @items : Array of json containing the details of the first 10 repos
pub async fn fetch_top_repos(language: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    const MAX_REPOS: i8 = 10;

    let client = reqwest::Client::new();
    // build the query parameters
    let query_string = format!("language:{}", language);
    let params = [
        ("q", query_string.as_str()),
        ("sort", "stars"),
        ("order", "desc"),
        ("per_page", &MAX_REPOS.to_string()),
    ];

    let response = client
        .get("https://api.github.com/search/repositories")
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .query(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("API Error ({}): {}", status, error_text);
        return Err(format!("GitHub API returned {}: {}", status, error_text).into());
    }

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
// @repo_name : String of the original repo's name
// created:
// @forks : Array of json containing information of the 20 most recent forks
pub async fn fetch_forks(owner: &str, repo_name: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    const MAX_FORKS: i8 = 20;
    // set url based on the original owner and the repo name
    let url = format!(
        "https://api.github.com/repos/{}/{}/forks?per_page={}&sort=newest",
        owner, repo_name, MAX_FORKS
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("API Error ({}): {}", status, error_text);
        return Err(format!("GitHub API returned {}: {}", status, error_text).into());
    }

    // set forks to the list of json;
    // return error if failed
    let forks: Vec<Value> = response.json().await?;
    Ok(forks)
}

// Purpose:
// get the commits from a given repo
// input:
// @owner : String containing the owner name of the repo
// @repo_name : String containing the repo
// @count : int, the number of commits to look at
// created:
// @commits : Array of json containing information about the commits of the given @repo
pub async fn fetch_commits(
    owner: &str,
    repo_name: &str,
    count: u32,
) -> Result<Vec<Value>, Box<dyn Error>> {
    // set up url
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits?per_page={}",
        owner, repo_name, count
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("API Error ({}): {}", status, error_text);
        return Err(format!("GitHub API returned {}: {}", status, error_text).into());
    }
    // grab the list of json,
    // return error if failed
    let commits: Vec<Value> = response.json().await?;
    Ok(commits)
}

// Purpose:
// get the commit details from a given repo
// input:
// @owner : String containing the owner name
// @repo_name : String contaning the repo name
// @sha : String containing the sha used to get specific commit details
// created:
// @commit_detail : a json containing the commit details of a specific commit with @sha
pub async fn fetch_commit_details(
    owner: &str,
    repo_name: &str,
    sha: &str,
) -> Result<Value, Box<dyn Error>> {
    // set up url
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        owner, repo_name, sha
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("API Error ({}): {}", status, error_text);
        return Err(format!("GitHub API returned {}: {}", status, error_text).into());
    }

    // commit_detail is a json
    let commit_detail: Value = response.json().await?;
    Ok(commit_detail)
}

pub async fn fetch_repo_tree(
    owner: &str,
    repo_name: &str,
    branch: &str,
) -> Result<Vec<Value>, Box<dyn Error>> {
    // set up url
    let url = format!(
        "https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1",
        owner, repo_name, branch
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("API Error ({}): {}", status, error_text);
        return Err(format!("GitHub API returned {}: {}", status, error_text).into());
    }

    let json: Value = response.json().await?;
    let tree = json["tree"].as_array().ok_or("No tree found")?.clone();

    Ok(tree)
}
