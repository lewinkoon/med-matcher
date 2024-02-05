use polars::prelude::*;
use smartstring::alias::String;
use std::fs;

fn main() {
    let applicants = LazyCsvReader::new("files/applicants.csv").finish().unwrap();

    let positions = LazyCsvReader::new("files/positions.csv").finish().unwrap();

    let _profiles = positions
        .clone()
        .select([col("PERFIL")])
        .drop_nulls(None)
        .unique(None, UniqueKeepStrategy::Any)
        .collect();

    println!("{:?}", _profiles);

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
    let priorities: Vec<String> = applicants
        .clone()
        .collect()
        .unwrap()
        .get_column_names()
        .iter()
        .filter(|&x| x.contains("ORDEN"))
        .cloned()
        .map(|s| String::from(s))
        .collect();

    // disaggregate and merge into single table
    let mut board = applicants
        .clone()
        .melt(MeltArgs {
            id_vars: vec![String::from("SOLICITANTE")],
            value_vars: priorities,
            variable_name: Some(String::from("PRIORIDAD")),
            value_name: Some(String::from("DESTINO")),
            streamable: false,
        })
        .join(
            applicants.clone(),
            [col("SOLICITANTE")],
            [col("SOLICITANTE")],
            JoinArgs::new(JoinType::Inner),
        )
        .join(
            positions.clone(),
            [col("DESTINO")],
            [col("DESTINO")],
            JoinArgs::new(JoinType::Inner),
        )
        .with_column(
            when(col("PREFERENCIA_DIURNO").eq(lit("SI")))
                .then(lit(true))
                .otherwise(lit(false))
                .alias("PREFERENCIA_DIURNO"),
        ) // mark entries without daytime work shift
        .filter(col("VALIDO").eq(lit("SI")))
        .with_column(
            when(col("PERFIL").eq(lit("SI")))
                .then(lit(true))
                .otherwise(lit(false))
                .alias("PREFERENCIA_DIURNO"),
        )
        .collect()
        .unwrap();



    let output = std::fs::File::create("output/data.csv").unwrap();

    CsvWriter::new(output).finish(&mut board).unwrap();
}

fn folder_exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

fn create_folder(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir(path)
}
