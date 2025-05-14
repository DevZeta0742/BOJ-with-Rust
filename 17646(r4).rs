// u128, i128의 존재를 몰랐음
// random을 보통 시간을 많이 활용해서 하는데 이게 런타임 에러의 원인이 되었어서 외부에서 랜덤을 끌어서 써야함

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

fn pow(base: u128, exp: u128) -> u128 {
    mod_pow(base, exp, u128::MAX)
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

fn tonelli_shanks(n: u128, p: u128) -> Option<u128> {
    if mod_pow(n, (p - 1) / 2, p) != 1 {
        return None;
    }

    let mut q = p - 1;
    let mut s = 0;

    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    if s == 1 {
        return Some(mod_pow(n, (p + 1) / 4, p));
    }

    let mut z = 2;
    while mod_pow(z, (p - 1) / 2, p) == 1 {
        z += 1;
    }

    let mut c = mod_pow(z, q, p);
    let mut r = mod_pow(n, (q + 1) / 2, p);
    let mut t = mod_pow(n, q, p);
    let mut m = s;
    while t != 0 && t != 1 {
        let mut i = 0;
        let mut t2 = t;

        while t2 != 1 {
            t2 = mod_pow(t2, 2, p);
            i += 1;
        }

        let b = mod_pow(c, 1 << (m - i - 1), p);
        r = (r * b) % p;
        c = (b * b) % p;
        t = (t * c) % p;
        m = i;
    }

    Some(r)
}

fn cornacchia(n: u128) -> u128 {
    if n % 4 == 3 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    let mut r1 = match tonelli_shanks(n - 1, n) {
        None => panic!("None"),
        Some(r) => r,
    };
    let mut r2 = n;

    while r1 * r1 > n {
        r2 %= r1;

        if r2 * r2 < n {
            return r2;
        }

        r1 %= r2;
    }
    r1
}

fn multiply_pairs(p1: Vec<u128>, p2: Vec<u128>) -> Vec<u128> {
    let (a, b) = (p1[0] as i128, p1[1] as i128);
    let (c, d) = (p2[0] as i128, p2[1] as i128);

    vec![(a * c - b * d).abs() as u128, (a * d + b * c) as u128]
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

fn two(n: u128) -> Vec<u128> {
    let factors = prime_factors(n);
    let mut m = 1;
    let mut l: Vec<u128> = vec![];

    for i in factors.keys() {
        m *= pow(*i, factors[i] / 2);

        if factors[i] % 2 == 1 {
            l.push(*i);
        }
    }

    let mut ans: Vec<u128> = vec![1, 0];

    for i in l.iter() {
        let k = cornacchia(*i);
        let res = [k, ((i - k * k) as f64).sqrt() as u128];
        ans = multiply_pairs(ans, Vec::from(res));
    }

    ans = Vec::from([ans[0] * m, ans[1] * m]);

    ans.sort();
    ans
}

fn three(n: u128) -> Vec<u128> {
    let factors = prime_factors(n);
    let mut m = 1;
    let mut k = 1;

    for i in factors.keys() {
        m *= pow(*i, factors[i] / 2);
        k *= pow(*i, factors[i] % 2);
    }

    let mut t = 1;
    while count(k - t * t) != 2 {
        t += 1;
    }

    let mut ans = two(k - t * t);
    ans.push(t);

    ans = Vec::from([ans[0] * m, ans[1] * m, ans[2] * m]);

    ans.sort();
    ans
}

fn four(mut n: u128) -> Vec<u128> {
    let mut ct = 0;

    while n % 4 == 0 {
        ct += 1;
        n /= 4;
    }

    let mut ans: Vec<u128> = three(n - 1);
    let pow_ct = pow(2, ct);
    ans = Vec::from([ans[0] * pow_ct, ans[1] * pow_ct, ans[2] * pow_ct]);
    ans.push(pow_ct);

    ans.sort();
    ans
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n: u128 = input.trim().parse::<u128>().unwrap();

    let tmp = n;
    let ch = count(n);

    writeln!(stdout(), "{}", ch)?;

    if ch == 1 {
        writeln!(stdout(), "{}", (tmp as f64).sqrt() as u128)?
    } else if ch == 2 {
        let ans = two(tmp);
        writeln!(stdout(), "{} {}", ans[0], ans[1])?
    } else if ch == 3 {
        let ans = three(tmp);
        writeln!(stdout(), "{} {} {}", ans[0], ans[1], ans[2])?
    } else {
        let ans = four(tmp);
        writeln!(stdout(), "{} {} {} {}", ans[0], ans[1], ans[2], ans[3])?
    }

    Ok(())
}
