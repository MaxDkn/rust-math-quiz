use rand::Rng;
use crate::models::{Quiz, Answer };
use rand::prelude::IndexedRandom;

fn q_antecedent() -> Quiz {
    // const MIN_A: isize = -4; const MAX_A: isize = 4;
    // const MIN_C: isize = -2; const MAX_C: isize = 2;
    // const MIN_X: isize = -10; const MAX_X: isize = 10;
    const SENTENCES: [&str; 3] = [
        "Quelle est la valeur de ${x}$ dans ${eq}={c}$ ?",
        "Quelle est la solution de ${eq}={c}$ ?",
        "Donner l'antécédent de ${c}$ par $f$ avec $f(x)={equation}$."
    ];

    let mut rng = rand::rng();

    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("${x}$", "${x}$");

    Quiz { text, suggested_answer: Answer::Close([true, false]), index_answer: 1 }
}

pub fn generate() -> Quiz {
    let mut rng = rand::rng();
    // let x: bool = rng.random();

    match rng.random_range(0..1) {
        0 => q_antecedent(),
        _ => q_antecedent(),
    }
}