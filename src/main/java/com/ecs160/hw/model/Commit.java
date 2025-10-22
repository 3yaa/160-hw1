package com.ecs160.hw.model;

import java.util.List;

public class Commit {
    private List<String> files;
    private String sha; // make setter getter later

    // constructor temp
    public Commit(String sha) {
        this.sha = sha;
    }

    // setters
    // public void setFileName(String fileName) { this.fileName = fileName; }

    // getters
    // public String getFileName() { return fileName; }

}
