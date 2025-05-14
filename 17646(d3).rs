use std::collections::HashMap;
use std::io::{Result, Write, stdin, stdout};

extern "C" {
    fn rand() -> u64;
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 > 0 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn is_prime(n: u128) -> bool {
    let pil = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];

    if n == 1 {
        return false;
    }

    if pil.contains(&n) {
        return true;
    }

    for a in &pil {
        if !miller_rabin(n, *a) {
            return false;
        }
    }
    true
}

fn miller_rabin(n: u128, k: u128) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 || n <= 1 {
        return false;
    }

    let mut s: u128 = 0;
    let mut d: u128 = n - 1;

    while d % 2 == 0 {
        s += 1;
        d /= 2;
    }

    for _ in 0..k {
        let a = unsafe { rand() as u128 % (n - 2) + 1 };
        let mut x = mod_pow(a, d, n);

        if x == 1 || x == n - 1 {
            continue;
        }

        let mut is_composite = true;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                is_composite = false;
                break;
            }
        }

        if is_composite {
            return false;
        }
    }
    true
}

fn gcd(a: i128, b: i128) -> i128 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }

    if is_prime(n) {
        return n;
    }

    if n == 1 {
        return 1;
    }

    let mut x: u128 = unsafe { rand() as u128 % (n - 1) + 1 };
    let mut y: u128 = x;
    let c: u128 = unsafe { rand() as u128 % (n - 1) + 1 };
    let mut g: u128 = 1;

    while g == 1 {
        x = (mod_pow(x, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;

        g = gcd((x as i128 - y as i128).abs(), n as i128) as u128;

        if g == n {
            return pollard_rho(n);
        }
    }

    if is_prime(g) { g } else { pollard_rho(g) }
}

fn prime_factors(mut n: u128) -> HashMap<u128, u128> {
    if n == 1 {
        return HashMap::new();
    }

    if is_prime(n) {
        let mut factors = HashMap::new();
        factors.insert(n, 1);
        return factors;
    }

    let mut factors = HashMap::new();
    while n != 1 {
        let factor = pollard_rho(n);
        *factors.entry(factor).or_insert(0) += 1;
        n /= factor;
    }

    factors
}

fn count(mut n: u128) -> u128 {
    while n % 4 == 0 {
        n /= 4;
    }

    if n % 8 == 7 {
        4
    } else {
        let factors = prime_factors(n);
        let mut s = HashMap::new();

        for (factor, &cnt) in factors.iter() {
            if cnt % 2 == 1 {
                s.insert(*factor, true);
            }
        }

        for factor in s.keys() {
            if factor % 4 == 3 {
                return 3;
            }
        }

        let sqrt = (n as f64).sqrt().floor() as u128;
        match sqrt * sqrt == n {
            true => 1,
            false => 2,
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n: u128 = input.trim().parse::<u128>().unwrap();

    let ch = count(n);

    writeln!(stdout(), "{}", ch)?;

    Ok(())
}
