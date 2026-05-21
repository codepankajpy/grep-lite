use std::fs;
use grep_lite::{cli::config::Config, print::output};
use grep_lite::search::search;
use clap::Parser;

fn main() {

    let args = Config::parse();

    let contents = read_from_file(&args.path);

    let ans= search(&contents, &args.query);

    output(ans, &args.query);

}

// &str is preffered over &string for read-only string borrowing 
fn read_from_file(path: &str) -> String {
    // read entire file into memory 
    fs::read_to_string(path).expect("Error reading from file!!")
}

