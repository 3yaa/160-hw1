use hw1::models::fork::{parse_fork, parse_forks};
use hw1::models::repo::{Repo, parse_repo, parse_repos};
use hw1::data_manager::{compute_stars, compute_forks, compute_fork_commits, compute_open_issues, compute_top_three};
use hw1::redis::{store_redis};
use serde_json::json;

// TEST 1: parse_repo()
// parse through a valid json
#[test]
fn test_parse_repo_valid_json() {
    let data = json!({
        "name": "example-repo",
        "owner": {"login":"example_user"},
        "html_url": "www.testing.link",
        "language": "Rust",
        "default_branch" : "main",
        "stargazers_count": 160,
        "forks_count": 16,
        "open_issues_count": 60
    });

    let repo = parse_repo(&data).expect("Failed to parse repo");

    assert_eq!(repo.name, "example-repo");
    assert_eq!(repo.owner_login, "example_user");
    assert_eq!(repo.html_url, "www.testing.link");
    assert_eq!(repo.language, "Rust");
    assert_eq!(repo.default_branch, "main");
    assert_eq!(repo.stars, 160);
    assert_eq!(repo.forks_count, 16);
    assert_eq!(repo.open_issues_count, 60);
}

// parse through a valid json with missing language
#[test]
fn test_parse_repo_valid_json_no_language() {
    let data = json!({
        "name": "a-new-repo-example",
        "owner": {"login":"no-language-user"},
        "html_url": "https://no-language-found.net",
        "default_branch" : "main",
        "stargazers_count": 67,
        "forks_count": 21,
        "open_issues_count": 17
    });

    let repo = parse_repo(&data).expect("Failed to parse repo");

    assert_eq!(repo.name, "a-new-repo-example");
    assert_eq!(repo.owner_login, "no-language-user");
    assert_eq!(repo.html_url, "https://no-language-found.net");
    assert_eq!(repo.language, "Unknown");
    assert_eq!(repo.default_branch, "main");
    assert_eq!(repo.stars, 67);
    assert_eq!(repo.forks_count, 21);
    assert_eq!(repo.open_issues_count, 17);
}

// parse through a valid json with missing mandatory information
#[test]
fn test_parse_repo_valid_json_missing_field() {
    // name is important, it is used a lot
    // name will hence be missing from this json
    let data = json!({
        "owner": {"login":"ghost-user"},
        "html_url": "www.does-not-exist.com",
        "language": "Java",
        "stargazers_count": 1,
        "forks_count": 2,
        "open_issues_count": 93
    });

    assert!(parse_repo(&data).is_none());
}

// TEST 2: parse_repos()

