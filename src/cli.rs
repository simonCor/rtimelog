use clap::Parser;
use chrono::{Local, NaiveDateTime, Timelike, Weekday};
use crate::parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show today
    #[arg(short, long, default_value_t = false)]
    today: bool,

    /// Show the current week
    #[arg(short, long, default_value_t = false)]
    week: bool,

    /// Add an entry
    #[arg(short, long, default_value_t = String::new())]
    new_entry: String,
}

pub fn cli()
{
    let args = Args::parse();

    if args.new_entry.len() != 0
    {
        let local: NaiveDateTime = Local::now().naive_local();
        parser::append_entry_to_file(local, args.new_entry);
        println!("Added entry for today")
    }

    if args.today
    {
        let local: NaiveDateTime = Local::now().naive_local();
        let from = local.with_hour(0).expect("arggghh1").with_minute(0).expect("dfsdfsd");
        let to = local.with_hour(23).expect("arggghh2").with_minute(59).expect("sdfds");
        let content = parser::get_range(from, to);
        
        //TODO: Print this prettier
        println!("Entries for today {}", local.format("%Y-%m-%d").to_string());
        for one_entry in content
        {
            println!("{} - {}", one_entry.0, one_entry.1)
        }
    }

    if args.week
    {
        let local: NaiveDateTime = Local::now().naive_local();
        let week = local.date().week(Weekday::Mon);

        let from = week.first_day().and_hms_opt(0, 0, 0).unwrap();
        let to = week.last_day().and_hms_opt(23, 59, 59).unwrap();
        let content = parser::get_range(from, to);
        //TODO: Print this prettier
        println!("Entries for this week {} to {}", from.format("%Y-%m-%d").to_string(), to.format("%Y-%m-%d").to_string());
        for one_entry in content
        {
            println!("{} - {}", one_entry.0, one_entry.1)
        }
    }
}