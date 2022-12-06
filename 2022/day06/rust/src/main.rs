use std::{collections::HashSet, ops::Range};

fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input.sample");
    let ch: Vec<char> = input.to_string().chars().collect();
    let sz = ch.len();

    let signal_size = 14;

    for i in 0..sz {
        let end = i + signal_size;
        if end >= sz {
            break;
        }
        let signal: HashSet<&char> = ch[i..end].into_iter().collect();
        if signal.len() == signal_size {
            let idx = i + signal_size;
            println!("{idx}");
            break;
        }
    }
}
