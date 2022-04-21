fn fractional(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * fractional(n - 1);
}

fn fractional1(n: i32, r: i32) -> i32 {
    if n <= 1 {
        return r;
    }
    return fractional1(n - 1, n * r);
}

fn handler() {
    let _a = fractional1(5, 3);
    println!("A: {_a}")
}
