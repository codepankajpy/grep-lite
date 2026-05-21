// use clap::Parser;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
pub struct Config<'args>{

    // #[arg(required = true)]
    pub query: &'args str,
    
    // #[arg(required = true)]
    pub path: &'args str,
}

impl<'args> Config<'args> {
    pub fn build(args: &'args [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }
        let path: &str = &args[1];
        let query: &str = &args[2];

        Ok(Self {path, query})
    }
}
