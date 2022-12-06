fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    let mut overlap = 0;
    for line in input.lines() {
        let (x, y, a, b) = parse_line(line);
        if is_overlapping(x, y, a, b) {
            overlap += 1;
        }
    }
    println!("{overlap}")
}

fn parse_line(s: &str) -> (u64, u64, u64, u64) {
    let pair: Vec<&str> = s.split(',').collect();
    let xy: Vec<&str> = pair.first().unwrap().split("-").collect();
    let ab: Vec<&str> = pair.last().unwrap().split("-").collect();

    let x = xy.first().unwrap().parse::<u64>().unwrap();
    let y = xy.last().unwrap().parse::<u64>().unwrap();
    let a = ab.first().unwrap().parse::<u64>().unwrap();
    let b = ab.last().unwrap().parse::<u64>().unwrap();

    println!("{x}-{y},{a}-{b}");
    return (x, y, a, b);
}

fn is_overlapping(x: u64, y: u64, a: u64, b: u64) -> bool {
    x <= b && a <= y
}

fn is_completely_overlapping(x: u64, y: u64, a: u64, b: u64) -> bool {
    x >= a && y <= b || a >= x && b <= y
}
