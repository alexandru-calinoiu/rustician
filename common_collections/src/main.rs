fn main() {
    vector();
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
