mod helpers;

use polars::prelude::*;
use smartstring::alias::String;

fn main() {
    // read input data files
    let applicants = LazyCsvReader::new("files/applicants.csv").finish().unwrap();
    let positions = LazyCsvReader::new("files/positions.csv").finish().unwrap();

    // extract list of unique profiles
    let _profiles = positions
        .clone()
        .select([col("PERFIL")])
        .drop_nulls(None)
        .unique_stable(None, UniqueKeepStrategy::Any)
        .collect();

    // extract list of priority columns
    let preferences: Vec<String> = applicants
        .clone()
        .collect()
        .unwrap()
        .get_column_names()
        .iter()
        .filter(|&x| x.contains("ORDEN"))
        .cloned()
        .map(String::from)
        .collect();

    // disaggregate and merge into single table
    let mut board = applicants
        .clone()
        .melt(MeltArgs {
            id_vars: vec![String::from("SOLICITANTE")],
            value_vars: preferences,
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
        .sort("DESTINO", Default::default());

    // remove illegal entries
    board = board.filter(col("VALIDO").eq(lit("SI")));

    // mark entries with daylight shift privilege
    board = board.with_column(
        when(
            col("PREFERENCIA_DIURNO")
                .eq(lit("SI"))
                .and(col("TURNO").eq(lit("DIURNO"))),
        )
        .then(lit(true))
        .otherwise(lit(false))
        .alias("PRIVILEGIO"),
    );

    // create score column with profile experience and formation
    board = board.clone().with_column(
        col("ANTIGUEDAD_TOTAL")
            .cast(DataType::Float32)
            .map(
                |x| Ok(Some(x * 0.05)),
                GetOutput::from_type(DataType::Float32),
            )
            .alias("PUNTOS"),
    );

    // export dataframe to csv
    let output = std::fs::File::create("output/board.csv").unwrap();
    CsvWriter::new(output)
        .finish(&mut board.collect().unwrap())
        .unwrap();
}
