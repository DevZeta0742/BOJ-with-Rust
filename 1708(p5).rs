// i32로 썼다가 overflow나서 6% 틀 계속 경험함. 다음에는 자료형크기 잘 생각해보고 사용하자...

use std::io::{stdin, stdout, Write, Result};

fn ccw(p1: &(i64, i64), p2: &(i64, i64), p3: &(i64, i64)) -> i64 {
    (p2.0 - p1.0) * (p3.1 - p1.1) - (p2.1 - p1.1) * (p3.0 - p1.0)
}

fn convex_hull(points: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    points.sort();

    let mut lower = Vec::new();
    for p in points.iter() {
        while lower.len() >= 2 && ccw(&lower[lower.len() - 2], &lower[lower.len() - 1], p) <= 0 {
            lower.pop();
        }
        lower.push(*p);
    }

    let mut upper = Vec::new();
    for p in points.iter().rev() {
        while upper.len() >= 2 && ccw(&upper[upper.len() - 2], &upper[upper.len() - 1], p) <= 0 {
            upper.pop();
        }
        upper.push(*p);
    }

    lower.pop();
    upper.pop();

    lower.extend(upper);
    lower
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Invalid input for n");

    let mut points = Vec::new();
    for _ in 0..n {
        input.clear();
        stdin().read_line(&mut input)?;
        let coords: Vec<i64> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();

        points.push((coords[0], coords[1]));
    }

    writeln!(stdout(), "{}", convex_hull(&mut points).len())?;

    Ok(())
}
