use anyhow::Result;
use git2::Repository;
use std::{env, fs};

use crate::constants::DEFAULT_BUILDWISE_PATH;

pub fn add_repo(url: String) -> Result<()> {
    let path = env::var("BUILDWISE_PATH").unwrap_or(DEFAULT_BUILDWISE_PATH.into()) + "/repos";
    Repository::clone(&url, path)?;

    Ok(())
}

pub fn list_repos() -> Result<Vec<String>> {
    let path = env::var("BUILDWISE_PATH").unwrap_or(DEFAULT_BUILDWISE_PATH.into()) + "/repos";
    let dir = fs::read_dir(path)?;

    let mut repos = Vec::new();
    for repo in dir {
        repos.push(repo?.path().display().to_string());
    }

    Ok(repos)
}

pub fn delete_repo(name: String) -> Result<()> {
    let path = env::var("BUILDWISE_PATH").unwrap_or(DEFAULT_BUILDWISE_PATH.into()) + "/repos";

    fs::remove_dir_all(path + "/" + &name)?;
    Ok(())
}
