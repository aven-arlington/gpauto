use crate::cli::CliArgs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Application {
    log_file: PathBuf,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub fn run(&self) {
        println!("Running application with log file: {:?}", self.log_file);
    }
}

#[derive(Default)]
pub struct ApplicationBuilder {
    log_file: PathBuf,
}

impl ApplicationBuilder {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            log_file: PathBuf::new(),
        }
    }

    pub fn build(self) -> Application {
        Application {
            log_file: self.log_file,
        }
    }

    pub fn set_cli_arguments(mut self, args: CliArgs) -> ApplicationBuilder {
        self.log_file = args.log_path;
        self
    }
}
