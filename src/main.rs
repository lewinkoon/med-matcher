use polars::{lazy::dsl::col, prelude::*};
use smartstring::alias::String;
use std::fs;

fn main() {
    let applicants = LazyCsvReader::new("files/applicants.csv")
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let _positions = LazyCsvReader::new("files/positions.csv").finish().unwrap();

    // check if the output folder exists
    let output_path = "output";
    if !folder_exists(&output_path) {
        // if it does not exist then create it
        if let Err(err) = create_folder(&output_path) {
            eprintln!("Error creating output folder: {}", err);
        } else {
            println!("Output folder created succesfully!");
        }
    }

    // get priority columns as a list
    let _priorities: Vec<String> = applicants
        .get_column_names()
        .iter()
        .filter(|&x| x.contains("ORDEN"))
        .cloned()
        .map(|s| String::from(s))
        .collect();

    // disaggregate and merges tables
    let mut board = applicants
        .clone()
        .lazy()
        .melt(MeltArgs {
            id_vars: vec![String::from("SOLICITANTE")],
            value_vars: _priorities,
            variable_name: Some(String::from("PRIORIDAD")),
            value_name: Some(String::from("POSICION_SOLICITADA")),
            streamable: false,
        })
        .join(
            applicants.clone().lazy(),
            [col("SOLICITANTE")],
            [col("SOLICITANTE")],
            JoinArgs::new(JoinType::Inner),
        )
        .collect()
        .unwrap();

    // mark entries without daytime work shift
    // let shift = board
    //     .lazy()
    //     .with_column(
    //         when(col("PREFERENCIA_DIURNO").eq(lit("SI")))
    //             .then(lit(true))
    //             .otherwise(lit(false))
    //             .alias("PRIORIDAD"),
    //     )
    //     .collect()
    //     .unwrap();

    let file = std::fs::File::create("output/data.csv").unwrap();

    CsvWriter::new(file).finish(&mut board).unwrap();
}

fn folder_exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

fn create_folder(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir(path)
}
