use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config{

    #[arg(required = true)]
    pub query: String,
    
    #[arg(required = true)]
    pub path: String,
}

