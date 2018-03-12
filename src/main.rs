fn factorial(num: i8) -> usize {
    let mut result: usize = 1;
    for x in 1..num as usize {
        result *= x;
    }
    return result;
}

fn main() {
    for x in 0..std::i8::MAX {
        println!("{}! = {}", x, factorial(x));
    }
}