// json list with valid jsons
#[test]
fn test_parse_repos_valid_input() {
    let valid_repo = serde_json::json!({
        "name": "valid-repo",
        "owner": { "login": "user" },
        "html_url": "www.valid_repo.com",
        "language": "Rust",
        "default_branch" : "main",
        "stargazers_count": 7,
        "forks_count": 12,
        "open_issues_count": 2
    });

    let another_valid_repo = serde_json::json!({
        "name": "super-valid-repo",
        "owner": { "login": "super-user" },
        "html_url": "www.super-valid_repo.com",
        "language": "Java",
        "default_branch" : "main",
        "stargazers_count": 11,
        "forks_count": 21,
        "open_issues_count": 6
    });

    let one_more_valid_repo = serde_json::json!({
        "name": "truly-valid-repo",
        "owner": { "login": "truly-user" },
        "html_url": "www.truly-valid_repo.com",
        "language": "C",
        "default_branch" : "main",
        "stargazers_count": 2,
        "forks_count": 31,
        "open_issues_count": 7
    });

    let jsons = vec![valid_repo, another_valid_repo, one_more_valid_repo];

    let repos = parse_repos(jsons);

    assert_eq!(repos.len(), 3);
    assert_eq!(repos[0].name, "valid-repo");
    assert_eq!(repos[0].owner_login, "user");
    assert_eq!(repos[0].html_url, "www.valid_repo.com");
    assert_eq!(repos[0].language, "Rust");
    assert_eq!(repos[0].default_branch, "main");
    assert_eq!(repos[0].stars, 7);
    assert_eq!(repos[0].forks_count, 12);
    assert_eq!(repos[0].open_issues_count, 2);

    assert_eq!(repos[1].name, "super-valid-repo");
    assert_eq!(repos[1].owner_login, "super-user");
    assert_eq!(repos[1].html_url, "www.super-valid_repo.com");
    assert_eq!(repos[1].language, "Java");
    assert_eq!(repos[1].default_branch, "main");
    assert_eq!(repos[1].stars, 11);
    assert_eq!(repos[1].forks_count, 21);
    assert_eq!(repos[1].open_issues_count, 6);

    assert_eq!(repos[2].name, "truly-valid-repo");
    assert_eq!(repos[2].owner_login, "truly-user");
    assert_eq!(repos[2].html_url, "www.truly-valid_repo.com");
    assert_eq!(repos[2].language, "C");
    assert_eq!(repos[2].default_branch, "main");
    assert_eq!(repos[2].stars, 2);
    assert_eq!(repos[2].forks_count, 31);
    assert_eq!(repos[2].open_issues_count, 7);
}

// json list with an invalid json
#[test]
fn test_parse_repos_mixed_input() {
    let valid_repo = serde_json::json!({
        "name": "valid-repo",
        "owner": { "login": "user" },
        "html_url": "www.valid_repo.com",
        "language": "Rust",
        "default_branch" : "main",
        "stargazers_count": 7,
        "forks_count": 12,
        "open_issues_count": 2
    });

    // json with many missing fields
    let invalid_repo = serde_json::json!({
        "name": "invalid-repo",
    });

    let another_valid_repo = serde_json::json!({
        "name": "another-valid-repo",
        "owner": { "login": "another-user" },
        "html_url": "www.another-valid_repo.com",
        "language": "Java",
        "default_branch" : "main",
        "stargazers_count": 27,
        "forks_count": 9,
        "open_issues_count": 31
    });

    let jsons = vec![valid_repo, invalid_repo, another_valid_repo];

    let repos = parse_repos(jsons);

    assert_eq!(repos.len(), 2);
    assert_eq!(repos[0].name, "valid-repo");
    assert_eq!(repos[0].owner_login, "user");
    assert_eq!(repos[0].html_url, "www.valid_repo.com");
    assert_eq!(repos[0].language, "Rust");
    assert_eq!(repos[0].default_branch, "main");
    assert_eq!(repos[0].stars, 7);
    assert_eq!(repos[0].forks_count, 12);
    assert_eq!(repos[0].open_issues_count, 2);

    assert_eq!(repos[1].name, "another-valid-repo");
    assert_eq!(repos[1].owner_login, "another-user");
    assert_eq!(repos[1].html_url, "www.another-valid_repo.com");
    assert_eq!(repos[1].language, "Java");
    assert_eq!(repos[1].default_branch, "main");
    assert_eq!(repos[1].stars, 27);
    assert_eq!(repos[1].forks_count, 9);
    assert_eq!(repos[1].open_issues_count, 31);
}

// TEST 3: parse_fork()

// valid json
#[test]
fn test_parse_fork_valid_json() {
    let data = json!({
        "name": "fork-spoon-knife",
        "owner": {
            "login": "fork-owner"
        }
    });

    let fork = parse_fork(&data).expect("Failed to parse fork");

    assert_eq!(fork.name, "fork-spoon-knife");
    assert_eq!(fork.owner_login, "fork-owner");
}

// valid json, but owner is missing login
#[test]
fn test_parse_fork_valid_json_missing_owner() {
    let data = json!({
        "name": "fork-spoon-knife",
        "owner": { "not_login": "not_fork-owner" }
    });

    assert!(parse_fork(&data).is_none());
}

