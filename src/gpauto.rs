use crate::cli::CliArgs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Application {
    cur_dir: PathBuf,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub fn run(&self) {
        println!("Current directory: {:?}", self.cur_dir);
    }
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
