use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

pub fn export(file_path: &str, data: Vec<Request>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;
    wtr.write_record(headers())?;
    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Applicant {
    id: String,
    name: String,
    surname1: String,
    surname2: String,
    birthday: String,
    specialty: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vacancy {
    id: String,
    quantity: String,
    shift: String,
    department: String,
    specialty: String,
    profile: String,
}

#[derive(Debug, Serialize)]
pub struct Request<'a> {
    applicant: &'a Applicant,
    vacancy: &'a Vacancy,
    preference: String,
}

impl<'a> Request<'a> {
    pub fn build(row: csv::StringRecord, app: &'a [Applicant], vac: &'a [Vacancy]) -> Self {
        let app_idx = row.get(0).unwrap()[1..]
            .parse::<usize>()
            .unwrap_or_default();
        let app_row = &app[app_idx - 1];

        let vac_idx = row.get(0).unwrap()[1..]
            .parse::<usize>()
            .unwrap_or_default();
        let vac_row = &vac[vac_idx - 1];

        Self {
            applicant: app_row,
            vacancy: vac_row,
            preference: row.get(2).unwrap().to_string(),
        }
    }
}

pub fn headers<'a>() -> Vec<&'a str> {
    vec![
        "applicant",
        "name",
        "surname1",
        "surname2",
        "birthday",
        "specialty",
        "vacancy",
        "quantity",
        "shift",
        "department",
        "required_specialty",
        "profile",
        "preference",
    ]
}
