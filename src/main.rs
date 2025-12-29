mod tools;
mod models;
mod subjects;

use subjects::Subject;
use models::Answer;

fn count() {
    let mut count_a_close = 0usize;
    let mut count_b_close = 0usize;
    let mut count_a = 0usize;
    let mut count_b = 0usize;
    let mut count_c = 0usize;
    let mut count_d = 0usize;

    for _ in 0..10 {
        let question = subjects::generate(None);

        match question.suggested_answer {
            Answer::Close(_) => {
                match question.index_answer {
                    0usize => {count_a_close += 1},
                    1usize => {count_b_close += 1},
                    _ => {panic!("Unexpected answer");}
                }
            },
            Answer::Open(_) => {
                match question.index_answer {
                    0usize => {count_a += 1},
                    1usize => {count_b += 1},
                    2usize => {count_c += 1},
                    3usize => {count_d += 1},
                    _ => {panic!("Unexpected answer");}
                }
            },
        }
    }
    println!("Close A: {}, Close B: {}, A: {}, B: {}, C: {}, D: {}",
             count_a_close, count_b_close, count_a, count_b, count_c, count_d);

}

fn main() {
    count();
    let question = subjects::generate(Some(Subject::Arithmetic));
    println!("{}\n{}", question.text, question.suggested_answer);
    println!("Answer: {}", ['A', 'B', 'C', 'D'][question.index_answer]);

}
