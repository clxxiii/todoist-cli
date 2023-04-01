use std::env::{args, Args};
mod commands;

fn main() {
    let mut args: Args = args();
    let process_name = args.nth(0).unwrap();
    commands::handle(&mut args, process_name);
}
