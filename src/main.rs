use log::error;
use log::info;
use medmatch::*;
use std::process;

fn main() {
    // initialize logger
    env_logger::init();

    // read applicants requests
    let applicants_path: &str = "files/applicants.csv";
    let applicants: Vec<Applicant> = parse_file(applicants_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    info!("Loaded {} applicant requests.", applicants.len());

    // read vacant positions
    let vacancies_path: &str = "files/vacancies.csv";
    let vacancies: Vec<Vacancy> = parse_file(vacancies_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    info!("Loaded {} vacancy positions.", vacancies.len());

    // build requests board
    let preferences_path: &str = "files/preferences.csv";
    let mut rdr = csv::Reader::from_path(preferences_path).unwrap();
    let mut requests: Vec<Request> = Vec::new();
    for row in rdr.records() {
        let record: Request = Request::new(row.unwrap(), applicants.as_ref(), vacancies.as_ref());
        requests.push(record);
    }
    info!("Loaded {} requests.", requests.len());

    // write requests board to file
    let board_path = "results/board.csv";
    match export(board_path, requests) {
        Ok(_) => info!("Requests board successfully written."),
        Err(err) => error!("{}", err),
    }
}
