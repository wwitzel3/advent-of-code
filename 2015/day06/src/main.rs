fn main() {
    let mut grid = vec![vec![0; 1000]; 1000];
    // grid[0][0] = true;

    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    for line in input.lines() {
        let command = line.split(" through ").collect::<Vec<&str>>();

        let stop = command[1].split(",").collect::<Vec<&str>>();
        let stop = (
            stop[0].parse::<usize>().unwrap(),
            stop[1].parse::<usize>().unwrap(),
        );

        let front = command[0].replace("turn ", "");
        let front = front.split(" ").collect::<Vec<&str>>();

        let command = front[0];

        let start = front[1].split(",").collect::<Vec<&str>>();
        let start = (
            start[0].parse::<usize>().unwrap(),
            start[1].parse::<usize>().unwrap(),
        );

        match command {
            "on" => turn_on(grid.as_mut_slice(), start, stop),
            "off" => turn_off(grid.as_mut_slice(), start, stop),
            _ => toggle(grid.as_mut_slice(), start, stop),
        }
    }

    println!("{:}", get_lit(grid.as_mut_slice()));
}

fn get_lit(g: &mut [Vec<i64>]) -> i64 {
    let mut count = 0;
    for i in g {
        for j in i {
            count += *j;
        }
    }
    count
}

fn turn_off(grid: &mut [Vec<i64>], start: (usize, usize), stop: (usize, usize)) {
    let stop = (stop.0 + 1, stop.1 + 1);
    for i in start.0..stop.0 {
        for j in start.1..stop.1 {
            if grid[i][j] == 0 {
                continue;
            }
            grid[i][j] -= 1;
        }
    }
}

fn turn_on(grid: &mut [Vec<i64>], start: (usize, usize), stop: (usize, usize)) {
    let stop = (stop.0 + 1, stop.1 + 1);
    for i in start.0..stop.0 {
        for j in start.1..stop.1 {
            grid[i][j] += 1;
        }
    }
}

fn toggle(grid: &mut [Vec<i64>], start: (usize, usize), stop: (usize, usize)) {
    let stop = (stop.0 + 1, stop.1 + 1);
    for i in start.0..stop.0 {
        for j in start.1..stop.1 {
            grid[i][j] += 2;
        }
    }
}
