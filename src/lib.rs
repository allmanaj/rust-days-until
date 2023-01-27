use chrono::TimeZone;
use chrono::{DateTime, Utc};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;
use text_io::read;

const DATE_FORMAT: &str = "%Y-%m-%d %T";

#[derive(Debug)]
struct Event {
    name: String,
    date: DateTime<Utc>,
}

impl Event {
    fn new(name: &str, date: &str) -> Self {
        Event {
            date: match Utc::datetime_from_str(&Utc, &date, DATE_FORMAT) {
                Ok(date_time) => date_time,
                Err(error) => {
                    println!("{}", error);
                    Utc::now()
                }
            },
            name: name.into(),
        }
    }

    fn from_file(line: &str) -> Self {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        Event::new(&parts[0], &parts[1])
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

    fn complsory_arguments(&self) -> usize {
        match self {
            Command::Get => 2,
            Command::New => 3,
            Command::Invalid => 0,
        }
    }
}

const FILENAME: &str = "days_until_dates.toml";

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];

    // load event from file
    match Command::from_string(command) {
        Command::New => create_new(&args, Command::New),
        Command::Get => get(&args),
        Command::Invalid => invalid(),
    }
}

fn create_new(args: &Vec<String>, command: Command) {
    if args.len() > command.complsory_arguments() {
        println!("Missing necessary arguments");
        exit(0);
    }

    let name: String = match prompt_for_name() {
        Ok(name) => name,
        Err(_) => panic!("Enter a name!"),
    };

    if load_events().iter().any(|event| event.name == name) {
        println!("An event already exists with that name!");
        exit(0);
    }

    let date: String = match prompt_for_date() {
        Ok(date) => date,
        Err(_) => panic!("Enter a date!"),
    };

    let time: String = match prompt_for_time() {
        Ok(time) => time,
        Err(_) => panic!("Enter a date!"),
    };

    let mut file = fs::OpenOptions::new().append(true).open(FILENAME).unwrap();

    if let Err(_e) = writeln!(file, "{} = {} {}", name, date, time) {
        panic!("Could not write to file");
    }
}

fn get(args: &Vec<String>) {
    if args.len() < 3 {
        panic!("You must supply an event name");
    }

    match load_events().iter().find(|event| event.name == args[2]) {
        Some(event) => event_output(event),
        None => {
            println!("No event found with that name.");
            exit(0);
        }
    }
}

fn event_output(event: &Event) {
    println!(
        "There are {} days until {}",
        days_between(event.date, Utc::now()),
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

fn load_events() -> Vec<Event> {
    match fs::read_to_string(FILENAME) {
        Ok(data) => data,
        Err(_error) => create_file_and_return_content(),
    }
    .split("\n")
    .filter(|line| !line.is_empty())
    .map(|line| Event::from_file(line))
    .collect::<Vec<Event>>()
}

fn invalid() {
    println!("invalid!");
}

fn prompt_for_name() -> io::Result<String> {
    println!("What's your event called?");
    let line = read!("{}\n");
    Ok(line)
}

fn prompt_for_date() -> io::Result<String> {
    println!("What date is it happening? e.g 2023-10-04");
    let line = read!("{}\n");
    Ok(line)
}

fn prompt_for_time() -> io::Result<String> {
    println!("What time is it happening? e.g 14:30:00");
    let line = read!("{}\n");
    Ok(line)
}
