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
    let n = input::<usize>();
    let n = n[0];

    if n == 0 {
        writeln!(io::stdout(), "0")?;
        return Ok(());
    }
    
    if n == 1 {
        writeln!(io::stdout(), "1")?;
        return Ok(());
    }

    let mut arr = vec![0; n];
    arr[0] = 1;
    arr[1] = 1;

    for i in 2..n {
        arr[i] = arr[i - 1] + arr[i - 2];
    }

    let output = if n > 0 { arr[n - 1] } else { 0 };

    writeln!(io::stdout(), "{}", output)?;

    Ok(())
}
