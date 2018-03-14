use std::env;

fn rot13(txt: &String) {
    let mut result: String = "".to_string();

    // For simplicity we convert to lowercase before switching to byte vals
    // A/a == 97, Z/z == 122
    // We baulk at any other char
    for chr in txt.to_ascii_lowercase().as_bytes() {
        if chr < &97 || chr > &122 {
            result = "Out of range chars found!".to_string();
            break;
        } else {
            // Good grief, I know there must be a smarter way to do this
            if (chr + 13) <= 122 {
                result.push((chr + 13) as char);
            } else {
                let overrun: u8 = (chr + 13) - 122;
                result.push((96 + overrun) as char);
            }
        }
    }

    println!("{}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    rot13(&args[1]);
}
