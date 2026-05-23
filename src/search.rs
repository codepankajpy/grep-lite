pub fn search<'matches>(current_line: &'matches str, query: &str) ->  bool {
    
    let query = &query.to_lowercase();

    return current_line.to_lowercase().contains(query);
}
