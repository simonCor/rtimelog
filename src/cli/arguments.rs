use clap::Parser;

//TODO: Rewrite or understand the used syntax

/// time logging program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show today
    #[arg(short, long, default_value_t = false)]
    pub today: bool,

    /// Show the current week
    #[arg(short, long, default_value_t = false)]
    pub week: bool,

    /// Add an entry
    #[arg(short, long, default_value_t = String::new())]
    pub new_entry: String,

    #[arg(short('k'), long, required(false))]
    pub with_task: Option<i32>,

    /// Show available tasks
    #[arg(long, default_value_t = false)]
    pub tasks: bool,
}

pub fn parse_args() -> Args
{
    Args::parse()
}