// 아직 미완성
// input : 658146524385565391  --> overflow
use std::collections::HashMap;
use std::io::{Result, Write, stdin, stdout};
use std::time::{SystemTime, UNIX_EPOCH};

const P: u64 = 998244353;

struct RandGenerator(u64);

impl From<u64> for RandGenerator {
    fn from(seed: u64) -> Self {
        RandGenerator(seed)
    }
}

impl RandGenerator {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("can't get current time")
            .as_nanos() as u64;

        RandGenerator(seed)
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn randint(&mut self, min: u32, max: u32) -> u32 {
        if min > max {
            panic!("min must be less than max");
        }

        let range = (max - min + 1) as u64;
        let random_value = self.next() % range;

        min + random_value as u32
    }
}

static mut RNG: RandGenerator = RandGenerator(0);

#[allow(static_mut_refs)]
fn randint(min: u64, max: u64) -> u64 {
    unsafe {
        let min_high = (min >> 32) as u32;
        let min_low = (min & 0x00000000FFFFFFFF) as u32;
        let max_high = (max >> 32) as u32;
        let max_low = (max & 0x00000000FFFFFFFF) as u32;

        let high = RNG.randint(min_high, max_high) as u64;
        let low = RNG.randint(min_low, max_low) as u64;

        (high << 32) | low
    }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
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

fn fft(a: &mut Vec<u64>, inv: bool) {
    let n = a.len();
    let mut j = 0;

    for i in 1..n {
        let mut rev = n / 2;
        while j >= rev {
            j -= rev;
            rev /= 2
        }
        j += rev;
        if i < j {
            a.swap(i, j);
        }
    }

    let mut step = 2;
    while step <= n {
        let half = step / 2;
        let u = mod_pow(3, (P - 1) / step as u64, P);
        let u = if inv { mod_pow(u, P - 2, P) } else { u };

        for i in (0..n).step_by(step) {
            let mut w = 1;
            for j in i..i+half {
                let v = a[j + half] * w % P;
                a[j + half] = (a[j] - v + P) % P;
                a[j] = (a[j] + v) % P;
                w = w * u % P;
            }
        }
        step *= 2;
    }

    if inv {
        let num = mod_pow(n as u64, P - 2, P);
        for i in 0..n {
            a[i] = a[i] * num % P;
        }
    }
}

fn multiply(a: &[u64], b: &[u64]) -> String {
    let mut n: u64 = 1;
    let total_length: usize = a.len() + b.len();
    while n < total_length as u64 {
        n *= 2;
    }

    let mut a_reversed: Vec<u64> = a.iter().rev().cloned().collect();
    a_reversed.resize(n as usize, 0);

    let mut b_reversed: Vec<u64> = b.iter().rev().cloned().collect();
    b_reversed.resize(n as usize, 0);

    fft(&mut a_reversed, false);
    fft(&mut b_reversed, false);

    let mut c: Vec<u64> = vec![0; n as usize];
    for i in 0..n as usize {
        c[i] = (a_reversed[i] * b_reversed[i]) % P;
    }

    fft(&mut c, true);

    let mut carry: u64 = 0;
    for i in 0..c.len() {
        carry += c[i];
        c[i] = carry % 10;
        carry /= 10;
    }

    while carry > 0 {
        c.push(carry % 10);
        carry /= 10;
    }

    let result: String = c.iter().rev().map(|&digit| digit.to_string()).collect();

    let result = result.trim_start_matches('0');
    if result.is_empty() {
        "0".to_string()
    } else {
        result.to_string()
    }
}

fn pow(base: u64, exp: u64) -> u64 {
    mod_pow(base, exp, u64::MAX)
}


fn is_prime(n: u64) -> bool {
    let pil = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];

    if n == 1 {
        return false;
    }

    if pil.contains(&n) {
        return true;
    }

    for a in &pil {
        if !millar_rabin(n, *a) {
            return false;
        }
    }
    true
}

