use csv::StringRecord;
use std::error::Error;

#[allow(dead_code)]
pub struct Applicant {
    applicant: String,
    name: String,
    surname1: String,
    surname2: String,
    birthday: String,
    specialty: String,
}

#[allow(dead_code)]
pub struct Vacancy {
    vacancy: String,
    quantity: u8,
    shift: String,
    department: String,
    specialty: String,
    profile: String,
}

pub trait Record {
    fn new(row: StringRecord) -> Self;
}

impl Record for Applicant {
    fn new(row: StringRecord) -> Self {
        Self {
            applicant: row.get(0).unwrap().to_string(),
            name: row.get(1).unwrap().to_string(),
            surname1: row.get(2).unwrap().to_string(),
            surname2: row.get(3).unwrap().to_string(),
            birthday: row.get(4).unwrap().to_string(),
            specialty: row.get(6).unwrap().to_string(),
        }
    }
}

impl Record for Vacancy {
    fn new(row: StringRecord) -> Self {
        Self {
            vacancy: row.get(0).unwrap().to_string(),
            quantity: row.get(1).unwrap().parse().unwrap(),
            shift: row.get(2).unwrap().to_string(),
            department: row.get(3).unwrap().to_string(),
            specialty: row.get(4).unwrap().to_string(),
            profile: row.get(5).unwrap().to_string(),
        }
    }
}

pub fn parse_file<T: Record>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    // read csv file
    let mut rdr = csv::Reader::from_path(file_path)?;

    // push each csv row into a vector of structs
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = Record::new(result?);
        records.push(record);
    }
    Ok(records)
}

// pub fn parse_file<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
//     // read csv file
//     let mut rdr = csv::Reader::from_path(file_path)?;

//     // push each csv row into a vector of structs
//     let mut records: Vec<T> = Vec::new();
//     for result in rdr.deserialize() {
//         let record = result?;
//         records.push(record);
//     }
//     Ok(records)
// }
