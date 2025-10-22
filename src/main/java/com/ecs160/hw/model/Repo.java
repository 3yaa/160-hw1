package com.ecs160.hw.model;

import java.util.List;
import java.util.ArrayList;

public class Repo {
    private String name;
    private String ownerLogin;
    private String htmlUrl;
    private int forksCount;
    private int stargazers_count; // make setter getter later
    private String language;
    private int openIssuesCount;
    private int commitCount;
    private List<Repo> forks;
    private List<Commit> recentCommits;
    private List<Issue> issues;

    // constructors
    public Repo() {
        this.name = "";
        this.ownerLogin = "";
        this.htmlUrl = "";
        this.forksCount = 0;
        this.stargazers_count = 0;
        this.language = "";
        this.openIssuesCount = 0;
        this.commitCount = 0;
        this.forks = new ArrayList<>();
        this.recentCommits = new ArrayList<>();
        this.issues = new ArrayList<>();
    }
    
    // this constructor is very specific in the gitservice.java might change later on
    public Repo(String name, String ownerLogin, String htmlUrl, int forksCount,
                int stargazers_count, String language, int openIssuesCount) {
        this.name = name;
        this.ownerLogin = ownerLogin;
        this.htmlUrl = htmlUrl;
        this.forksCount = forksCount;
        this.stargazers_count = stargazers_count;
        this.language = language;
        this.openIssuesCount = openIssuesCount;
    }

    // setters
    public void setName(String name) { this.name = name; }
    public void setOwnerLogin(String ownerLogin) { this.ownerLogin = ownerLogin; }
    public void setHtmlUrl(String url) { this.htmlUrl = url; }
    public void setForksCount(int forksCount) { this.forksCount = forksCount; }
    public void setStargazersCount(int stargazers_count) { this.stargazers_count = stargazers_count; }
    public void setLanguage(String language) { this.language = language; }
    public void setOpenIssuesCount(int openIssuesCount) { this.openIssuesCount = openIssuesCount; }
    public void setCommitCount(int count) { this.commitCount = count; }
    public void setForks(List<Repo> forks) { this.forks = forks; }
    public void setRecentCommits(List<Commit> recentCommits) { this.recentCommits = recentCommits; }
    public void setIssues(List<Issue> issues) { this.issues = issues; }

    // getters
    public String getName() { return name; }
    public String getOwnerLogin() { return ownerLogin; }
    public String getHtmlUrl() { return htmlUrl; }
    public int getForksCount() { return forksCount; }
    public int getStargazersCount() { return stargazers_count; }
    public String getLanguage() { return language; }
    public int getOpenIssuesCount() { return openIssuesCount; }
    public int getCommitCount() { return commitCount; }
    public List<Repo> getForks() { return forks; }
    public List<Commit> getRecentCommits() { return recentCommits; }
    public List<Issue> getIssues() { return issues; }

}
