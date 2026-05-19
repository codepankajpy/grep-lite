use std::{env, fs};
use grep_lite::config::Config;

fn main() {
    // collect command line arguments in a vector of string type
    let env_args: Vec<String> = env::args().collect();

    // passing the slice of the vector to avoid trasferring ownership
    let config = Config::build(&env_args);

    // lowercase the query
    let query = config.query.to_lowercase(); // fix this

    // read file contents into single string and lowercase the contents
    let contents: String  = read_from_file(config.path).to_lowercase(); // fix this

    // search each line for query
    find_from_data(&contents, &query);
}

// &str is preffered over &string for read-only string borrowing 
fn read_from_file(path: &str) -> String {
    // read entire file into memory 
    fs::read_to_string(path).expect("Error reading from file!!")
}

fn find_from_data(data: &str, query: &str) {
    // .lines() creates and iterator over each line
    let mut index = 1;
    for line in data.lines() {
        if line.contains(query){
            println!("{index}  {line}");
        }
        index += 1;
    }
}