
pub fn search<'matches>(data: &'matches str, query: &str) ->  Vec<(usize, &'matches str)>{
    // .lines() creates and iterator over each line
    data.lines().enumerate().filter(|line| line.1.contains(query)).collect()
}