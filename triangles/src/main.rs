use std::env;

fn triangle(width: u8) {
    for line in 1..(width + 1) {
        let mut stars = "".to_string();
        let mut padding = "".to_string();
        let padsize = (width + 1) - line;

        for _ in 1..padsize {
            padding.push_str(" ");
        }
        for _ in 1..(line + 1) {
            stars.push_str("* ");
        }
        println!("{}{}", padding, stars);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    triangle(args[1].parse().unwrap());
}
