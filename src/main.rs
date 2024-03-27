use log::info;
use serde::{de::DeserializeOwned, Deserialize};
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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

#[allow(dead_code)]
#[derive(Debug)]
struct Vacancy {
    vacancy: String,
    quantity: u8,
    shift: String,
    department: String,
    characteristics: String,
    specialty: String,
    profile: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Board {
    applicant: String,
    vacancy: String,
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
    total_worked_time: u8,
    permanent_worked_time: u8,
    emergencies_experience: u8,
    emergencies_training: u8,
}

fn read_file<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    // read csv file
    let mut rdr = csv::Reader::from_path(file_path)?;

    // push each csv row into a vector of structs
    let mut records: Vec<T> = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

fn main() {
    // initialize logger
    env_logger::init();

    // read applicants requests
    let applicants_path: &str = "files/applicants.csv";
    let _vacancies_path: &str = "files/applicants.csv";
    let applicants: Vec<Request> = read_file(applicants_path).unwrap();
    info!("Applicants data was read successfully.");
    // let _vacancies: Vec<Vacancy> = read_file(vacancies_path).unwrap();
    // info!("Vacancies data was read successfully.");

    let mut test: Vec<Board> = Vec::new();
    for item in applicants.into_iter() {
        let row = Board {
            applicant: item.applicant,
            vacancy: String::new(),
            name: String::new(),
            surname1: String::new(),
            surname2: String::new(),
            birthday: String::new(),
            admission_date: String::new(),
            specialty: Some(String::new()),
            preference_request: false,
            forced_movility: false,
            excluded: false,
            reason: Some(0),
            total_worked_time: 0,
            permanent_worked_time: 0,
            emergencies_experience: 0,
            emergencies_training: 0,
        };
        test.push(row);
    }
}
