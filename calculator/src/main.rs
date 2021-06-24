use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        prompt();
    }
}

fn prompt() {
    println!("Pick an operator [+-*/]: ");
    let op: char = read_op();
    println!("Pick the first number: ");
    let n1 = read_num();
    println!("Pick the second number: ");
    let n2 = read_num();

    match calculate(op, n1, n2) {
        Ok(n) => println!("{} {} {} = {}", n1, op, n2, n),
        Err(e) => panic!(e),
    }
}

fn calculate<'a>(op: char, n1: f32, n2: f32) -> Result<f32, &'a str> {
    match op {
        '+' => Ok(n1 + n2),
        '-' => Ok(n1 - n2),
        '*' => Ok(n1 * n2),
        '/' => Ok(n1 / n2),
        _ => Err("Not a valid operator"),
    }
}

fn read_num() -> f32 {
    let mut res = String::new();
    read(&mut res);
    res.trim().parse().unwrap()
}

fn read_op() -> char {
    let mut res = String::new();
    read(&mut res);
    res.trim().chars().next().unwrap()
}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}
