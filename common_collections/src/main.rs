fn main() {
    vector();
    string();
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
