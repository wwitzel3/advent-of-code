fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    let mut totals: Vec<i64> = Vec::new();
    let mut value: i64 = 0;

    for (_, line) in input.lines().enumerate() {
        if line.len() == 0 {
            totals.push(value);
            value = 0;
            continue;
        }
        let v = line.parse::<i64>().unwrap();
        value += v;
    }

    totals.sort();
    println!("{:?}", totals.last().unwrap());

    let last3: i64 = totals.iter().rev().take(3).sum();
    println!("{last3}");
}
