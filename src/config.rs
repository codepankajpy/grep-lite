/// Stores borrowed command line arguments
///
/// See:
/// - ../explain.md/#config-ownership-flow
/// - ../explain.md/#deref-coercion

pub struct Config<'args> {
    pub path: &'args str,
    pub query: &'args str,
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
