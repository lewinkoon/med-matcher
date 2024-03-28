use crate::input::Applicant;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Request {
    applicant: String,
    vacancy: String,
    preference: u8,
}

impl Request {
    pub fn build(app: Vec<Applicant>) -> Vec<Self> {
        let mut board: Vec<Self> = Vec::new();
        for i in app {
            let record1 = Request {
                applicant: i.id.clone(),
                vacancy: i.preference01,
                preference: 1,
            };
            let record2 = Request {
                applicant: i.id.clone(),
                vacancy: i.preference02,
                preference: 2,
            };
            let record3 = Request {
                applicant: i.id.clone(),
                vacancy: i.preference03,
                preference: 3,
            };
            board.push(record1);
            board.push(record2);
            board.push(record3);
        }

        board
    }
}
