use chrono::NaiveDateTime;
use std::collections::BTreeMap;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

pub struct TimelogParser {
    pub path: PathBuf,
}

impl TimelogParser {
    pub fn append_entry_to_file(&self, date: NaiveDateTime, message: String) {
        let path = &self.path;

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

    pub fn get_range(
        &self,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> BTreeMap<NaiveDateTime, (String, Vec<String>)> {
        let mut result: BTreeMap<NaiveDateTime, (String, Vec<String>)> = BTreeMap::new();
        let path = &self.path;

        for line in read_to_string(path).unwrap().lines() {
            if !line.starts_with("#") {
                let mut splitted_line = line.splitn(3, ':');

                let mut date = String::new();
                date.push_str(splitted_line.next().expect("No content!"));
                date.push(':');
                date.push_str(splitted_line.next().expect("No content!"));
                let description_with_tags: String =
                    splitted_line.next().expect("No content!").to_string();
                let mut splitted_description_tags = description_with_tags.split("--");
                let description: String = splitted_description_tags
                    .next()
                    .expect("No description")
                    .trim()
                    .to_string();
                let tags = match splitted_description_tags.next() {
                    Some(tag_strings) => {
                        let mut tags: Vec<String> = Vec::new();
                        for tag in tag_strings.split(" ") {
                            let trimmed_tag = tag.trim().to_string();
                            if !trimmed_tag.is_empty() {
                                tags.push(trimmed_tag.to_string());
                            }
                        }
                        tags
                    }
                    None => {
                        let tags: Vec<String> = Vec::new();
                        tags
                    }
                };

                let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M")
                    .expect("Parsing failed!");

                if (date >= from) && (date <= to) {
                    result.insert(date, (description, tags));
                }
            }
        }

        result
    }
}

pub struct TasksParser {
    pub path: PathBuf,
}

impl TasksParser {
    pub fn get_tasks(&self) -> BTreeMap<i32, String> {
        let path = &self.path;

        let mut tasks: BTreeMap<i32, String> = BTreeMap::new();
        let mut i = 0;
        for line in read_to_string(path).unwrap().lines() {
            if !line.starts_with("#") {
                tasks.insert(i, line.to_string());
                i += 1;
            }
        }

        tasks
    }
}
