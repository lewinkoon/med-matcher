use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;

pub fn export<T: Serialize>(file_path: &str, data: &Vec<T>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    // wtr.write_record(headers())?;
    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
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
