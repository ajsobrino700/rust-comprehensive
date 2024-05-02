fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn collatz_length(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n % 2 == 0 {
        return collatz_length(n / 2) + 1;
    } else {
        return collatz_length(3 * n + 1) + 1;
    }
}

fn main() {
    let n = 20;
    println!("fib(n) = {}", fib(n));

    println!("collatz_lengtgh(10) = {} ", collatz_length(10));
    println!("collatz_lengtgh(5) = {} ", collatz_length(5));
    println!("collatz_lengtgh(4) = {} ", collatz_length(4));
    println!("collatz_lengtgh(3) = {} ", collatz_length(3));
}

#[test]
fn test_collatz_length(){
    assert_eq!(collatz_length(11),15);
}
