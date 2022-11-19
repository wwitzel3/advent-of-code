use std::process::ExitStatus;

fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    let mut totalPaper = 0;
    let mut totalRibbon = 0;

    for s in input.lines() {
        let values = s.split('x');
        let vec = values.collect::<Vec<&str>>();

        let l = vec[0].parse::<i32>().unwrap();
        let w = vec[1].parse::<i32>().unwrap();
        let h = vec[2].parse::<i32>().unwrap();
        let mut lengths: [i32; 3] = [l, w, h];
        lengths.sort();

        let ribbon = lengths[0] * 2 + lengths[1] * 2;
        let volume = l * w * h;
        totalRibbon += ribbon + volume;

        let l_a = l * w;
        let w_a = w * h;
        let h_a = h * l;

        let mut areas = [l_a, w_a, h_a];
        areas.sort();

        let extra: i32 = areas[0];
        let area: i32 = l_a * 2 + w_a * 2 + h_a * 2;
        totalPaper += area + extra;
    }
    println!("{totalPaper}");
    println!("{totalRibbon}");
}
