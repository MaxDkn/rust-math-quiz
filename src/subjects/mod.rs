use rand::Rng;
use crate::models::Quiz;

pub mod arithmetic;

pub enum Subject {
    Arithmetic,
}

pub fn generate(subject: Option<Subject>) -> Quiz {
    let mut rng = rand::rng();

    let chosen_subject = subject.unwrap_or_else(|| {
        match rng.random_range(0..1) {
            _ => Subject::Arithmetic,
        }
    });

    match chosen_subject {
        Subject::Arithmetic => arithmetic::generate(),
    }
}
