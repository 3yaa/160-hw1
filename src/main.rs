mod api;
mod data_manager;
mod inspect_repo;
mod models;
mod redis;
use std::error::Error;
//
use models::repo;

// main function: oscastrates through each part of the hw
#[tokio::main] //sets up the async runtime  
async fn main() -> Result<(), Box<dyn Error>> {
    let languages = vec!["C++", "C", "Rust", "Java"];

    // loop thru each lang
    for lang in languages {
        // fetch top 10 repo
        let repos_json = api::fetch_top_repos(lang).await?;
        let mut repos = repo::parse_repos(repos_json);
        // part A: process each repo
        for repo in &mut repos {
            // fetch forks for repo
            data_manager::get_forks_info(repo).await;
            // fetch commits for repo
            data_manager::get_commits_info(repo).await;
        }
        // part B: showcase stat found in api calls
        data_manager::display_stat(&repos, lang);
        // part C: download the top real repo
        inspect_repo::clone_top_repo(&repos).await;
    }

    // save redis to disk
    redis::save_redis_to_disk().await;

    Ok(())
}
