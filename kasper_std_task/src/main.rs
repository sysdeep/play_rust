use std::{env, process};

mod application;
mod config;

/**
 * read file, parse, calculate, print
 */
fn main() {
    let args_raw: Vec<String> = env::args().collect();
    let args = config::Config::build_from_args(&args_raw).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("file_path: {}", args.file_path);

    application::run(args);
}
