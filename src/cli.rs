use clap::Parser;
use std::{env, path::PathBuf};

fn default_log_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    println!("Current path: {:?}", path);
    path.pop();
    path.push("log/debug.log");
    println!("Default log path: {:?}", path);
    path
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(default_value = default_log_path().into_os_string())]
    pub log_path: PathBuf,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
