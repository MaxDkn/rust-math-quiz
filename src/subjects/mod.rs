use rand::Rng;
use crate::models::Quiz;

pub mod algebra;
pub mod arithmetic;

pub enum Subject {
    Algebra,
    Arithmetic,
}

pub fn generate(subject: Option<Subject>) -> Quiz {
    let mut rng = rand::rng();

    let chosen_subject = subject.unwrap_or_else(|| {
        match rng.random_range(0..2) {
            0 => Subject::Algebra,
            _ => Subject::Arithmetic,
        }
    });

    match chosen_subject {
        Subject::Algebra => algebra::generate(),
        Subject::Arithmetic => arithmetic::generate(),
    }
}
