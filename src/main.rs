use std::{env, fs};
use grep_lite::config::Config;
use grep_lite::print::output;
use grep_lite::search::search;

fn main() {
    // collect command line arguments in a vector of string type
    let env_args: Vec<String> = env::args().collect();

    // passing the slice of the vector to avoid trasferring ownership
    let config = Config::build(&env_args).expect("Failed to parse arguments");

    let contents = read_from_file(config.path);

    let ans= search(&contents, &config.query.to_lowercase());

    output(ans, &config.query);

}

// &str is preffered over &string for read-only string borrowing 
fn read_from_file(path: &str) -> String {
    // read entire file into memory 
    fs::read_to_string(path).expect("Error reading from file!!")
}

