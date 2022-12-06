use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input.sample");
    let sz = input.len();

    let signal_size = 14;

    for i in 0..sz {
        let end = i + signal_size;
        if end >= sz {
            break;
        }
        let signal: HashSet<char> = input[i..end].chars().collect();
        if signal.len() == signal_size {
            println!("{end}");
            break;
        }
    }
}
