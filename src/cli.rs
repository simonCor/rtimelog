use crate::parser;
use chrono::{Local, NaiveDateTime, Timelike, Weekday, Duration};
use colored::*;
use home;
use serde_derive::{Deserialize, Serialize};
use humantime::format_duration;

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

fn print_entry(entry: (NaiveDateTime, (String, Vec<String>, Duration))) {
    let (date, (description, tags, duration)) = entry;
    print!("{} ({}) - ", date.to_string().green(), format_duration(duration.to_std().expect("TODO")).to_string().truecolor(235, 235, 52));
    if description.ends_with("**") {
        print!("{}", description.truecolor(115, 115, 115));
    } else {
        print!("{}", description);
    }

    if !tags.is_empty() {
        print!(" --");
        for tag in tags {
            print!(" {}", tag.cyan())
        }
    }
    print!("\n");
}

fn print_total_times(worktime: Duration, breaktime: Duration) {
    println!("Total work time: {}", format_duration(worktime.to_std().expect("TODO")).to_string().truecolor(235, 235, 52) );
    println!("Total break time: {}", format_duration(breaktime.to_std().expect("TODO")).to_string().truecolor(235, 235, 52) );
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

    match args.command {
        arguments::Command::Entry{ description, task } => {
            let local: NaiveDateTime = Local::now().naive_local();
            let task = match task {
                Some(number) => {
                    let tasks_parser = parser::TasksParser {
                        path: get_tasks_file_path(),
                    };
                    let tasks = tasks_parser.get_tasks();
                    match tasks.get(&number) {
                        Some(task) => task.to_string(),
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
            if !task.is_empty() {
                message = task + ": " + &description
            } else {
                message += &description
            }
            timelog_parser.append_entry_to_file(local, message);
            println!("Added entry for today")
        }

        arguments::Command::Today{filter} => {
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
            let content = timelog_parser.get_range(from, to, filter);

            //TODO: Print this prettier
            println!("Entries for today {}:", local.format("%Y-%m-%d").to_string().yellow());
            for one_entry in content.entries {
                print_entry(one_entry);
            }
            print!("\n");
            print_total_times(content.worktime, content.breaktime);
        }

        arguments::Command::Week{filter} => {
            let local: NaiveDateTime = Local::now().naive_local();
            let week = local.date().week(Weekday::Mon);

            let from = week.first_day().and_hms_opt(0, 0, 0).unwrap();
            let to = week.last_day().and_hms_opt(23, 59, 59).unwrap();
            let content = timelog_parser.get_range(from, to, filter);
            //TODO: Print this prettier
            println!(
                "Entries for this week {} to {}",
                from.format("%Y-%m-%d").to_string().yellow(),
                to.format("%Y-%m-%d").to_string().yellow()
            );
            for one_entry in content.entries {
                print_entry(one_entry);
            }
            print!("\n");
            print_total_times(content.worktime, content.breaktime);
        }

        arguments::Command::Tasks{} => {
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
}
