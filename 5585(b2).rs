use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut money_str: String = String::new();
    io::stdin().read_line(&mut money_str).expect("error");
    let mut money: i32 = money_str.trim().parse().expect("not a number");
    let mut cnt: i32 = 0;
    money = 1000 - money;

    while money >= 500 {
        cnt += 1;
        money -= 500;
    }

    while money >= 100 {
        cnt += 1;
        money -= 100;
    }

    while money >= 50 {
        cnt += 1;
        money -= 50;
    }

    while money >= 10 {
        cnt += 1;
        money -= 10;
    }

    while money >= 5 {
        cnt += 1;
        money -= 5;
    }

    while money >= 1 {
        cnt += 1;
        money -= 1;
    }

    writeln!(out, "{}", cnt).expect("error");
}

///////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////

use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut money_str: String = String::new();
    io::stdin().read_line(&mut money_str).expect("error");
    let mut money: i32 = money_str.trim().parse().expect("not a number");
    let mut cnt: i32 = 0;
    money = 1000 - money;

    while money > 0 {
        if money >= 500 {
            money -= 500;
            cnt += 1;
        } else if money >= 100 {
            money -= 100;
            cnt += 1;
        } else if money >= 50 {
            money -= 50;
            cnt += 1;
        } else if money >= 10 {
            money -= 10;
            cnt += 1;
        } else if money >= 5 {
            money -= 5;
            cnt += 1;
        } else if money >= 1 {
            money -= 1;
            cnt += 1;
        }
    }

    writeln!(out, "{}", cnt).expect("error");
}
