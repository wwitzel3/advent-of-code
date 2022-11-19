use std::iter::Enumerate;

fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");

    let mut c_x = 0;
    let mut c_y = 0;

    let mut x_robo = 0;
    let mut y_robo = 0;

    let mut v: Vec<(i32, i32)> = Vec::new();
    v.push((c_x, c_y));

    let mut v_robo: Vec<(i32, i32)> = Vec::new();
    v_robo.push((x_robo, y_robo));

    let mut count = 0;
    for d in input.as_bytes() {
        let store = if count % 2 == 0 { &mut v } else { &mut v_robo };
        let x = if count % 2 == 0 {
            &mut c_x
        } else {
            &mut x_robo
        };

        let y = if count % 2 == 0 {
            &mut c_y
        } else {
            &mut y_robo
        };

        println!("{x},{y}");

        if *d == b'^' {
            // north
            println!("north!");
            *x += 1;
        } else if *d == b'v' {
            // south
            println!("south!");
            *x -= 1;
        } else if *d == b'>' {
            // east
            println!("east!");
            *y += 1;
        } else if *d == b'<' {
            // west
            println!("west!");
            *y -= 1;
        }
        store.push((*x, *y));
        count += 1;
    }
    v.append(&mut v_robo);

    v.sort();
    v.dedup();

    println!("{:?}", v.len());
}
