use std::iter::Enumerate;

extern crate md5;

fn main() {
    let input = "iwrupvqb";

    for i in 0..std::u64::MAX {
        let value = format!("{input}{:0>5}", i);
        let s_value = value.as_str();

        let digest = md5::compute(s_value);

        let s_digest = format!("{:x}", digest).to_string();
        if s_digest.starts_with("000000") {
            println!("{i} Found {:?}", digest);
            break;
        }
    }
}
