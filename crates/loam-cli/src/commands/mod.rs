use std::str::FromStr;

use clap::{command, CommandFactory, FromArgMatches, Parser};

pub mod build;



const ABOUT: &str = "Build contracts";

// long_about is shown when someone uses `--help`; short help when using `-h`
const LONG_ABOUT: &str = "LONG ABOUT";

#[derive(Parser, Debug)]
#[command(
    name = "loam",
    about = ABOUT,
    long_about = ABOUT.to_string() + LONG_ABOUT,
    disable_help_subcommand = true,
)]
pub struct Root {
    // #[clap(flatten)]
    // pub global_args: global::Args,

    #[command(subcommand)]
    pub cmd: Cmd,
}

impl Root {
    pub fn new() -> Result<Self, clap::Error> {
        let mut matches = Self::command().get_matches();
        Self::from_arg_matches_mut(&mut matches)
    }

    pub fn from_arg_matches<I, T>(itr: I) -> Result<Self, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Self::from_arg_matches_mut(&mut Self::command().get_matches_from(itr))
    }
    pub fn run(&mut self) -> Result<(), Error> {
        match &mut self.cmd {
            Cmd::Build(build_info) => build_info.run()?,
        };
        Ok(())
    }
}

impl FromStr for Root {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_arg_matches(s.split_whitespace())
    }
}

#[derive(Parser, Debug)]
pub enum Cmd {
    /// Build contracts
    Build(build::Cmd),
    
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // TODO: stop using Debug for displaying errors
    #[error(transparent)]
    Build(#[from] build::Error),
}
