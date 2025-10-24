use crate::api;
use crate::repo;
use git2::{FetchOptions, build::RepoBuilder};
use std::error::Error;
use std::fs;
use std::path::Path;

fn clone_repo(url: &str, lang: &str, repo_name: &str) -> Result<(), Box<dyn Error>> {
    let path = format!("repo_cloned/{}_{}", lang, repo_name);

    if Path::new(&path).exists() {
        fs::remove_dir_all(&path)?;
    }

    let mut fetch_options = FetchOptions::new();
    fetch_options.depth(1);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    builder
        .clone(url, Path::new(&path))
        .map_err(|e| format!("Failed to clone repository: {}", e))?;

    Ok(())
}
//
async fn analyze_repo(
    owner: &str,
    repo_name: &str,
    default_branch: &str,
) -> Result<bool, Box<dyn Error>> {
    // all the valid code extentions
    let valid_code_exts = [".rs", ".java", ".c", ".cpp", ".h"];

    // init
    let mut total_files = 0;
    let mut code_files = 0;

    // make api call to fetch repo tree
    let tree = api::fetch_repo_tree(owner, repo_name, default_branch).await?;

    for item in &tree {
        // check if cur item is a file -- github ids file as "blob"
        if item["type"].as_str() == Some("blob") {
            total_files += 1;
            // check if cur item has an extention ending in valid_code_exts
            if let Some(path) = item["path"].as_str() {
                if valid_code_exts.iter().any(|ext| path.ends_with(ext)) {
                    code_files += 1;
                }
            }
        }
    }

    // if no files just skip
    if total_files == 0 {
        return Ok(false);
    }

    // check a threadhold of 70% to count as a non tutorial/docuement repo
    let percentage = (code_files * 100) / total_files;
    Ok(percentage >= 70)
}

pub async fn clone_top_repo(repos: &[repo::Repo]) {
    for repo in repos {
        let valid_repo_to_clone =
            analyze_repo(&repo.owner_login, &repo.name, &repo.default_branch).await;

        match valid_repo_to_clone {
            Ok(true) => {
                if let Err(e) = clone_repo(&repo.html_url, &repo.language, &repo.name) {
                    println!("----->error failed to clone {}: {}", repo.name, e);
                }
            }
            Ok(false) => continue,
            Err(e) => {
                println!("----->error failed to analyze {}: {}", repo.name, e);
            }
        }
    }
}
