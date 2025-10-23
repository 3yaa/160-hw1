mod api;
mod models;
mod storage;

use models::fork;
use models::repo;
use std::error::Error;

async fn get_forks_info(repo: &mut repo::Repo) {
    match api::fetch_forks(&repo.owner_login, &repo.name).await {
        Ok(forks_json) => {
            let forks = fork::parse_forks(forks_json);
            let mut fork_commit_count: u64 = 0;
            // fetch recent commits
            for fork in &forks {
                match api::fetch_commits(&fork.owner_login, &fork.name, 10).await {
                    Ok(commits) => {
                        fork_commit_count += commits.len() as u64;
                    }
                    Err(e) => {
                        println!("---->error fetching commits: {}", e);
                    }
                }
            }
            repo.fork_commit_count = fork_commit_count;
        }
        Err(e) => {
            println!("---->error fetching forks: {}", e);
        }
    }
}

async fn get_commits_info(repo: &mut repo::Repo) {
    match api::fetch_commits(&repo.owner_login, &repo.name, 50).await {
        Ok(commits) => {
            use std::collections::HashMap;
            let mut file_counts: HashMap<String, u32> = HashMap::new();

            for commit in &commits {
                if let Some(sha) = commit["sha"].as_str() {
                    match api::fetch_commit_details(&repo.owner_login, &repo.name, sha).await {
                        Ok(detail) => {
                            if let Some(files) = detail["files"].as_array() {
                                for file in files {
                                    let filename =
                                        file["filename"].as_str().unwrap_or("unknown").to_string();
                                    *file_counts.entry(filename).or_insert(0) += 1;
                                }
                            }
                        }
                        Err(e) => println!("----->error fetching commit details: {}", e),
                    }
                }
            }

            // convert hash into vec
            repo.top_modified_files = file_counts.into_iter().collect();

            // sort
            repo.top_modified_files.sort_by(|a, b| b.1.cmp(&a.1));

            // keep top 3
            repo.top_modified_files.truncate(3);
        }
        Err(e) => {
            println!("   Error fetching commits: {}", e);
        }
    }
}

#[tokio::main] //sets up the async runtime  
async fn main() -> Result<(), Box<dyn Error>> {
    let languages = vec!["Rust", "C++", "Java"];

    // loop thru each lang
    for lang in languages {
        // fetch top 10 repo
        let repos_json = api::fetch_top_repos(lang).await?;
        let mut repos = repo::parse_repos(repos_json);
        // process each repo
        for repo in &mut repos {
            // fetch forks for repo
            get_forks_info(repo).await;
            // fetch commits for repo
            get_commits_info(repo).await;
        }
        // ADD UP FOR TOP 10
        let total_stars: u64 = repos.iter().map(|r| r.stars).sum();
        let total_forks: u64 = repos.iter().map(|r| r.forks_count).sum();
        let total_fork_commits: u64 = repos.iter().map(|r| r.fork_commit_count).sum();
        let total_open_issues: u64 = repos.iter().map(|r| r.open_issues_count).sum();

        // DISPLAY TOP 10
        println!("--------------------------------------------");
        println!("Language: {}", lang);
        println!("Total stars: {}", total_stars);
        println!("Total forks: {}", total_forks);
        println!("Top-3 Most modified file per repo");
        for repo in repos {
            for (index, (filename, count)) in repo.top_modified_files.iter().enumerate() {
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
    Ok(())
}
