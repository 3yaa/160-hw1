mod api;
mod models;
mod storage;

use models::fork;
use models::repo;
use std::error::Error;
use redis::{AsyncCommands, RedisResult};
use std::collections::HashMap;

// Purpose:
// get the forks using GitHub API
// inputs:
// @repo : use member variables for GitHub API calls
// modified:
// @fork_commit_count : an int that holds the total commits among the forks of a given @repo
async fn get_forks_info(repo: &mut repo::Repo) {
    // call GitHub API call for forks
    match api::fetch_forks(&repo.owner_login, &repo.name).await {
        Ok(forks_json) => { 
            // declare vector of Forks with parsed forks from json 
            let forks = fork::parse_forks(forks_json);
            // @fork_commit_count : get the number of new commits in the forked repos
            let mut fork_commit_count: u64 = 0;
            // fetch recent commits
            for fork in &forks {
                // TODO: CHANGE THIS TO 20 LATER 
                // get the 20 most recent forked repos using GitHub API call
                match api::fetch_commits(&fork.owner_login, &fork.name, 10).await {
                    Ok(commits) => {
                        fork_commit_count += commits.len() as u64;
                    }
                    Err(e) => {
                        println!("---->error fetching commits: {}", e);
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
async fn get_commits_info(repo: &mut repo::Repo) {
    // make GitHub API call for 50 most recent commits
    match api::fetch_commits(&repo.owner_login, &repo.name, 50).await {
        Ok(commits) => {
            // declare HashMap file_counts to keep track of what files exist and how many times
            // they were edited
            use std::collections::HashMap; // why here?
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

            // keep top 3
            repo.top_modified_files.truncate(3);
        }
        Err(e) => {
            println!("   Error fetching commits: {}", e);
        }
    }
}


// main function:
// makes calls to helper functions:
//      get_forks_info()
//      get_commits_info()
//
// afterwards, print out relevant information regarding the repos 
//
// then, clone and inspect repositories
//
// lastly, upload the repository details to Redis 
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

    // ################### Clone and Inspect Repos ################### //


    // feel free to comment this part out after Part C
    // // declare Redis client
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_multiplexed_async_connection().await?;

    // // structure:
    // // first parameter is the key (String), for us probably the repoName
    // // second parameter is a list of tuples : &[(String, String), (String, String) ...]
    // // essentially, first String of the tuple is like the key
    // // second String of the tuple is like the value
    // 
    // let _: () = con.hset_multiple(
    //     "reponame:r1",
    //     &[
    //         ("url", "https://github.com/repos/r1"),
    //         ("owner", "owner:u1"),
    //         // add relevant tuples here depending on part C
    //     ],
    // )
    // .await?;

    // // Retrieve all fields
    // let repo_info: HashMap<String, String> = con.hgetall("reponame:r1").await?;

    // println!("Repo info: {:?}", repo_info);


    Ok(())
}
