use clap::Parser;
use std::{env, path::PathBuf};

fn default_dir() -> PathBuf {
    env::current_dir().unwrap()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(default_value = default_dir().into_os_string())]
    pub cwdir: PathBuf,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
