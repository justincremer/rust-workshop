use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        prompt();
    }
}

fn calculate<'a>(n1: f32, n2: f32, op: char) -> Result<f32, &'a str> {
    match op {
        '+' => Ok(n1 + n2),
        '-' => Ok(n1 - n2),
        '*' => Ok(n1 * n2),
        '/' => Ok(n1 / n2),
        _ => Err("Not a valid operator"),
    }
}

fn prompt() {}

fn read_num<'a>() -> Result<f32, &'a str> {}

fn read_op<'a>() -> Result<char, &'a str> {}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}
