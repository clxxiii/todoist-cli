use std::env::Args;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn run(input: &mut Args, process_name: String) {
    let token: String = match input.nth(0) {
        Some(token) => token,
        None => {
            println!("Usage: {} login [token]", process_name);
            println!("\tReplace [token] with your todoist API token");
            std::process::exit(1)
        }
    };

    let token_file = match File::create(&crate::token_path()) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };
    let mut writer = BufWriter::new(token_file);

    match writer.write(token.as_bytes()) {
        Ok(_) => println!("Sucessfully saved token to ~/.todoist_token"),
        Err(err) => panic!("{}", err),
    }
}
