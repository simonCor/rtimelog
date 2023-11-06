use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct App {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub command: Command,
}


#[derive(Debug, Subcommand)]
pub enum Command {
    /// Make a new entry
    Entry {
        /// The entry description
        description: String,
        /// An example option
        #[clap(long, short = 't')]
        task: Option<i32>,
    },
    /// Show entries for today
    Today {

    },
    /// Show entries for current week
    Week {

    },
    /// Show available tasks
    Tasks {

    },
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    //... other global options
}

pub fn parse_args() -> App
{
    App::parse()
}