use std::collections::{BTreeMap, HashSet};

#[derive(Copy, Clone, Debug)]
enum Priority {
    a = 1,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    A = 27,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Priority {
    fn val(&self) -> u64 {
        *self as u64
    }

    fn from(c: char) -> Result<Priority, ()> {
        match c {
            'a' => Ok(Priority::a),
            'b' => Ok(Priority::b),
            'c' => Ok(Priority::c),
            'd' => Ok(Priority::d),
            'e' => Ok(Priority::e),
            'f' => Ok(Priority::f),
            'g' => Ok(Priority::g),
            'h' => Ok(Priority::h),
            'i' => Ok(Priority::i),
            'j' => Ok(Priority::j),
            'k' => Ok(Priority::k),
            'l' => Ok(Priority::l),
            'm' => Ok(Priority::m),
            'n' => Ok(Priority::n),
            'o' => Ok(Priority::o),
            'p' => Ok(Priority::p),
            'q' => Ok(Priority::q),
            'r' => Ok(Priority::r),
            's' => Ok(Priority::s),
            't' => Ok(Priority::t),
            'u' => Ok(Priority::u),
            'v' => Ok(Priority::v),
            'w' => Ok(Priority::w),
            'x' => Ok(Priority::x),
            'y' => Ok(Priority::y),
            'z' => Ok(Priority::z),
            'A' => Ok(Priority::A),
            'B' => Ok(Priority::B),
            'C' => Ok(Priority::C),
            'D' => Ok(Priority::D),
            'E' => Ok(Priority::E),
            'F' => Ok(Priority::F),
            'G' => Ok(Priority::G),
            'H' => Ok(Priority::H),
            'I' => Ok(Priority::I),
            'J' => Ok(Priority::J),
            'K' => Ok(Priority::K),
            'L' => Ok(Priority::L),
            'M' => Ok(Priority::M),
            'N' => Ok(Priority::N),
            'O' => Ok(Priority::O),
            'P' => Ok(Priority::P),
            'Q' => Ok(Priority::Q),
            'R' => Ok(Priority::R),
            'S' => Ok(Priority::S),
            'T' => Ok(Priority::T),
            'U' => Ok(Priority::U),
            'V' => Ok(Priority::V),
            'W' => Ok(Priority::W),
            'X' => Ok(Priority::X),
            'Y' => Ok(Priority::Y),
            'Z' => Ok(Priority::Z),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle.input")
        .expect("should have been ablet to read puzzle.input");
    let lines: Vec<&str> = input.lines().collect();

    let mut total = 0;
    let sz: usize = 3;

    for chunk in lines.chunks(sz) {
        let first: HashSet<char> = chunk[0].chars().collect();
        let second: HashSet<char> = chunk[1].chars().collect();
        let third: HashSet<char> = chunk[2].chars().collect();

        let i: HashSet<_> = first.intersection(&second).collect();
        for c in i {
            if third.contains(c) {
                let priority = Priority::from(*c).unwrap();
                total += priority.val();
            }
        }
    }
    println!("{}", total);
}
