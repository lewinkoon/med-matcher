use serde::Serialize;
use std::error::Error;

pub fn export<T: Serialize>(file_path: &str, data: Vec<T>) -> Result<(), Box<dyn Error>> {
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
