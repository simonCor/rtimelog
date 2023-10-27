use std::fs::{OpenOptions, read_to_string};
use std::io::prelude::*;
use chrono::{NaiveDateTime};
use std::collections::BTreeMap;

pub fn append_entry_to_file(date: NaiveDateTime, message: String) {
    let mut output = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("./testfile")
        .unwrap();

        if let Err(e) = writeln!(output, "{}: {}", date.format("%Y-%m-%d %H:%M"), message) {
            eprintln!("Couldn't write to file: {}", e);
        }
}

pub fn get_range(from: NaiveDateTime, to: NaiveDateTime) -> BTreeMap<NaiveDateTime, String>{
    let mut result: BTreeMap<NaiveDateTime, String> = BTreeMap::new();

    for line in read_to_string("./testfile").unwrap().lines() {
        let mut  splitted_line = line.splitn(3, ':');

        let mut date = String::new();
        date.push_str(splitted_line.next().expect("No content!"));
        date.push(':');
        date.push_str(splitted_line.next().expect("No content!"));
        let description: String = splitted_line.next().expect("No content!").to_string();

        let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M").expect("Parsing failed!");

        // TODO: Split into date and description part
        if (date >= from) && (date <= to)
        {
            result.insert(date, description);
        }
    }
    
    // TODO: Only return data from date
    result
}