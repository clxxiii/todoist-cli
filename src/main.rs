use dirs;
use std::env::{args, Args};
use std::fs;
use std::path::PathBuf;
use tokio;
mod commands;

const BASE_URL: &str = "https://api.todoist.com/rest/v2/";

fn main() {
    let mut args: Args = args();
    let process_name = args.nth(0).unwrap();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(commands::handle(&mut args, process_name));
}

pub fn token_path() -> PathBuf {
    let mut home: PathBuf = match dirs::home_dir() {
        Some(a) => a,
        None => panic!("Cannot read home directory"),
    };
    home.push(".todoist_token");
    home
}

pub fn token() -> String {
    match fs::read_to_string(token_path()) {
        Ok(token) => token,
        Err(err) => {
            println!("{}", err);
            println!("HINT: Try using the login command first, and try again.");
            std::process::exit(1);
        }
    }
}
