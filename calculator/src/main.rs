use std::io::{stdin, stdout, Write};
use std::panic::panic_any;

fn main() {
    loop {
        prompt();
        match prompt_continue() {
            true => continue,
            false => return,
        }
    }
}

fn prompt_continue() -> bool {
    let mut res = String::new();
    print!("Continue? [y/n]: ");
    read(&mut res);
    match res.trim().chars().next().unwrap() {
        'y' => true,
        'n' => false,
        _ => {
            print!("Invalid answer ");
            prompt_continue()
        }
    }
}

fn prompt() {
    print!("Pick an operator [+-*/]: ");
    let op: char = read_op();
    print!("Pick the first number: ");
    let n1 = read_num();
    print!("Pick the second number: ");
    let n2 = read_num();

    match calculate(op, n1, n2) {
        Ok(n) => print!("{} {} {} = {}\n", n1, op, n2, n),
        Err(e) => panic_any(e),
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

    let res: char = res.trim().chars().next().unwrap();
    if "+-*/".contains(res) {
        res
    } else {
        print!("Invalid operator, try again [+-*/]: ");
        read_op()
    }
}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}
