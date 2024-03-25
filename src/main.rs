mod helpers;
extern crate csv;

use csv::Reader;
use std::fs::File;

fn main() -> Result<(), csv::Error> {
    let mut rdr: Reader<File> = csv::Reader::from_path("./files/applicants.csv")?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
