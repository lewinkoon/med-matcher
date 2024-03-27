use log::{error, info};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
struct Request {
    applicant: String,
    name: String,
    surname1: String,
    surname2: String,
    birthday: String,
    admission_date: String,
    specialty: Option<String>,
    preference_request: bool,
    forced_movility: bool,
    excluded: bool,
    reason: Option<u8>,
    preference01: String,
    preference02: String,
    preference03: String,
    total_worked_time: u8,
    permanent_worked_time: u8,
    emergencies_experience: u8,
    emergencies_training: u8,
}

fn read_file(file_path: &str) -> Result<Vec<Request>, Box<dyn Error>> {
    // read csv file
    let mut rdr = csv::Reader::from_path(file_path)?;

    // push each csv row into a vector of structs
    let mut records: Vec<Request> = Vec::new();
    for result in rdr.deserialize() {
        let record: Request = result?;
        records.push(record);
    }

    // sort records by name
    records.sort_by(|a, b| b.name.cmp(&a.name));

    Ok(records)
}

fn main() {
    // initialize logger
    env_logger::init();

    let path = "files/applicants.csv";

    match read_file(path) {
        Ok(res) => info!("Script finished succesfully: {:?}", res[0]),
        Err(error) => error!("Script crashed: {}", error),
    };
}
