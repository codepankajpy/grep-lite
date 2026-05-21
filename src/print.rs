use colored::*;

pub fn output(ans: Vec<(usize, &str)>, query: &str) {
    for (index, value) in ans {
        println!("{} : {}", index+1, highlight(value, query));
    }
}

fn highlight(line: &str, query: &str) -> String {
    line.replace(query, &query.red().bold().to_string(),)
}