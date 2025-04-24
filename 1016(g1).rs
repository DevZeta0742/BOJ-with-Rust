use std::io::{stdin, stdout, Result, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let arr: Vec<&str> = input.trim().split(' ').collect();

    let min: i64 = arr[0].parse::<i64>().unwrap();
    let max: i64 = arr[1].parse::<i64>().unwrap();

    let mut sieve: Vec<bool> = vec![true; (max - min + 1) as usize];

    for i in 2..=((max as f64).sqrt() as i64) {
        let square = i * i;

        for j in ((min + square - 1) / square)..=(max / square) {
            sieve[(square * j - min) as usize] = false;
        }
    }

    writeln!(stdout(), "{}", sieve.iter().filter(|&&x| x).count())?;

    Ok(())
}
