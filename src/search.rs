
pub fn search<'matches>(data: &'matches str, query: &str) ->  Vec<(usize, &'matches str)>{
    // .lines() creates and iterator over each line
    let query = query.to_lowercase();
    data.lines().enumerate().filter(|(_, line)| {
        line.to_lowercase().contains(&query)
    }).collect()
}