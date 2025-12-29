// Max Deckmyn
// TODO :
// q_gcd_lcm
// Make class for the different subject
// Algebra | Geometry | Trigonometry

mod tools;
mod models;
mod subjects;

use subjects::Subject;

fn main() {
    let question = subjects::generate(Some(Subject::Arithmetic));
    println!("{}\n{}", question.text, question.suggested_answer);
    println!("Answer: {}", ['A', 'B', 'C', 'D'][question.index_answer]);

}
