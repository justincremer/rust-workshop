pub fn fac(n: i32) -> i32 {
    match n {
        0 | 1 => 1,
        _ => n * fac(n - 1),
    }
}
