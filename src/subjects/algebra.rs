use rand::Rng;
use crate::models::{Quiz, Answer};
use crate::tools::function::format_answers;
use rand::prelude::{IndexedRandom, SliceRandom};

fn q_prod() -> Quiz {
    const MIN: usize = 6; const MAX: usize = 12;
    // const MIN_C: isize = -2; const MAX_C: isize = 2;
    // const MIN_X: isize = -10; const MAX_X: isize = 10;
    const SENTENCES: [&str; 3] = [
        "Quel est le produit de ${a}$ par ${b}$ ?",
        "Combien font ${a}\\times{b}$ ?",
        "${a}\\times{b}=?$"
    ];

    let mut rng = rand::rng();

    let special = rng.random_bool(0.3); // Wanna have question about the 11 table?

    let a = if special { 11 } else { rng.random_range(MIN..=MAX) };
    let b = if special {
        rng.random_range(12..=99)
    } else {
        rng.random_range(MIN..=MAX)
    };

    let value = a * b;
    let mut answers = vec![value];

    let range = if special { 12..=99 } else { MIN..=MAX };

    while answers.len() < 4 {
        let v = a * rng.random_range(range.clone());
        if !answers.contains(&v) {
            answers.push(v);
        }
    }
    answers.shuffle(&mut rng);
    let index_answer = answers.iter().position(|&a| a == value).unwrap();

    let answers = format_answers(&answers, |v| format!("${}$", v));

    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("{a}", &a.to_string())
        .replace("{b}", &b.to_string());

    Quiz { text, suggested_answer: Answer::Open(answers), index_answer}
}

pub fn generate() -> Quiz {
    let mut rng = rand::rng();
    // let x: bool = rng.random();

    match rng.random_range(0..1) {
        0 => q_prod(),
        _ => q_prod(),
    }
}