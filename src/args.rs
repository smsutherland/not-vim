use std::env;

pub struct Args {
    pub file: String,
}

impl Args {
    pub fn parse_args() -> Self {
        let mut args = env::args();
        args.next(); // skip program name

        Self {
            file: args.next().expect("Expected to be passed a file name"),
        }
    }
}
