use redis::{AsyncCommands, RedisResult};
use std::collections::HashMap;

pub async fn store_redis() -> redis::RedisResult<()> {
    // feel free to comment this part out after Part C
    // declare Redis client
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_multiplexed_async_connection().await?;

    // structure:
    // first parameter is the key (String), for us probably the repoName
    // second parameter is a list of tuples : &[(String, String), (String, String) ...]
    // essentially, first String of the tuple is like the key
    // second String of the tuple is like the value

    let _: () = con
        .hset_multiple(
            "reponame:r1",
            &[
                ("url", "https://github.com/repos/r1"),
                ("owner", "owner:u1"),
                // add relevant tuples here depending on part C
            ],
        )
        .await?;

    // Retrieve all fields
    let repo_info: HashMap<String, String> = con.hgetall("reponame:r1").await?;

    println!("Repo info: {:?}", repo_info);
}
