fn main() {
    let s = String::from("hello world");
    let slice = first_word(&s);
    println!("{}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
