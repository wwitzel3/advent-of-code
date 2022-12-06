#[derive(Copy, Clone, Debug)]
enum Value {
    Rock = 1,
    Paper,
    Scissors,
}
impl Value {
    fn val(&self) -> u64 {
        *self as u64
    }
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
impl Outcome {
    fn val(&self) -> u64 {
        *self as u64
    }
}

fn lookup2(them: &str, you: &str) -> (Value, Outcome) {
    if them == "A" {
        if you == "X" {
            return (Value::Scissors, Outcome::Loss);
        } else if you == "Y" {
            return (Value::Rock, Outcome::Draw);
        } else {
            return (Value::Paper, Outcome::Win);
        }
    } else if them == "B" {
        if you == "X" {
            return (Value::Rock, Outcome::Loss);
        } else if you == "Y" {
            return (Value::Paper, Outcome::Draw);
        } else {
            return (Value::Scissors, Outcome::Win);
        }
    } else {
        if you == "X" {
            return (Value::Paper, Outcome::Loss);
        } else if you == "Y" {
            return (Value::Scissors, Outcome::Draw);
        } else {
            return (Value::Rock, Outcome::Win);
        }
    }
}

fn lookup(them: &str, you: &str) -> (Value, Outcome) {
    if them == "A" {
        if you == "X" {
            return (Value::Rock, Outcome::Draw);
        } else if you == "Y" {
            return (Value::Paper, Outcome::Win);
        } else {
            return (Value::Scissors, Outcome::Loss);
        }
    } else if them == "B" {
        if you == "X" {
            return (Value::Rock, Outcome::Loss);
        } else if you == "Y" {
            return (Value::Paper, Outcome::Draw);
        } else {
            return (Value::Scissors, Outcome::Win);
        }
    } else {
        if you == "X" {
            return (Value::Rock, Outcome::Win);
        } else if you == "Y" {
            return (Value::Paper, Outcome::Loss);
        } else {
            return (Value::Scissors, Outcome::Draw);
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    let mut score: u64 = 0;

    for (_, line) in input.lines().enumerate() {
        let throws: Vec<&str> = line.split(" ").collect();
        let you = throws.last().unwrap();
        let them = throws.first().unwrap();

        let results = lookup2(them, you);
        score += results.0.val() + results.1.val();
    }
    println!("{:?}", score);
}
