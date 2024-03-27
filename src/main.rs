use log::info;
use medmatch::Request;
use std::process;

fn main() {
    // initialize logger
    env_logger::init();

    // read applicants requests
    let applicants_path: &str = "files/applicants.csv";
    let applicants = Request::build(applicants_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    info!("Loaded {} applicant requests.", applicants.len());

    // read vacant positions
    // let vacancies_path: &str = "files/applicants.csv";
    // let vacancies: Vec<Vacancy> = read_file(vacancies_path).unwrap();
    // info!("Vacancies data was read successfully.");

    // let mut test: Vec<Board> = Vec::new();
    // for item in applicants.into_iter() {
    //     let row = Board {
    //         applicant: item.applicant,
    //         vacancy: String::new(),
    //         name: String::new(),
    //         surname1: String::new(),
    //         surname2: String::new(),
    //         birthday: String::new(),
    //         admission_date: String::new(),
    //         specialty: Some(String::new()),
    //         preference_request: false,
    //         forced_movility: false,
    //         excluded: false,
    //         reason: Some(0),
    //         total_worked_time: 0,
    //         permanent_worked_time: 0,
    //         emergencies_experience: 0,
    //         emergencies_training: 0,
    //     };
    //     test.push(row);
    // }
}