// valid json, but missing name field
#[test]
fn test_parse_fork_valid_json_missing_name() {
    let data = json!({
        "owner": { "login": "no_name"}
    });

    assert!(parse_fork(&data).is_none());
}

// TEST 4: parse_forks()

// valid json array
#[test]
fn test_parse_forks_valid_json() {
    let fork_one = json!({
        "name": "valid-user-one",
        "owner": { "login": "user-one" }
    });

    let fork_two = json!({
        "name": "valid-user-two",
        "owner": {"login": "user-two"}
    });

    let fork_three = json!({
        "name": "valid-user-three",
        "owner": {"login": "user-three"}
    });

    let json_array = vec![fork_one, fork_two, fork_three];

    let forks = parse_forks(json_array);
    assert_eq!(forks.len(), 3);
    assert_eq!(forks[0].name, "valid-user-one");
    assert_eq!(forks[0].owner_login, "user-one");
    assert_eq!(forks[1].name, "valid-user-two");
    assert_eq!(forks[1].owner_login, "user-two");
    assert_eq!(forks[2].name, "valid-user-three");
    assert_eq!(forks[2].owner_login, "user-three");
}

// json array with one invalid json
#[test]
fn test_parse_forks_invalid_json() {
    let fork_one = json!({
        "name": "valid-user-one",
        "owner": {"login": "user-one"}
    });

    let fork_two = json!({
        "name": "invalid-user-two",
        "owner": {}
    });

    let fork_three = json!({
        "name": "valid-user-three",
        "owner": {"login": "user-three"}
    });

    let json_array = vec![fork_one, fork_two, fork_three];

    let forks = parse_forks(json_array);
    assert_eq!(forks.len(), 2);
    assert_eq!(forks[0].name, "valid-user-one");
    assert_eq!(forks[0].owner_login, "user-one");
    assert_eq!(forks[1].name, "valid-user-three");
    assert_eq!(forks[1].owner_login, "user-three");
}

// TEST 5: compute_stars()

// valid stars
#[test]
fn test_compute_stars() {
    let repos = vec![
        Repo {
            name: "repo1".to_string(),
            owner_login: "owner1".to_string(),
            html_url: "https://github.com/owner1/repo1".to_string(),
            language: "Rust".to_string(),
            default_branch: "main".to_string(),
            stars: 10,
            forks_count: 5,
            open_issues_count: 2,
            top_modified_files: vec![],
            fork_commit_count: 6,
        },
        Repo {
            name: "repo2".to_string(),
            owner_login: "owner2".to_string(),
            html_url: "https://github.com/owner2/repo2".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 15,
            forks_count: 3,
            open_issues_count: 1,
            top_modified_files: vec![],
            fork_commit_count: 2,
        },
        Repo {
            name: "repo3".to_string(),
            owner_login: "owner3".to_string(),
            html_url: "https://github.com/owner2/repo3".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 19,
            forks_count: 2,
            open_issues_count: 11,
            top_modified_files: vec![],
            fork_commit_count: 4,
        },
    ];
    
    let total_stars = compute_stars(&repos);
    assert_eq!(total_stars, 44);

}

// TEST 6: compute_forks()

// valid forks
#[test]
fn test_compute_forks() {
    let repos = vec![
        Repo {
            name: "repo1".to_string(),
            owner_login: "owner1".to_string(),
            html_url: "https://github.com/owner1/repo1".to_string(),
            language: "Rust".to_string(),
            default_branch: "main".to_string(),
            stars: 10,
            forks_count: 5,
            open_issues_count: 2,
            top_modified_files: vec![],
            fork_commit_count: 6,
        },
        Repo {
            name: "repo2".to_string(),
            owner_login: "owner2".to_string(),
            html_url: "https://github.com/owner2/repo2".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 15,
            forks_count: 3,
            open_issues_count: 1,
            top_modified_files: vec![],
            fork_commit_count: 2,
        },
        Repo {
            name: "repo3".to_string(),
            owner_login: "owner3".to_string(),
            html_url: "https://github.com/owner2/repo3".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 19,
            forks_count: 2,
            open_issues_count: 11,
            top_modified_files: vec![],
            fork_commit_count: 4,
        },
    ];
    
    let total_forks = compute_forks(&repos);
    assert_eq!(total_forks, 10);

}

