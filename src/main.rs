use std::{env, fs, process::exit};

// Here config stores borrowed references to command line arguments
// it does not own the strings
struct Config<'args> {
    path: &'args String,
    query: &'args String
}

fn main() {
    // collect command line arguments in a vector of string type
    let env_args: Vec<String> = env::args().collect();

    // passing the slice of the vector to avoid trasferring ownership
    let config = handle_argument(&env_args);

    // read file contents into single string
    let contents: String  = read_from_file(config.path);

    // search each line for query
    find_from_data(&contents, config.query);
} 

// lifetime 'args means: config cannot outlive the borrowed arguments
fn handle_argument<'args>(args: &'args [String]) -> Config<'args>{
    if args.len() < 3 {
        eprintln!("Not enough arguments!!");
    }
    // borrowing reference instead of cloning
    let path: &String = &args[1];
    let query: &String = &args[2];
    // this struct is storing references
    Config {path, query}
}

// &str is preffered over &string for read-only string borrowing 
fn read_from_file(path: &str) -> String {
    // read entire file into memory 
    fs::read_to_string(path).expect("Error reading from file!!")
}

fn find_from_data(data: &str, query: &str) {
    // .lines() creates and iterator over each line
    for line in data.lines() {
        if line.contains(query){
            println!("{line}");
        }
    }
}