use polars::{
    lazy::dsl::{col, lit, when},
    prelude::*,
};
use std::fs;

fn main() {
    let df: DataFrame = CsvReader::from_path("files/applicants.csv")
        .unwrap()
        .finish()
        .unwrap();

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
    let priorities: Vec<&str> = df
        .get_column_names()
        .iter()
        .filter(|&x| x.contains("ORDEN"))
        .cloned()
        .collect();

    // disaggregate and merges tables
    let board = df
        .clone()
        .melt(["SOLICITANTE"], priorities)
        .unwrap()
        .drop("value")
        .unwrap()
        .rename("variable", "PRIORIDAD")
        .unwrap()
        .join(
            &df.clone(),
            ["SOLICITANTE"],
            ["SOLICITANTE"],
            JoinArgs::new(JoinType::Inner),
        )
        .unwrap();

    let test = board
        .lazy()
        .with_column(
            when(col("PREFERENCIA_DIURNO").eq(lit("SI")))
                .then(lit(true))
                .otherwise(lit(false))
                .alias("PRIORIDAD"),
        )
        .collect()
        .unwrap();

    println!("{}", &test);
}

fn folder_exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

fn create_folder(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir(path)
}
