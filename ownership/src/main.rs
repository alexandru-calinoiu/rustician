fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    variable_move();

    take_ownership(s);

    let x = 5;
    make_copy(x);
}

fn variable_move() {
    let x = 5;
    let y = x;

    println!("x is {} and y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 {} and s2 {}", s1, s2);
}

fn take_ownership(some_string: String) {
    println!("I own {}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("I copied {}", some_integer);
}