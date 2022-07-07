use std::{
    io::{stdin, stdout, Write},
    vec,
};
struct Candidate {
    name: String,
    number: i32,
    votes: i32,
}
impl Candidate {
    fn new(name: &str, number: i32) -> Candidate {
        Candidate {
            name: name.to_string(),
            number: number,
            votes: 0,
        }
    }
    fn add_vote(&mut self) {
        self.votes += 1;
    }
}
fn is_valid_vote(vote: i32, numbers: &Vec<i32>) -> bool {
    numbers.contains(&vote)
}
fn print_candidates(candidates: &Vec<Candidate>) {
    for candidate in candidates {
        println!("{} - {}", candidate.name, candidate.number);
    }
}
fn sort_candidates(candidates: &mut Vec<Candidate>) {
    candidates.sort_by(|a, b| b.votes.cmp(&a.votes));
}
fn input(question: &str) -> String {
    let mut buffer = String::new();
    print!("{}", question);
    stdout().flush().unwrap();
    stdin().read_line(&mut buffer).expect("Error in stdin");
    buffer
}
fn get_candidates() -> Vec<Candidate> {
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut numbers = vec![-1];
    let num_votes: i32 = input("How many candidates do you want: ")
        .trim()
        .parse()
        .expect("Error in parsing");
    for _ in 0..num_votes {
        let name = input("Type the candidate's name: ");
        let mut number = -1;
        while numbers.contains(&number) {
            let mut question = String::from("Type the candidate's number");
            let warning = " (It cannot be a number already in use)";
            if number != -1 {
                question = question + warning;
            }
            number = input(format!("{}: ", question).as_str())
                .trim()
                .parse()
                .expect("Error while parsing");
        }
        candidates.push(Candidate::new(name.trim(), number));
        numbers.push(number);
    }
    candidates
}
fn main() {
    let mut candidates = get_candidates();
    let num_votes: i32;
    let mut numbers: Vec<i32> = Vec::new();
    println!("Hello, welcome to my voting system!\n\n");

    num_votes = input("How many people are gonna vote: ")
        .trim()
        .parse::<i32>()
        .expect("Error while parsing input");
    let _ = candidates
        .iter()
        .map(|candidate| candidate.number)
        .for_each(|number| numbers.push(number));
    for _ in 1..=num_votes {
        let mut vote = -1;
        print_candidates(&candidates);
        while !is_valid_vote(vote, &numbers) {
            vote = input("Please insert the number of one of the above candidates: ")
                .trim()
                .parse::<i32>()
                .expect("Error while parsing vote")
        }
        for candidate in &mut candidates {
            if candidate.number == vote {
                candidate.add_vote();
                break;
            }
        }
        print!("\x1B[2J\x1B[1;1H");
    }
    sort_candidates(&mut candidates);
    for candidate in candidates {
        println!(
            "{} had {} votes, a total of {:.1}% of the votes",
            candidate.name,
            candidate.votes,
            (candidate.votes as f32 / num_votes as f32) * 100.0
        )
    }
}
