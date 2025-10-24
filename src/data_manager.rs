use crate::api;
use crate::models::fork;
use crate::models::repo;
use std::collections::HashMap;

// Purpose:
// get the forks using GitHub API
// inputs:
// @repo : use member variables for GitHub API calls
// modified:
// @fork_commit_count : an int that holds the total commits among the forks of a given @repo
pub async fn get_forks_info(repo: &mut repo::Repo) {
    // call GitHub API call for forks
    match api::fetch_forks(&repo.owner_login, &repo.name).await {
        Ok(forks_json) => {
            // declare vector of Forks with parsed forks from json
            let forks = fork::parse_forks(forks_json);
            // @fork_commit_count : get the number of new commits in the forked repos
            let mut fork_commit_count: u64 = 0;
            // fetch recent commits
            for fork in &forks {
                // get up to 10 commits for each fork using GitHub API call
                match api::fetch_commits(&fork.owner_login, &fork.name, 10).await {
                    Ok(commits) => {
                        fork_commit_count += commits.len() as u64;
                    }
                    Err(e) => {
                        // silently skip 404s (deleted/private forks)
                        if !e.to_string().contains("404") {
                            println!(
                                "---->error fetching commits for {}/{}: {}",
                                fork.owner_login, fork.name, e
                            );
                        }
                    }
                }
            }
            // save fork_commit_count into repo
            repo.fork_commit_count = fork_commit_count;
        }
        Err(e) => {
            println!("---->error fetching forks: {}", e);
        }
    }
}

// Purpose:
// get the commits info using GitHub API
// inputs:
// @repo : use member variables for GitHub API calls
// modified:
// @top_modified_files : set to the top 3 modified files of a given @repo
pub async fn get_commits_info(repo: &mut repo::Repo) {
    // make GitHub API call for 50 most recent commits
    match api::fetch_commits(&repo.owner_login, &repo.name, 50).await {
        Ok(commits) => {
            // declare HashMap file_counts to keep track of what files exist and how many times
            // they were edited
            let mut file_counts: HashMap<String, u32> = HashMap::new();

            // go through each commit and extract the sha
            // using the sha, use the GitHub API to grab the modified files of said commit
            // update hashmap accordingly and increment or initialize depending on if it exists or not respectively
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
        }
        Err(e) => {
            println!("----->error fetching commits: {}", e);
        }
    }
}

pub fn compute_stars(repos: &[repo::Repo]) -> u64 {
    repos.iter().map(|r| r.stars).sum()
}

pub fn compute_forks(repos: &[repo::Repo]) -> u64 {
    repos.iter().map(|r| r.forks_count).sum()
}

pub fn compute_fork_commits(repos: &[repo::Repo]) -> u64 {
    repos.iter().map(|r| r.fork_commit_count).sum()
}

pub fn compute_open_issues(repos: &[repo::Repo]) -> u64 {
    repos.iter().map(|r| r.open_issues_count).sum()
}

pub fn compute_top_three(repo: &repo::Repo) -> Vec<(String, u32)> {
    repo.top_modified_files.iter().take(3).cloned().collect()
}

pub fn display_stat(repos: &[repo::Repo], lang: &str) {
    // ADD UP FOR TOP 10
    let total_stars = compute_stars(&repos);
    let total_forks = compute_forks(&repos);
    let total_fork_commits = compute_fork_commits(&repos);
    let total_open_issues = compute_open_issues(&repos);

    // DISPLAY TOP 10
    println!("Language: {}", lang);
    println!("Total stars: {}", total_stars);
    println!("Total forks: {}", total_forks);
    println!("Top-3 Most modified file per repo");
    for repo in repos {
        println!("Repo name: {}", repo.name);
        // display top 3 of the files
        let top_three_files = compute_top_three(repo);
        for (index, (filename, count)) in top_three_files.iter().enumerate() {
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
