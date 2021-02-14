use std::io;

// exponential complexity T(n) = T(n-1) + T(n-2)
fn fib_recursive(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    return fib_recursive(n-1) + fib_recursive(n-2);
}

// complexity O(n)
fn fib_optimized(n: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128;
    if n == 0 {
        return a;
    }

    for _ in 2..n {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}

fn main() {
    println!("Which Fibonacci number do you want to know?");
    let n: u128 = loop {
        let mut n = String::new();
        io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

        match n.trim().parse::<u128>() {
            Ok(n) => break n,
            Err(_) => {
                println!("You did not enter a number. Please try again:");
                continue
            },
        };
    };
    let nth_fibonacci = fib_optimized(n);

    println!("Here it is: {}", nth_fibonacci);
    
}
