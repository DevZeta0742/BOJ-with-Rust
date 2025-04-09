use std::error::Error;
use std::fmt::Debug;
use std::io::{self, Write};
use std::str::FromStr;
use std::collections::HashSet;

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
    let mut input_lines = Vec::new();

    for _ in 0..10 {
        let number: i32 = input::<i32>()[0];
        input_lines.push(number % 42);
    }

    let output = input_lines
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .len();

    writeln!(io::stdout(), "{}", output)?;

    Ok(())
}
