// 빠른 A+B
// Rust에서는 buffer를 사용하지 않는 println!을 사용하지 않고 writeln!을 사용함으로써 시간초과를 피할 수 있다.
// 이걸 모르고 println!으로 제출했다가 tle가 나버렸다.
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut testcase_str: String = String::new();
    io::stdin().read_line(&mut testcase_str).expect("error");
    let testcase: i32 = testcase_str.trim().parse().expect("not a number");

    let mut counter = 0;
    loop {

        if counter >= testcase {
            break;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let numbers: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("error"))
            .collect();

        let a = numbers[0];
        let b = numbers[1];

        writeln!(out, "{}", a + b);
        counter += 1;
    }
}
