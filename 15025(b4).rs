// input 받는 법을 제대로 안배워서 고생했다. 먼저 문법을 정독하고 문제를 풀어야겠다..
use std::error::Error;
use std::fmt::Debug;
use std::io::{self, Write};
use std::str::FromStr;

fn input<T: FromStr>() -> Vec<T> where <T as FromStr>::Err: Debug {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
        .trim()
        .to_string()
        .split(" ")
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn main() -> Result<(), Box<dyn Error>> {

    let [a, b] = input::<i32>()[..] else { panic!("panic") };

    if a == 0 && b == 0 {
        writeln!(io::stdout(), "Not a moose")?;
    } else if a == b {
        writeln!(io::stdout(), "Even {}", a * 2)?;
    } else {
        writeln!(io::stdout(), "Odd {}", a.max(b) * 2)?;
    }

    Ok(())
}
