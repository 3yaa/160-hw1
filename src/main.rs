mod api;
mod data_manager;
use crate::{data_manager::{get_commits_info, get_forks_info}, inspect::looks_like_source};
mod inspect;
mod models;
mod redis;
use std::error::Error;
//
use models::repo;

// main function: oscastrates through each part of the hw
#[tokio::main] //sets up the async runtime  
async fn main() -> Result<(), Box<dyn Error>> {
    // Add C later?
    let languages = vec!["Rust", "C++", "Java"];

    // loop thru each lang
    for lang in languages {
        // fetch top 10 repo
        let repos_json = api::fetch_top_repos(lang).await?;
        let mut repos = repo::parse_repos(repos_json);
        // process each repo
        // for each repo, run heuristic to determine if it looks like source
        // store <String, int> into hashmap
        // github_url, number of stars
        let mut repo_source: HashMap<String, u64> = HashMap::new();
        for repo in &mut repos {
            // fetch forks for repo
            get_forks_info(repo).await;
            // fetch commits for repo
            get_commits_info(repo).await;
            // TODO: make repo.files or something to check the files keywords
            let source =  true; // looks_like_source(repo.files);
            if (source) {
                repo_source.insert(repo.html_url, repo.stars);
            }
            // get the repo with the most stars:
            if let Some((url, stars)) = repo_stars.iter().max_by_key(|entry| entry.1) {
                // Clone the Repo
                // get details
                // call Redis and store it
            }
        }
        // showcase stat found in api calls
        display_stat(repos, lang);
    }

    Ok(())
}

fn display_stat(repos: Vec<repo::Repo>, lang: &str) {
    // ADD UP FOR TOP 10
    let total_stars: u64 = repos.iter().map(|r| r.stars).sum();
    let total_forks: u64 = repos.iter().map(|r| r.forks_count).sum();
    let total_fork_commits: u64 = repos.iter().map(|r| r.fork_commit_count).sum();
    let total_open_issues: u64 = repos.iter().map(|r| r.open_issues_count).sum();

    // DISPLAY TOP 10
    println!("Language: {}", lang);
    println!("Total stars: {}", total_stars);
    println!("Total forks: {}", total_forks);
    println!("Top-3 Most modified file per repo");
    for repo in repos {
        // keep top 3 of the files
        let top_three_files: Vec<(String, u32)> =
            repo.top_modified_files.iter().take(3).cloned().collect();
        for (index, (filename, count)) in top_three_files.iter().enumerate() {
            println!("Repo name: {}", repo.name);
            println!(
                "File name {}: {}, Modifications: {}",
                index + 1,
                filename,
                count
            );
        }
    }
    println!("New commits in forked repos: {}", total_fork_commits);
    println!("Open issues in top-10 repos: {}", total_open_issues);
    println!("--------------------------------------------");
}
