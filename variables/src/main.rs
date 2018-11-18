const MAX_NUMBER_OF_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("The max number of points is {}", MAX_NUMBER_OF_POINTS);

    let tup: (i32, f64, u8) = (600, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    println!("The first variable in the tuplet is {}", tup.0);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Some array {:?}", array);
}
