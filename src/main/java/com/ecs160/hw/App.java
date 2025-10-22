package com.ecs160.hw;

import com.ecs160.hw.model.Repo;
import com.ecs160.hw.model.Commit;
import com.ecs160.hw.model.Issue;
import com.ecs160.hw.service.GitService;

import java.util.List;

public class App 
{
    public static void main( String[] args ) {
        GitService service = new GitService();
        try {
            List<Repo> test = service.getRepos("rust");
            // System.out.println("good");
            for (Repo repo : test) {
                System.out.println("Name: " + repo.getName());
                System.out.println("Owner: " + repo.getOwnerLogin());
                System.out.println("URL: " + repo.getHtmlUrl());
                System.out.println("Forks: " + repo.getForksCount());
                System.out.println("Stars: " + repo.getStargazersCount());
                System.out.println("Language: " + repo.getLanguage());
                System.out.println("Open Issues: " + repo.getOpenIssuesCount());
                System.out.println("Commits: " + repo.getCommitCount());
                System.out.println("----");
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
