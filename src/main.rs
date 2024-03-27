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

    // let mut test: Vec<Request> = Vec::new();
    // for item in applicants.into_iter() {
    //     let row = Request::build(item.applicant);
    //     test.push(row);
    // }
}
