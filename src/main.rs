mod api;
mod data_manager;
use crate::{data_manager::{get_commits_info, get_forks_info, display_stat}, inspect::looks_like_source};
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


