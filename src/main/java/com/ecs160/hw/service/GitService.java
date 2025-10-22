package com.ecs160.hw.service;

import com.ecs160.hw.model.Repo;
import com.ecs160.hw.model.Commit;
import com.ecs160.hw.model.Issue;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import com.google.gson.JsonElement;

import java.util.List;
import java.util.ArrayList;

public class GitService {

    private final HttpClient client;
    
    public GitService() {
        this.client = HttpClient.newHttpClient();
    }

    // get top 10 repos by language
    public List<Repo> getRepos(String language) throws Exception {
        String url = "https://api.github.com/search/repositories?q=language:" + language + "&sort=stars&order=desc&per_page=10";
        JsonObject response = sendGetRequest(url).getAsJsonObject();
        JsonArray items = response.getAsJsonArray("items");

        List<Repo> repos = new ArrayList<>();
        for (JsonElement e : items) {
            
            JsonObject repoJson = e.getAsJsonObject();
            String name = repoJson.get("name").getAsString();
            //String ownerLogin = repoJson.get("owner").get("login").getAsString();
            String ownerLogin = "temp";
            String htmlUrl = repoJson.get("html_url").getAsString();
            int forks_count = repoJson.get("forks_count").getAsInt();
            int stargazer_count = repoJson.get("stargazers_count").getAsInt();
            String lang = repoJson.get("language").getAsString();
            int open_issues_count = repoJson.get("open_issues_count").getAsInt();
            
            // very specific constructor; might change later
            Repo temp = new Repo(name, ownerLogin, htmlUrl, forks_count, stargazer_count, lang, open_issues_count);
            repos.add(temp);
            
        }
        return repos;
    }

    // get commits
    public List<Commit> getCommits(String owner, String repo) throws Exception {
        String url = "https://api.github.com/repos/" + owner + "/" + repo + "/commits?per_page=50";
        JsonArray commitsArray = sendGetRequest(url).getAsJsonArray();

        List<Commit> commits = new ArrayList<>();
        for (JsonElement e : commitsArray) {
            JsonObject commitObj = e.getAsJsonObject();
            String sha = commitObj.get("sha").getAsString();
            
            Commit temp = new Commit(sha);
            commits.add(temp);
        }
        return commits;
    }

    // GET
    private JsonElement sendGetRequest(String url) throws Exception {
        HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create(url))
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .GET()
                .build();

        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        if (response.statusCode() == 200) {
            return JsonParser.parseString(response.body());
        } else {
            throw new RuntimeException("GitHub API call failed: " + response.statusCode() + " Response: " + response.body());
        }
    }
}




