fn fib(x: u32) -> u32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn main() {
    for i in 1..1000 {
        println!("fib {} = {} ", i, fib(i));
    }
}
