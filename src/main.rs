extern crate num;

use num::{Integer, One};

fn factorial<T: Integer>(num: &T) -> T {
    if *num == One::one() {
        return One::one();
    } else {
        return *num * factorial(&(*num - One::one()));
    }
}

fn main() {
    for x in 0..10 {
        println!("Factorial {} = {}", x, factorial(&x));
    }
}
