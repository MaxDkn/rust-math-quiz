use std::fmt;

pub enum Answer {
    Close([bool; 2]), // Vrai/Faux
    Open([String; 4]), // QCM
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::Close(answers) =>
                write!(f, "A. {} | B. {}", answers[0], answers[1]),
            Answer::Open(answers) =>
                write!(f, "A. {} | B. {} | C. {} | D. {}", answers[0], answers[1], answers[2], answers[3]),
        }
    }
}

pub struct Quiz {
    pub text: String,
    pub suggested_answer: Answer,
    pub index_answer: usize,
}