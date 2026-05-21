use colored::*;
use regex::Regex;

pub fn output(ans: Vec<(usize, &str)>, query: &str) {

    for (index, value) in ans {
        println!("{} : {}", index+1, highlight(value, query));
    }
}

fn highlight(line: &str, query: &str) -> String {
    let regex = Regex::new(&format!("(?i){}", regex::escape(query))).unwrap();

    regex
        .replace_all(line, |caps: &regex::Captures| {
            caps[0].red().bold().to_string()
        })
        .to_string()
}