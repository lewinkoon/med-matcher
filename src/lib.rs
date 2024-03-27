use ::serde::Deserialize;
use std::error::Error;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Request {
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

impl Request {
    pub fn build(file_path: &str) -> Result<Vec<Request>, Box<dyn Error>> {
        // read csv file
        let mut rdr = csv::Reader::from_path(file_path)?;

        // push each csv row into a vector of structs
        let mut records: Vec<Request> = Vec::new();
        for result in rdr.deserialize() {
            let record: Request = result?;
            records.push(record);
        }
        Ok(records)
    }
}

// #[derive(Debug, Deserialize)]
// struct Vacancy {
//     vacancy: String,
//     quantity: u8,
//     shift: String,
//     department: String,
//     characteristics: String,
//     specialty: String,
//     profile: String,
// }

// struct Board {
//     applicant: String,
//     vacancy: String,
//     name: String,
//     surname1: String,
//     surname2: String,
//     birthday: String,
//     admission_date: String,
//     specialty: Option<String>,
//     preference_request: bool,
//     forced_movility: bool,
//     excluded: bool,
//     reason: Option<u8>,
//     total_worked_time: u8,
//     permanent_worked_time: u8,
//     emergencies_experience: u8,
//     emergencies_training: u8,
// }
