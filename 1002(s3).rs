use std::io::{stdin, stdout, Result, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let testcase = input.trim().parse::<u32>().unwrap();
    
    for _ in 0..testcase {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let arr: Vec<&str> = input.trim().split(' ').collect();
        
        let x1 = arr[0].parse::<i32>().unwrap();
        let y1 = arr[1].parse::<i32>().unwrap();
        let r1 = arr[2].parse::<i32>().unwrap();
        let x2 = arr[3].parse::<i32>().unwrap();
        let y2 = arr[4].parse::<i32>().unwrap();
        let r2 = arr[5].parse::<i32>().unwrap();

        let dist = (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
        
        if dist == 0.0 && r1 == r2 {
            writeln!(stdout(), "-1")?;
        }
        else if (r1 - r2).abs() as f64 == dist || (r1 + r2) as f64 == dist {
            writeln!(stdout(), "1")?;
        }
        else if ((r1 - r2).abs() as f64) < dist && dist < (r1 + r2) as f64 {
            writeln!(stdout(), "2")?;
        }
        else {
            writeln!(stdout(), "0")?;
        }
    }

    Ok(())
}
