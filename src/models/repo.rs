use serde_json::Value;

pub struct Repo {
    pub name: String,
    pub owner_login: String,
    #[allow(dead_code)]
    pub html_url: String,
    #[allow(dead_code)]
    pub language: String,
    // stats
    pub stars: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    //
    pub top_modified_files: Vec<(String, u32)>,
    pub fork_commit_count: u64,
}

pub fn parse_repo(json: &Value) -> Option<Repo> {
    Some(Repo {
        name: json["name"].as_str()?.to_string(),
        owner_login: json["owner"]["login"].as_str()?.to_string(),
        html_url: json["html_url"].as_str()?.to_string(),
        language: json["language"].as_str().unwrap_or("Unknown").to_string(),
        stars: json["stargazers_count"].as_u64()?,
        forks_count: json["forks_count"].as_u64()?,
        open_issues_count: json["open_issues_count"].as_u64()?,
        // modify later
        top_modified_files: Vec::new(),
        fork_commit_count: 0,
    })
}

pub fn parse_repos(json_array: Vec<Value>) -> Vec<Repo> {
    json_array
        .iter()
        .filter_map(|json| parse_repo(json))
        .collect()
}
