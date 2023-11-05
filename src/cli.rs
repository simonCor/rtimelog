use crate::parser;
use chrono::{Local, NaiveDateTime, Timelike, Weekday};
use colored::*;
use home;
use serde_derive::{Deserialize, Serialize};

mod arguments;

fn get_timelog_file_path() -> std::path::PathBuf {
    match home::home_dir() {
        Some(mut path) => {
            path.push(".gtimelog/timelog.txt");
            path
        }
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

fn get_tasks_file_path() -> std::path::PathBuf {
    match home::home_dir() {
        Some(mut path) => {
            path.push(".gtimelog/tasks.txt");
            path
        }
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CliConfig {
    timelog_file: std::path::PathBuf,
    tasks_file: std::path::PathBuf,
}

impl ::std::default::Default for CliConfig {
    fn default() -> Self {
        Self {
            timelog_file: get_timelog_file_path(),
            tasks_file: get_tasks_file_path(),
        }
    }
}

fn print_entry(entry: (NaiveDateTime, (String, Vec<String>))) {
    print!("{} - ", entry.0.to_string().green());
    if entry.1.0.ends_with("**") | entry.1.0.ends_with("***")
    {
        print!("{}", entry.1.0.truecolor(115, 115, 115));
    } else {
        print!("{}", entry.1.0);
    }

    if !entry.1.1.is_empty()
    {
        print!(" --");
        for tag in entry.1.1 {
            print!(" {}", tag.cyan())
        }
    }
    print!("\n");
}

pub fn cli() {
    let args = arguments::parse_args();

/*     let config: CliConfig = match confy::load("rtimelog", None) {
        Ok(config) => config,
        Err(_) => {
            panic!("config error");
        }
    }; */

    let timelog_parser = parser::TimelogParser {
        path: get_timelog_file_path(),
    }; //TODO: from config

    if args.new_entry.len() != 0 {
        let local: NaiveDateTime = Local::now().naive_local();
        let task = match args.with_task {
            Some(number) => {
                let tasks_parser = parser::TasksParser {
                    path: get_tasks_file_path(),
                };
                let tasks = tasks_parser.get_tasks();
                match tasks.get(&number) {
                    Some(task) => {
                        task.to_string()
                    }
                    None => {
                        //TODO: Maybe a panic is not the right thing here
                        panic!("The given task number does not exist");
                    }

                }
            }
            None => {
                // Do nothing that is ok.
                String::new()
            }
        };
        let mut message: String = String::new();
        if !task.is_empty()
        {
            message = task + ": "

        } else {
            message += &args.new_entry
        }
        timelog_parser.append_entry_to_file(local, message);
        println!("Added entry for today")
    }

    if args.today {
        let local: NaiveDateTime = Local::now().naive_local();
        let from = local
            .with_hour(0)
            .expect("arggghh1")
            .with_minute(0)
            .expect("dfsdfsd");
        let to = local
            .with_hour(23)
            .expect("arggghh2")
            .with_minute(59)
            .expect("sdfds");
        let content = timelog_parser.get_range(from, to);

        //TODO: Print this prettier
        println!("Entries for today {}", local.format("%Y-%m-%d").to_string());
        for one_entry in content {
            print_entry(one_entry);
        }
    }

    if args.week {
        let local: NaiveDateTime = Local::now().naive_local();
        let week = local.date().week(Weekday::Mon);

        let from = week.first_day().and_hms_opt(0, 0, 0).unwrap();
        let to = week.last_day().and_hms_opt(23, 59, 59).unwrap();
        let content = timelog_parser.get_range(from, to);
        //TODO: Print this prettier
        println!(
            "Entries for this week {} to {}",
            from.format("%Y-%m-%d").to_string().yellow(),
            to.format("%Y-%m-%d").to_string().yellow()
        );
        for one_entry in content {
            print_entry(one_entry);
        }
    }

    if args.tasks {
        let tasks_parser = parser::TasksParser {
            path: get_tasks_file_path(),
        };
        let tasks = tasks_parser.get_tasks();

        println!("Available tasks:");
        for (i, one_entry) in tasks {

            println!("{}: {}", i, one_entry);
        }
    }
}
