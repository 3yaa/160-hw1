use redis::AsyncCommands;
use std::collections::HashMap;

use crate::models::repo;

pub async fn store_redis(repo: &repo::Repo) {
    // declare Redis client
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create Redis client: {:?}", e);
            return;
        }
    };

    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to get async connection: {:?}", e);
            return;
        }
    };

    let res: Result<(), _> = con
        .hset_multiple(
            repo.name.clone(),
            &[
                ("owner", repo.owner_login.clone()),
                ("url", repo.html_url.clone()),
                ("language", repo.language.clone()),
                ("default_branch", repo.default_branch.clone()),
                ("stars", repo.stars.to_string()),
                ("forks_count", repo.forks_count.to_string()),
                ("open_issues_count", repo.open_issues_count.to_string()),
                ("fork_commit_count", repo.fork_commit_count.to_string()),
            ],
        )
        .await;

    if let Err(e) = res {
        eprintln!("Failed to set values in Redis: {:?}", e);
        return;
    }

    let repo_info: HashMap<String, String> = match con.hgetall(&repo.name).await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to get values from Redis: {:?}", e);
            return;
        }
    };

    println!("Repo info: {:?}", repo_info);
}

// save to disk
pub async fn save_redis_to_disk() {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create Redis client: {:?}", e);
            return;
        }
    };

    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to get async connection: {:?}", e);
            return;
        }
    };

    // force redis to save to disk
    let _: Result<String, _> = redis::cmd("BGSAVE").query_async(&mut con).await;
    println!("Redis data saved to disk");
}
