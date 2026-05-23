use grep_lite::read::read;
use grep_lite::config::Config;
use std::io::Error;
use clap::Parser;

fn main() -> Result<(), Error>{

    let args = Config::parse();

    read(&args.path, &args.query)?;

    Ok(())

}
