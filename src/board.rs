use crate::input::Applicant;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Request {
    pub applicant: String,
    pub vacancy: String,
    preference: usize,
}

impl Request {
    pub fn build(app: Vec<Applicant>) -> Vec<Self> {
        let mut board: Vec<Self> = Vec::new();
        for i in app {
            board.push(Request {
                applicant: i.id.clone(),
                vacancy: i.pref01,
                preference: 1,
            });
            board.push(Request {
                applicant: i.id.clone(),
                vacancy: i.pref02,
                preference: 2,
            });
            board.push(Request {
                applicant: i.id.clone(),
                vacancy: i.pref03,
                preference: 3,
            });
        }

        board
    }
}
