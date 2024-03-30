// mod helpers;
mod algorithm;
mod board;
mod helpers;
mod input;

use log::error;
use log::info;
use std::process;

fn main() {
    // initialize logger
    env_logger::init();

    // define file paths
    let applicants_path: &str = "files/applicants.csv";
    let vacancies_path: &str = "files/vacancies.csv";
    let board_path: &str = "results/board.csv";

    // read applicants requests file
    let applicants: Vec<input::Applicant> = match helpers::parse_file(applicants_path) {
        Ok(res) => {
            info!("Loaded {} applicant requests.", res.len());
            res
        }
        Err(e) => {
            error!("{e}");
            process::exit(1)
        }
    };

    // read vacant positions file
    let _vacancies: Vec<input::Vacancy> = match helpers::parse_file(vacancies_path) {
        Ok(res) => {
            info!("Loaded {} vacancy positions.", res.len());
            res
        }
        Err(e) => {
            error!("{e}");
            process::exit(1)
        }
    };

    // build requests board
    let requests = board::Request::build(applicants);
    info!("Loaded {} board requests.", requests.len());

    // write requests board to file
    match helpers::export(board_path, &requests) {
        Ok(_) => info!("Requests board successfully written."),
        Err(e) => {
            error!("{e}");
            process::exit(1)
        }
    };

    let proposors = algorithm::Proposor::build(requests);
    info!("Created list of {} proposors", proposors.len());
}
