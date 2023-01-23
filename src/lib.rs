use chrono::offset::{FixedOffset, Local, TimeZone};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::exit;
use toml::Value;

const DATE_FORMAT: &str = "%Y-%m-%d %T";

#[derive(Debug)]
struct Event {
    name: String,
    date: DateTime<Utc>,
}

impl Event {
    fn new(name: String, date: String) -> Self {
        Event {
            date: match Utc::datetime_from_str(&Utc, &date, DATE_FORMAT) {
                Ok(date_time) => date_time,
                Err(error) => {
                    println!("{}", error);
                    Utc::now()
                }
            },
            name: name,
        }
    }

    fn from_file(line: &str) -> Self {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        Event::new("test".to_owned(), "2023-01-20 01:34:59".to_owned())
    }
}

enum Command {
    New,
    Get,
    Invalid,
}

impl Command {
    fn from_string(key: &str) -> Command {
        match key {
            "get" => Command::Get,
            "new" => Command::New,
            &_ => Command::Invalid,
        }
    }
}

const FILENAME: &str = "days_until_dates.toml";

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];

    // load event from file
    match Command::from_string(command) {
        Command::New => create_new(&args),
        Command::Get => get(&args),
        Command::Invalid => invalid(),
    }
    // get days until event from today

    //return
}

fn create_new(args: &Vec<String>) {
    // let data_file_result = fs::File::open(filename);

    // let dates_file: File = match data_file_result {
    //     Ok(file) => file,
    //     Err(_error) => panic!("Couldn't open the file!"),
    // };

    // // get correct line by key
    // io::BufReader::new()
    // return diff in date between date and now.
}

fn get(args: &Vec<String>) {
    if args.len() < 3 {
        panic!("You must supply an event name");
    }

    let file_data: Vec<Event> = match fs::read_to_string(FILENAME) {
        Ok(data) => data,
        Err(_error) => create_file_and_return_content(),
    }
    .split("\n")
    .map(|line| Event::from_file(line))
    .collect::<Vec<Event>>();

    for event in file_data {
        if event.name == args[2] {
            event_output(event)
        }
    }
}

fn event_output(event: Event) {
    println!(
        "There are {} days until {}",
        days_between(Utc::now(), event.date),
        event.name
    )
}

fn days_between(date1: DateTime<Utc>, date2: DateTime<Utc>) -> i64 {
    date1.signed_duration_since(date2).num_days()
}

fn create_file_and_return_content() -> String {
    match File::create(FILENAME) {
        Ok(_file) => String::new(),
        Err(_error) => panic!("could not find or create file"),
    }
}

fn invalid() {
    println!("invalid!");
}
