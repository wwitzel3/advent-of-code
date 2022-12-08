#[derive(Copy, Debug, Clone)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn main() {
    let stack1 = vec!["Z", "T", "F", "R", "W", "J", "G"];
    let stack2 = vec!["G", "W", "M"];
    let stack3 = vec!["J", "N", "H", "G"];
    let stack4 = vec!["J", "R", "C", "N", "W"];
    let stack5 = vec!["W", "F", "S", "B", "G", "Q", "V", "M"];
    let stack6 = vec!["S", "R", "T", "D", "V", "W", "C"];
    let stack7 = vec!["H", "B", "N", "C", "D", "Z", "G", "V"];
    let stack8 = vec!["S", "J", "N", "M", "G", "C"];
    let stack9 = vec!["G", "P", "N", "W", "C", "J", "D", "L"];

    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    // let stack1 = vec!["Z", "N"];
    // let stack2 = vec!["M", "C", "D"];
    // let stack3 = vec!["P"];

    let mut stacks = vec![
        stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9,
    ];

    for line in input.lines() {
        let instruction = parse(line);
        // println!("{:?}", instruction);

        let (head, tail) = stacks.split_at_mut(instruction.from);

        let from: &mut Vec<&str>;
        let to: &mut Vec<&str>;

        if instruction.from < instruction.to {
            let head_len = head.len();
            from = &mut head[instruction.from - 1];
            to = &mut tail[instruction.to - 1 - head_len];
        } else {
            let (h, t) = head.split_at_mut(instruction.from - 1);
            from = &mut t[0];
            to = &mut h[instruction.to - 1];
        }

        let mut tmp: Vec<&str> = Vec::new();
        for _ in 0..instruction.count {
            let v = from.pop().unwrap();
            tmp.push(v);
        }
        tmp.reverse();
        to.append(&mut tmp);
    }
    for s in &stacks {
        let s = s.last().unwrap();
        print!("{s}");
    }
    println!("");
}

fn parse(l: &str) -> Instruction {
    let values: Vec<&str> = l.split(" ").collect();
    let num = values[1].parse::<usize>().unwrap();
    let p1 = values[3].parse::<usize>().unwrap();
    let p2 = values[5].parse::<usize>().unwrap();

    Instruction {
        count: num,
        from: p1,
        to: p2,
    }
}
