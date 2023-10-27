use std::fs::{OpenOptions, read_to_string};
use std::io::prelude::*;
use chrono::{NaiveDateTime};
use std::collections::BTreeMap;
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

pub fn append_entry_to_file(date: NaiveDateTime, message: String) {
    let path = get_timelog_file_path();

    let mut output = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

        if let Err(e) = writeln!(output, "{}: {}", date.format("%Y-%m-%d %H:%M"), message) {
            eprintln!("Couldn't write to file: {}", e);
        }
}

pub fn get_range(from: NaiveDateTime, to: NaiveDateTime) -> BTreeMap<NaiveDateTime, String>{
    let mut result: BTreeMap<NaiveDateTime, String> = BTreeMap::new();
    let path  = get_timelog_file_path();

    for line in read_to_string(path).unwrap().lines() {
        let mut  splitted_line = line.splitn(3, ':');

        let mut date = String::new();
        date.push_str(splitted_line.next().expect("No content!"));
        date.push(':');
        date.push_str(splitted_line.next().expect("No content!"));
        let description: String = splitted_line.next().expect("No content!").to_string();

        let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M").expect("Parsing failed!");

        if (date >= from) && (date <= to)
        {
            result.insert(date, description);
        }
    }

    result
}