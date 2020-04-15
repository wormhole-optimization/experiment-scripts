use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = &args[1];
    let fc = fs::read_to_string(f).unwrap();
    let mut ln = 0;
    for l in fc.lines() {
        if l.contains(':') {
            let mut s = l.to_owned();
            s.pop();
            ln = s.parse::<u32>().unwrap();
        } else {
            println!("{},{}", ln, l);
        }
    }
}
