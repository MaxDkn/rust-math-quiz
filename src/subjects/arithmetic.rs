use std::array::from_fn;
use rand::{Rng};
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;
use crate::tools::function::{format_answers, fill_unique_random};
use crate::models::{ Quiz, Answer };

fn q_is_square() -> Quiz {
    const MIN: usize = 9; const MAX: usize = 169; // numbers here has to be a perfect square!
    const SENTENCES: [&str; 3] = [
        "Le nombre ${n}$ est-il un carré parfait ?",
        "${n}$ est-il le carré d'un nombre entier ?",
        "Peut-on écrire ${n}$ comme $k^2$ avec $k\\in\\mathbb{Z}$ ?"
    ];
    const ANSWERS: [bool; 2] = [true, false];

    let mut rng = rand::rng();

    let index_answer: usize = rng.random_range(0..=1);

    let mut num: usize;

    if ANSWERS[index_answer] {
        let squares: [usize; MAX.isqrt() - MIN.isqrt() + 1] = from_fn(|x| (x+ MIN.isqrt()) * (x + MIN.isqrt()));

        num = *squares.choose(&mut rng).unwrap();
    } else { loop {
        num = rng.random_range(MIN..=MAX);
        let temp = num.isqrt();
        if !(temp * temp == num) {
            break
        } };
    }
    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("{n}", &num.to_string());
    Quiz { text, suggested_answer: Answer::Close(ANSWERS), index_answer }
}

fn q_is_prime() -> Quiz {
    const MIN: usize = 4; const MAX: usize = 40;
    const SENTENCES: [&str; 3] = [
        "${n}$ est-il divisible uniquement par $1$ et lui-même ?",
        "Est-ce que ${n}$ est un nombre premier ?",
        "Le nombre ${n}$ est-il un nombre premier ?"
    ];
    const ANSWERS: [bool; 2] = [true, false];
    const PRIME_NUMBERS: [usize; 10] = [5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut rng = rand::rng();

    let index_answer: usize = rng.random_range(0..=1);

    let mut num: usize;

    if ANSWERS[index_answer] {
        num = *PRIME_NUMBERS.choose(&mut rng).unwrap();
    } else {
        loop {
            num = rng.random_range(MIN..=MAX);
            if !PRIME_NUMBERS.contains(&num) {
                break
            }
        }
    }

    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("{n}", num.to_string().as_str());

    Quiz { text, suggested_answer: Answer::Close(ANSWERS), index_answer }
}

fn q_div_rem() -> Quiz {
    const MIN: usize = 10; const MAX: usize = 70;
    const MIN_DIVISOR: usize = 4; const MAX_DIVISOR: usize = 9;
    const SENTENCES: [&str; 2] = [
        "Quel est le reste de la division euclidienne de ${dividend}$ par ${divisor}$ ?",
        "Que vaut $x$ dans ${dividend}\\equiv x\\pmod{{divisor}}$.",
    ];

    let mut rng = rand::rng();

    let dividend = rng.random_range(MIN..=MAX);
    let divisor = rng.random_range(MIN_DIVISOR..=MAX_DIVISOR);

    let answer = dividend % divisor;

    // Génération des réponses
    let mut values = vec![answer];
    fill_unique_random(&mut values, 4, || { rng.random_range(0..divisor )});
    values.shuffle(&mut rng);

    let answers: [String; 4] = format_answers(&values, |v| format!("${v}$"));

    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("{dividend}", &dividend.to_string())
        .replace("{divisor}", &divisor.to_string());

    let index_answer = values.iter().position(|&v| v == answer).unwrap();

    Quiz {
        text,
        suggested_answer: Answer::Open(answers),
        index_answer,
    }
}

fn q_gcd() {
    const MIN: usize = 20; const MAX: usize = 40;
    const MIN_SOLUTION: usize = 2; const MAX_SOLUTION: usize = 6;
    const SENTENCES: [&str; 3] = [
        "Trouve le {gcd_or_lcm} entre {l}{n1}{l} et {l}{n2}{l}.",
        "",
        ""
    ];

}

fn q_conv_dec() -> Quiz {
    const MIN: usize = 5; const MAX: usize = 31;
    const SENTENCES: [&str; 3] = [
        "Transforme le nombre ${n}$ en nombre binaire.",
        "Exprime ${n}$ en base $2$.",
        "Convertis ${n}$ du décimal vers le binaire."
    ];

    let mut rng = rand::rng();

    let value = rng.random_range(MIN..=MAX);
    let mut values = vec![value];
    fill_unique_random(&mut values, 4, || rng.random_range(MIN..=MAX));
    values.shuffle(&mut rng);

    let answers: [String; 4] = format_answers(&values, |v| format!("${:05b}_2$", v));

    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("{n}", &value.to_string());

    let suggested_answer = Answer::Open(answers);
    let index_answer = values.iter().position(|&v| v == value).unwrap();
    Quiz { text, suggested_answer, index_answer }
}

fn q_conv_bin() -> Quiz {
    const MIN: usize = 5; const MAX: usize = 31;
    const SENTENCES: [&str; 3] = [
        "Transforme le nombre ${n}_2$ en nombre décimal.",
        "Exprime ${n}_2$ en base $10$.",
        "Convertis ${n}_2$ du binaire vers le décimal."
    ];

    let mut rng = rand::rng();

    let value = rng.random_range(MIN..=MAX);
    let mut values = vec![value];

    fill_unique_random(&mut values, 4, || rng.random_range(MIN..=MAX));

    values.shuffle(&mut rng);

    let answers: [String; 4] = format_answers(&values, |v| format!("${v}$"));

    let text = SENTENCES
        .choose(&mut rng)
        .unwrap()
        .replace("{n}", &format!("{:05b}", value));

    let suggested_answer = Answer::Open(answers);
    let index_answer = values.iter().position(|&v| v == value).unwrap();
    Quiz { text, suggested_answer, index_answer }
}

pub fn generate() -> Quiz {
    let mut rng = rand::rng();
    let x: bool = rng.random();

    match rng.random_range(0..4) {
        0 => q_is_square(),
        1 => q_is_prime(),
        2 => q_div_rem(),
        _ => if x { q_conv_dec() } else { q_conv_bin() }
    }
}