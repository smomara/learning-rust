fn main() {
    let n = 8;
    let fib_n = fib(n);

    println!("The {n}th fibonaci number is {fib_n}");
}

fn fib(n: i32) -> i32 {
    if n == 0 { return 0 }
    else if n == 1 { return 1 }

    fib(n - 1) + fib(n - 2)
}
