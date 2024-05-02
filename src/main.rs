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

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            transposed[j][i] = matrix[i][j];
        }
    }

    return transposed;
}

fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut result = 0.0;
    for item in vector {
        result += item * item;
    }

    return result.sqrt();
}

fn normalize(vector: &mut [f64; 3]) {
    let module = magnitude(vector);
    for item in vector {
        *item /= module;
    }
}

fn main() {
    let n = 20;
    println!("fib(n) = {}", fib(n));

    println!("collatz_lengtgh(10) = {} ", collatz_length(10));
    println!("collatz_lengtgh(5) = {} ", collatz_length(5));
    println!("collatz_lengtgh(4) = {} ", collatz_length(4));
    println!("collatz_lengtgh(3) = {} ", collatz_length(3));

    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);

    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    normalize(&mut v);
    println!("Normalize {:#?}",v);
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    println!(" transposed{:#?}", transposed);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}
