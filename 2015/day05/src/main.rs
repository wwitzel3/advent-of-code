fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    let mut count = 0;
    for s in input.lines() {
        if new_string_eval(s) {
            count += 1;
        }
    }

    println!("{count}");
}

fn new_string_eval(s: &str) -> bool {
    let s = format!("{:}", s).to_string();
    let length = s.len();

    let mut has_pair = false;
    let mut has_skip = false;

    for i in 0..length - 1 {
        if i == length {
            break;
        }

        if !has_pair {
            let pattern = s.get(i..i + 2).unwrap();
            let pairs = s.split(pattern).collect::<Vec<&str>>();
            let matches = pairs.len();
            if matches >= 3 {
                has_pair = true;
            }
        }

        if !has_skip {
            if i + 3 <= length {
                let pattern = s.get(i..i + 3).unwrap();
                let c1 = pattern.as_bytes()[0];
                let c2 = pattern.as_bytes()[2];
                if c1 == c2 {
                    has_skip = true;
                }
            }
        }
    }
    has_pair && has_skip
}

fn string_eval(s: &str) -> bool {
    let bad = ["ab", "cd", "pq", "xy"];
    let vowels = [b'a', b'e', b'i', b'o', b'u'];

    let s = format!("{:}", s).to_string();
    for b in bad {
        if s.contains(b) {
            return false;
        }
    }

    let length = s.len();
    let mut vowel_count = 0;
    let mut double = false;

    for i in 0..length {
        println!("{i}");
        let c1 = s.as_bytes()[i];

        if vowels.contains(&c1) {
            println!("vowel {:?} {:?}", c1, vowels[0]);
            vowel_count += 1;
        }

        if !double && i < length - 1 {
            let c2 = s.as_bytes()[i + 1];
            if c1 == c2 {
                double = true;
            }
        }
    }

    println!("{vowel_count}");
    if vowel_count >= 3 && double {
        return true;
    }
    false
}
