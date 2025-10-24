use std::path::Path;
use git2::{Repository, FetchOptions, build::RepoBuilder};

pub fn looks_like_source(files: &[String]) -> bool {
    let code_extentions = [".rs", ".java", ".c", ".cpp", ".h"];
    let non_code = ["tutorial", "example", "readme"];

    let mut has_code = false;
    let mut not_source = false;

    for f in files {
        let name = f.to_lowercase();
        if code_extensions.iter().any(|code| name.ends_with(code)) {
            has_code = true;
        }
        if doc_keywords.iter().any(|not_code| name.contains(not_code)) {
            not_source = true;
        }
    }

    has_code && !not_source
}

pub fn clone_repo(url: &str, dest: &Path) -> Result<Repository, git2::Error> {
    let mut fo = FetchOptions::new();
    // clone with depth 1
    fo.depth(1);
    let mut build = git2::build::RepoBuilder::new();
    build.fetch_options(fo);
    build.clone(url, dest)
}
