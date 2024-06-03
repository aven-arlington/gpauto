use crate::cli::CliArgs;
use std::fs;
use std::io;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, PartialEq)]
pub struct Application {
    cur_dir: PathBuf,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub fn run(&self) {
        println!("Current working directory: {:?}", self.cur_dir);

        let repos: Vec<PathBuf> = get_repos(&self.cur_dir);
        println!("Found repos: {:?}", repos.len());
    }
}

fn get_repos(dir: &PathBuf) -> Vec<PathBuf> {
    let mut found_repos: Vec<PathBuf> = vec![];
    if dir.is_dir() {
        let found_files: Vec<DirEntry> = WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|f| f.file_name() == ".git")
            .collect();
        println!("Found files: {:?}", found_files.len());
        for file in found_files {
            if file.file_name() == ".git" {
                found_repos.push(file.path().to_path_buf());
            }
        }
    }
    found_repos
}

#[derive(Default)]
pub struct ApplicationBuilder {
    cur_dir: PathBuf,
}

impl ApplicationBuilder {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            cur_dir: PathBuf::new(),
        }
    }

    pub fn build(self) -> Application {
        Application {
            cur_dir: self.cur_dir,
        }
    }

    pub fn set_cli_arguments(mut self, args: CliArgs) -> ApplicationBuilder {
        self.cur_dir = args.cwdir;
        self
    }
}
