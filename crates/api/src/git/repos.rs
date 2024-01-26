use anyhow::{Context, Result};
use std::{env, fs};

use crate::constants::DEFAULT_BUILDWISE_PATH;

pub fn add_repo(url: String) -> Result<()> {
    let repo_name = get_repo_name_from_url(&url)?;
    let path = env::var("BUILDWISE_PATH").unwrap_or(DEFAULT_BUILDWISE_PATH.into()) + "/repos/" + &repo_name;
    git2::Repository::clone(&url, path)?;

    Ok(())
}

pub fn list_repos() -> Result<Vec<String>> {
    let path = env::var("BUILDWISE_PATH").unwrap_or(DEFAULT_BUILDWISE_PATH.into()) + "/repos";
    let org_dir = fs::read_dir(&path)?;

    let mut repos = Vec::new();
    for org in org_dir {
        let repo_dir = fs::read_dir(&org?.path().display().to_string())?;
        for repo in repo_dir {
            let sanitized_name = repo?.path().display().to_string().replace(&(path.clone() + &"/"), "");
            repos.push(sanitized_name);
        }
    }

    Ok(repos)
}

pub fn delete_repo(name: String) -> Result<()> {
    let path = env::var("BUILDWISE_PATH").unwrap_or(DEFAULT_BUILDWISE_PATH.into()) + "/repos";

    fs::remove_dir_all(path + "/" + &name)?;
    Ok(())
}

fn get_repo_name_from_url(url: &String) -> Result<String> {
    let url_standardized = url.replace(":", "/");
    let chunks: Vec<_> = url_standardized.split("/").collect();
    let penult_repo_name = *chunks.iter().nth_back(1).context("Can't fetch penult")?;
    let last_repo_name = *chunks.last().context("Can't fetch last chunk")?;
    let repo_name = penult_repo_name.to_string() + "/" + last_repo_name.into();
    
    Ok(repo_name)
}
