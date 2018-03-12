fn factorial(num: u8) -> usize {
    let mut result: usize = 1;
    for x in 1..num as u8 {
        result *= x;
    }
    return result;
}

fn main() {
    for x in 0..std::u8::MAX {
        println!("{}! = {}", x, factorial(x));
    }
}
