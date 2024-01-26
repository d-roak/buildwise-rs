use anyhow::Result;
use std::env;
use walkdir::WalkDir;

use crate::constants::DEFAULT_BUILDWISE_PATH;

pub fn get_src_files(repo: String) -> Result<Vec<String>> {
    let path = env::var("BUILDWISE_PATH").unwrap_or(DEFAULT_BUILDWISE_PATH.into()) + "/repos/" + &repo;

    let mut files_paths = Vec::new();
    for entry in WalkDir::new(&path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.path().to_string_lossy();
        if f_name.ends_with(".sol") || f_name.ends_with(".cairo") {
            let sanitized_path = f_name.replace(&path, "");
            files_paths.push(sanitized_path);
        }
    }

    Ok(files_paths)
}
