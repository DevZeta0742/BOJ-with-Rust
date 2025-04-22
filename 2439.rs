use std::io::{stdin, stdout, Write};

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let input: usize = buf.trim().parse().unwrap();
    
    for i in 1..=input {
        stdout().write(&[
            &[b' '].repeat(input - i)[..], 
            &[b'*'].repeat(i)[..], 
            b"\n"].concat()
        ).unwrap();
    }
}

-------------------------------------------------------------------
use std::io::{stdin, stdout, Write};

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let input: usize = buf.trim().parse().unwrap();
    
    for i in 1..=input {
        stdout().write(&[b' '].repeat(input - i)).unwrap();
        stdout().write(&[b'*'].repeat(i)).unwrap();
        stdout().write(b"\n").unwrap();
    }
}
