fn main() {
    println!("Hello, world!");

    another_function(20, 22);

    let y = {
        let x = 3;
        x + 1
    };

    println!("Y is {}", y);

    println!("Five is {}", five());
}

fn another_function(x: i32, y: i32) {
    println!("Another function. Was called with: {} {}", x, y);
}

fn five() -> i32 {
    5
}