use reqwest;
use serde_json::Value;
use std::env;
use std::error::Error;

// get token
fn get_token() -> String {
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set")
}

pub async fn fetch_top_repos(language: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let url: String = format!(
        "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc&per_page=10",
        language
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token())) // ← Use token
        .send()
        .await?;

    let json: Value = response.json().await?;
    let items = json["items"]
        .as_array()
        .ok_or("No items found with github call")?
        .clone();

    Ok(items)
}

pub async fn fetch_forks(owner: &str, repo: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/forks?per_page=20&sort=newest",
        owner, repo
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token())) // ← Use token
        .send()
        .await?;

    let forks: Vec<Value> = response.json().await?;
    Ok(forks)
}

pub async fn fetch_commits(
    owner: &str,
    repo: &str,
    count: u32,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits?per_page={}",
        owner, repo, count
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token())) // ← Use token
        .send()
        .await?;

    let commits: Vec<Value> = response.json().await?;
    Ok(commits)
}

pub async fn fetch_commit_details(
    owner: &str,
    repo: &str,
    sha: &str,
) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        owner, repo, sha
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "160-hw1")
        .header("Authorization", format!("Bearer {}", get_token())) // ← Use token
        .send()
        .await?;

    let commit_detail: Value = response.json().await?;
    Ok(commit_detail)
}