fn millar_rabin(n: u64, k: u64) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 || n <= 1 {
        return false;
    }

    let mut s: u64 = 0;
    let mut d = n - 1;

    while d % 2 == 0 {
        s += 1;
        d /= 2;
    }

    for _ in 0..k {
        let a = randint(2, n - 1);
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

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn pollard_rho(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }

    if is_prime(n) {
        return n;
    }

    if n == 1 {
        return 1;
    }

    let mut x: u64 = randint(1, n - 1);
    let mut y: u64 = x;
    let c: u64 = randint(1, n - 1);
    let mut g: u64 = 1;

    while g == 1 {
        x = (mod_pow(x, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;
        
//        x = ((x * x) % n + c) % n;
//        y = ((y * y) % n + c) % n;
//        y = ((y * y) % n + c) % n;
        g = gcd((x as i64 - y as i64).abs(), n as i64) as u64;

        if g == n {
            return pollard_rho(n);
        }
    }

    if is_prime(g) { g } else { pollard_rho(g) }
}

fn prime_factors(mut n: u64) -> HashMap<u64, u64> {
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

fn tonelli_shanks(n: u64, p: u64) -> Option<u64> {
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
        mod_pow(n, (p + 1) / 4, p);
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
        
        r = multiply(&[r], &[b]).parse::<u64>().unwrap() % p;
        c = multiply(&[b], &[b]).parse::<u64>().unwrap() % p;
        t = multiply(&[t], &[c]).parse::<u64>().unwrap() % p;
//        r = (r * b) % p;
//        c = (b * b) % p;
//        t = (t * c) % p;
        m = i;
    }

    Some(r)
}

fn cornacchia(n: u64) -> u64 {
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

fn multiply_pairs(p1: Vec<u64>, p2: Vec<u64>) -> Vec<u64> {
    let (a, b) = (p1[0] as i64, p1[1] as i64);
    let (c, d) = (p2[0] as i64, p2[1] as i64);
    
    vec![(a * c - b * d).abs() as u64, (a * d + b * c) as u64]
}

fn count(mut n: u64) -> u64 {
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

        let sqrt = (n as f64).sqrt().floor() as u64;
        match sqrt * sqrt == n {
            true => 1,
            false => 2,
        }
    }
}

fn two(n: u64) -> Vec<u64> {
    let factors = prime_factors(n);
    let mut m = 1;
    let mut l: Vec<u64> = vec![];

    for i in factors.keys() {
        m *= pow(*i, factors[i] / 2);

        if factors[i] % 2 == 1 {
            l.push(*i);
        }
    }

    let mut ans: Vec<u64> = vec![1, 0];

    for i in l.iter() {
        let k = cornacchia(*i);
        let res = [k, ((i - k * k) as f64).sqrt() as u64];
        ans = multiply_pairs(ans, Vec::from(res));
    }

    ans = Vec::from([ans[0] * m, ans[1] * m]);

    ans.sort();
    ans
}

fn three(n: u64) -> Vec<u64> {
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

fn four(mut n: u64) -> Vec<u64> {
    let mut ct = 0;

    while n % 4 == 0 {
        ct += 1;
        n /= 4;
    }

    let mut ans: Vec<u64> = three(n - 1);
    let pow_ct = pow(2, ct);
    ans = Vec::from([ans[0] * pow_ct, ans[1] * pow_ct, ans[2] * pow_ct]);
    ans.push(pow_ct);

    ans.sort();
    ans
}

fn main() -> Result<()> {
    unsafe { RNG = RandGenerator::new() };

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let n: u64 = input.trim().parse::<u64>().unwrap();

    let tmp = n;
    let ch = count(n);

    writeln!(stdout(), "{}", ch)?;

    if ch == 1 {
        writeln!(stdout(), "{}", (tmp as f64).sqrt() as u64)?
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
