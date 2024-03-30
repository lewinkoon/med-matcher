use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Applicant {
    pub id: String,
    pub pref01: String,
    pub pref02: String,
    pub pref03: String,
    name: String,
    surname1: String,
    surname2: String,
    birthday: String,
    specialty: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Vacancy {
    id: String,
    quantity: String,
    shift: String,
    department: String,
    specialty: String,
    profile: String,
}
