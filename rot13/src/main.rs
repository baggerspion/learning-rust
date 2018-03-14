use std::env;

fn rot13(chr: char) -> char {
    let start = match chr {
        'a'...'z' => 'a' as u8,
        'A'...'Z' => 'A' as u8,
        _ => return chr
    };

    let pos = chr as u8 - start;
    let rot = (pos + 13) % 26;
    (rot + start) as char
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut result: String = "".to_string();

    for letter in args[1].chars() {
        result.push(rot13(letter));
    }

    println!("{}", result);
}
