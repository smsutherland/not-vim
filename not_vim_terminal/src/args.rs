//! Command-line arguments being passed to the program.
//!
//! [`Args::parse_args`] will parse the command-line arguments as an [`Args`] and return it.
//! TODO: If the arguments get too complex, should we swap to using clap?

use std::env;

/// The command-line arguments passed into the program.
pub struct Args {
    /// The file to be edited.
    pub file: Option<String>,
}

impl Args {
    /// Interpret the command-line arguments as an [`Args`].
    pub fn parse_args() -> Self {
        let mut args = env::args();
        args.next(); // skip program name

        Self { file: args.next() }
    }
}
