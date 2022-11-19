use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle.input")
        .expect("sholud have been able to read puzzle input file");

    let mut counter = 0;

    for (i, &item) in input.as_bytes().iter().enumerate() {
        if item == b'(' {
            counter += 1;
        } else {
            counter -= 1;
        }
        if counter == -1 {
            let step = i + 1;
            println!("Basement: {step}");
        }
    }

    println!("Steps: {counter}");
}
