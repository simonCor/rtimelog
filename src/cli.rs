use clap::Parser;
use chrono::{Local, NaiveDateTime, Timelike, Weekday};
use colored::*;
use crate::parser;
use serde_derive::{Serialize, Deserialize};
use home;

fn get_timelog_file_path() -> std::path::PathBuf
{
    match home::home_dir() {
        Some(mut path) => {
            path.push(".gtimelog/timelog.txt");
            path
        },
        None => {
            panic!("Impossible to get your home dir!");
        },
    }
}

#[derive(Serialize, Deserialize)]
struct CliConfig {
    timelog_file: std::path::PathBuf,
}

impl ::std::default::Default for CliConfig {
    fn default() -> Self { Self { timelog_file: get_timelog_file_path() } }
}

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

fn print_entry(entry: (NaiveDateTime, (String, Vec<String>)))
{
    print!("{} - ", entry.0.to_string().green());
    print!("{} -- ", entry.1.0);
    for tag in entry.1.1 {
        print!("{} ", tag.cyan())
    }
    print!("\n");
}

pub fn cli()
{
    let args = Args::parse();

    let config: CliConfig = match confy::load("rtimelog", None) {
        Ok(config) => {config},
        Err(_) => {panic!("config error");},
    };

    let parser = parser::Parser{path: get_timelog_file_path()}; //TODO: from config

    if args.new_entry.len() != 0
    {
        let local: NaiveDateTime = Local::now().naive_local();
        parser.append_entry_to_file(local, args.new_entry);
        println!("Added entry for today")
    }

    if args.today
    {
        let local: NaiveDateTime = Local::now().naive_local();
        let from = local.with_hour(0).expect("arggghh1").with_minute(0).expect("dfsdfsd");
        let to = local.with_hour(23).expect("arggghh2").with_minute(59).expect("sdfds");
        let content = parser.get_range(from, to);

        //TODO: Print this prettier
        println!("Entries for today {}", local.format("%Y-%m-%d").to_string());
        for one_entry in content
        {
            print_entry(one_entry);
        }
    }

    if args.week
    {
        let local: NaiveDateTime = Local::now().naive_local();
        let week = local.date().week(Weekday::Mon);

        let from = week.first_day().and_hms_opt(0, 0, 0).unwrap();
        let to = week.last_day().and_hms_opt(23, 59, 59).unwrap();
        let content = parser.get_range(from, to);
        //TODO: Print this prettier
        println!("Entries for this week {} to {}", from.format("%Y-%m-%d").to_string().yellow(), to.format("%Y-%m-%d").to_string().yellow());
        for one_entry in content
        {
            print_entry(one_entry);
        }
    }
}