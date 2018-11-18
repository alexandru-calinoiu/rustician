fn main() {
    let string = String::from("Wizz air");

    let wizz = &string[0..=3];
    let air = &string[5..];

    println!("First is '{}' and then is '{}'", wizz, air);

    let greeting = String::from("Hello world!");
    println!("First word is '{}'", first_word(&greeting));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
