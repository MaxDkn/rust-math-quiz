use rand::Rng;
use crate::models::Quiz;

pub mod algebra;
pub mod arithmetic;
pub mod geometry;

pub enum Subject {
    Algebra,
    Arithmetic,
    Geometry,
}

pub fn generate(subject: Option<Subject>) -> Quiz {
    let mut rng = rand::rng();

    let chosen_subject = subject.unwrap_or_else(|| {
        match rng.random_range(0..3) {
            0 => Subject::Algebra,
            1 => Subject::Geometry,
            _ => Subject::Arithmetic,
        }
    });

    match chosen_subject {
        Subject::Algebra => algebra::generate(),
        Subject::Arithmetic => arithmetic::generate(),
        Subject::Geometry => geometry::generate(),
    }
}
