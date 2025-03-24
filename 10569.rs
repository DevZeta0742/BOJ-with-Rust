use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut testcase_str: String = String::new();
    io::stdin().read_line(&mut testcase_str).expect("error");
    let testcase: i32 = testcase_str.trim().parse().expect("not a number");

    for _ in 0..testcase {
        let mut input_str: String = String::new();
        io::stdin().read_line(&mut input_str).expect("error");
        let mut iter = input_str.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().expect("not a number");
        let b: i32 = iter.next().unwrap().parse().expect("not a number");

        writeln!(out, "{}", 2 - a + b).expect("error");
    }
}
