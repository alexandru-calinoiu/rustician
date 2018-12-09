use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
    println!("The longest string is {}", longest_with_announcement(string1.as_str(), string2.as_str(), "Important ann"));

    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
    println!("{}", i.level());
    println!("{}", i.announce_and_return_part("This is cool!"));

    println!("{}", first_word(first_sentence));

    let s: &'static str = "I have a static lifetime";
    println!("This is a static = {}", s);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn longest_with_announcement<'a, T>(str1: &'a str, str2: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    longest(str1, str2)
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
