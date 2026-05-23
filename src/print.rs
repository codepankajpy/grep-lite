use colored::*;
use regex::Regex;

pub fn output(index: usize, current_line: &str, query: &str){
    println!("{} : {}", index+1, highlight(current_line, query));
}

// wrote with the help of chatgpt - learn about replace_all more
fn highlight(line: &str, query: &str) -> String {
    let regex = Regex::new(&format!("(?i){}", regex::escape(query))).unwrap();

    regex
        .replace_all(line, |caps: &regex::Captures| {
            caps[0].red().bold().to_string()
        })
        .to_string()
}
