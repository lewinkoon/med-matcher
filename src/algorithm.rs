use crate::board::Request;
use itertools::Itertools;

pub struct Proposor {
    id: String,
    list: Vec<String>,
}

impl Proposor {
    pub fn build(board: Vec<Request>) -> Vec<Self> {
        let proposors: Vec<Self> = board
            .into_iter()
            .group_by(|req| req.applicant.clone())
            .into_iter()
            .map(|(applicant, group)| {
                let vacancies: Vec<String> = group.map(|req| req.vacancy.clone()).collect();
                Proposor {
                    id: applicant,
                    list: vacancies,
                }
            })
            .collect();
        proposors
    }
}
