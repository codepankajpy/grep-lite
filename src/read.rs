use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::print::output;
use crate::search::search;

use std::io::Error;

pub fn read(path: &str, query: &str) -> Result<(), Error>{
    let given_file = File::open(path)?;
    let reader = BufReader::new(given_file);

    for (index, line) in reader.lines().enumerate() {

        let current_line = line?;

        let search_line = search(&current_line, query); 
        
        if search_line {
            output(index, &current_line, query);
        }
    }
    Ok(())
}
