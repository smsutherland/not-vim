//! Command-line arguments being passed to the program.
//!
//! [`Args::parse_args`] will parse the command-line arguments as an [`Args`] and return it.
//! TODO: If the arguments get too complex, should we swap to using clap?

use anyhow::bail;
use std::env;

/// The command-line arguments passed into the program.
pub struct Args {
    /// The file to be edited.
    pub file: String,
}

impl Args {
    /// Interpret the command-line arguments as an [`Args`].
    pub fn parse_args() -> anyhow::Result<Self> {
        let mut args = env::args();
        args.next(); // skip program name

        Ok(Self {
            file: match args.next() {
                Some(file) => file,
                None => bail!("Expected to be passed a file name"),
            },
        })
    }
}
