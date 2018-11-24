use std::collections::HashMap;

fn main() {
    vector();
    string();
    hash_map();
}

fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 50);
    scores.insert(String::from("red"), 20);
    println!("{:?}", scores);

    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![50, 20];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("greed");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{}", field_name); will fail compile
    let team_name = String::from("blue");
    match scores.get(&team_name) {
        Some(score) => println!("The score for team {} is {}", team_name, score),
        None => println!("Could not find a score for team {}", team_name)
    };

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let color_key = String::from("Favorite color");
    map.insert(color_key, String::from("green"));
    println!("{:?}", map);

    let color_key = String::from("Favorite color");
    map.entry(color_key).or_insert(String::from("blue"));
    map.entry(String::from("Favorite pet")).or_insert(String::from("cat"));
    println!("{:?}", map);

    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);
}

fn string() {
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("Lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = "Здравствуйте";
    println!("The lenght of {} is {}", hello, hello.len());

    let s = &hello[0..4];
    println!("The slice is {}", s);

    let s = String::from("नमस्ते");
    for c in s.chars() {
        println!("{}", c);
    }
}

#[derive(Debug)]
enum SpreedSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector() {
    let _v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    println!("{:?}", v);

    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    let element = &v[2];
    println!("{}", element);

    let valid_index = 1;
    match v.get(valid_index) {
        Some(_) => println!("Trying to get vector at index {}", valid_index),
        _ => {}
    }

    let invalid_index = 42;
    match v.get(invalid_index) {
        None => println!("Unreachable index at {}", invalid_index),
        _ => {}
    }

    let mut v = vec![100, 20, 15];
    for item in &mut v {
        *item += 50;
    }

    println!("{:?}", v);

    let row = vec![
        SpreedSheetCell::Int(32),
        SpreedSheetCell::Float(3.141592),
        SpreedSheetCell::Text(String::from("Test")),
    ];

    println!("{:?}", row);
}
