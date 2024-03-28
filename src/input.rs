use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Applicant {
    pub id: String,
    pub preference01: String,
    pub preference02: String,
    pub preference03: String,
    name: String,
    surname1: String,
    surname2: String,
    birthday: String,
    specialty: String,
}

#[derive(Serialize, Deserialize)]
pub struct Vacancy {
    id: String,
    quantity: String,
    shift: String,
    department: String,
    specialty: String,
    profile: String,
}

pub fn parse_file<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    // read csv file
    let mut rdr = csv::Reader::from_path(file_path)?;

    // push each csv row into a vector of structs
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record = result?;
        records.push(record);
    }
    Ok(records)
}
