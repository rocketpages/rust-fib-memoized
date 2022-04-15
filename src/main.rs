use std::io;

fn main() {

    println!("Fibonacci calculator.");

    println!("How many places to calculate?");

    let mut places = String::new();

    io::stdin().read_line(&mut places).expect("Failed to read line.");

    let places: usize = places.trim().parse().expect("Please type a number.");

    let x = fib(places);

    println!("Fibonacci at {} is {}.", places, x);
}

fn fib(seed: usize) -> isize {
    if seed < 2 { 
        return seed.try_into().unwrap();
    }

    // "fibonacci at 10th place" means we need to store 11 values
    let mut memo: Vec<isize> = Vec::with_capacity(seed+1);

    for _i in 0..seed {
        memo.push(-1);
    }

    f(seed, &mut memo)
}

fn f(seed: usize, memo: &mut Vec<isize>) -> isize {
    if seed < 2 {
        return seed as isize;
    }

    let n2: isize = if memo[seed-2] == -1 {
        let fib_two = f(seed-2, memo);
        memo[seed-2] = fib_two;
        fib_two
    } else {
        memo[seed-2]
    };

    let n1: isize = if memo[seed-1] == -1 {
        let fib_one = f(seed-1, memo);
        memo[seed-1] = fib_one;
        fib_one
    } else {
        memo[seed-1]
    };

    n2 + n1
}

#[test]
fn test() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
    assert_eq!(fib(6), 8);
    assert_eq!(fib(10), 55);
}
