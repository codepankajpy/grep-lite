use colored::*;

pub fn output(ans: Vec<(usize, &str)>, query: &str) {

    for (index, value) in ans {
        println!("{} : {}", index+1, highlight(value, query.to_lowercase().as_str()));
    }
}

fn highlight(line: &str, query: &str) -> String {
    line.to_lowercase().replace(query, &query.red().bold().to_string(),)
}
