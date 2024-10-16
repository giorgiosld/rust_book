fn main() {
    println!("The 6-th fib number is {}", fib(6));
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}