// TEST 7: compute_fork_commits

// valid fork_commits
#[test]
fn test_compute_fork_commits() {
    let repos = vec![
        Repo {
            name: "repo1".to_string(),
            owner_login: "owner1".to_string(),
            html_url: "https://github.com/owner1/repo1".to_string(),
            language: "Rust".to_string(),
            default_branch: "main".to_string(),
            stars: 10,
            forks_count: 5,
            open_issues_count: 2,
            top_modified_files: vec![],
            fork_commit_count: 6,
        },
        Repo {
            name: "repo2".to_string(),
            owner_login: "owner2".to_string(),
            html_url: "https://github.com/owner2/repo2".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 15,
            forks_count: 3,
            open_issues_count: 1,
            top_modified_files: vec![],
            fork_commit_count: 2,
        },
        Repo {
            name: "repo3".to_string(),
            owner_login: "owner3".to_string(),
            html_url: "https://github.com/owner2/repo3".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 19,
            forks_count: 2,
            open_issues_count: 11,
            top_modified_files: vec![],
            fork_commit_count: 4,
        },
    ];
    
    let total_fork_commits = compute_fork_commits(&repos);
    assert_eq!(total_fork_commits, 12);

}

// TEST 8: compute_open_issues()
// valid stars
#[test]
fn test_compute_open_issues() {
    let repos = vec![
        Repo {
            name: "repo1".to_string(),
            owner_login: "owner1".to_string(),
            html_url: "https://github.com/owner1/repo1".to_string(),
            language: "Rust".to_string(),
            default_branch: "main".to_string(),
            stars: 10,
            forks_count: 5,
            open_issues_count: 2,
            top_modified_files: vec![],
            fork_commit_count: 6,
        },
        Repo {
            name: "repo2".to_string(),
            owner_login: "owner2".to_string(),
            html_url: "https://github.com/owner2/repo2".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 15,
            forks_count: 3,
            open_issues_count: 1,
            top_modified_files: vec![],
            fork_commit_count: 2,
        },
        Repo {
            name: "repo3".to_string(),
            owner_login: "owner3".to_string(),
            html_url: "https://github.com/owner2/repo3".to_string(),
            language: "Java".to_string(),
            default_branch: "main".to_string(),
            stars: 19,
            forks_count: 2,
            open_issues_count: 11,
            top_modified_files: vec![],
            fork_commit_count: 4,
        },
    ];
    
    let total_open_issues = compute_open_issues(&repos);
    assert_eq!(total_open_issues, 14);

}

// TEST 9: compute_top_three()

// valid files
#[test]
fn test_compute_top_three() {
    let repo = Repo {
        name: String::from("repo1"),
        owner_login: String::from("owner1"),
        html_url: String::from("https://github.com/owner1/repo1"),
        language: String::from("Rust"),
        default_branch: "main".to_string(),
        stars: 10,
        forks_count: 5,
        open_issues_count: 2,
        top_modified_files: vec![
            ("file1.rs".to_string(), 5),
            ("file2.rs".to_string(), 3),
            ("file3.rs".to_string(), 2),
            ("file4.rs".to_string(), 1),
        ],
        fork_commit_count: 0,
    };

    let top_three = compute_top_three(&repo);

    assert_eq!(
        top_three,
        vec![
            ("file1.rs".to_string(), 5),
            ("file2.rs".to_string(), 3),
            ("file3.rs".to_string(), 2),
        ]
    );
}