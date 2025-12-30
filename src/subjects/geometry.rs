use rand::Rng;
use rand::seq::IndexedRandom;
use crate::models::{Quiz, Answer};
use crate::tools::function::pythagorean_triplet;

fn q_triangle() -> Quiz {
    const MIN: usize = 3; const MAX: usize = 10;
    const SENTENCES: [&str; 3] = [
        "Détermine la nature du triangle aux côtés ${a}$, ${b}$, et ${c}$.",
        "À quelle catégorie appartient le triangle avec des côtés de ${a}$, ${b}$ et ${c}$ ?",
        "Identifie la nature du triangle ayant pour côtés ${a}$, ${b}$, et ${c}$."
    ];

    let mut rng = rand::rng();

    let values: [String; 4] = [
        "rectangle".to_string(), "isocèle".to_string(),
        "équilatéral".to_string(), "quelconque".to_string()
    ];

    let index_answer: usize = rng.random_range(0..4);

    let sides: (usize, usize, usize) = match index_answer {
        0 => {
            pythagorean_triplet(MIN, MAX)
                .choose(&mut rng)
                .copied()
                .expect("No Pythagorean triplets found")
        }
        2 => {
            // triangle équilatéral
            let a = rng.random_range(MIN..=MAX);
            (a, a, a)
        }
        1 => {
            // triangle isocèle
            let a = rng.random_range(MIN..=MAX);
            let b = loop {
                let b = rng.random_range(MIN..=MAX);
                if b != a {
                    break b;
                }
            };
            match rng.random_range(0..3) {
                0 => (a, a, b),
                1 => (a, b, a),
                _ => (b, a, a)
            }

        }
        _ => {
            // triangle quelconque
            let a = rng.random_range(MIN..=MAX);
            let b = loop {
                let b = rng.random_range(MIN..=MAX);
                if b != a {
                    break b;
                }
            };
            let c = loop {
                let c = rng.random_range(MIN..=MAX);
                if c != a && c != b {
                    break c;
                }
            };
            (a, b, c)
        }
    };

    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("{a}", &sides.0.to_string())
        .replace("{b}", &sides.1.to_string())
        .replace("{c}", &sides.2.to_string());

    Quiz { text, suggested_answer: Answer::Open(values), index_answer }
}

pub fn generate() -> Quiz {
    let mut rng = rand::rng();
    // let x: bool = rng.random();

    match rng.random_range(0..1) {
        0 => q_triangle(),
        _ => q_triangle(),
    }
}