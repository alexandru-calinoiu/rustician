fn main() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("The number is divisible by 4!");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3!");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    }

    let condition = true;

    let x = if condition {
        5
    } else {
        6
    };

    println!("x is {}", x);

    example_loop();
    example_for();
}

fn example_loop() {
    let mut counter = 0;

    loop {
        println!("I am in a loop!");
        
        if counter == 5 { break; };
        counter += 1;
    }
}

fn example_for() {
    let array = [1, 2, 42];

    for element in array.iter() {
        println!("{}", element);
    }

    for element in (1..4).rev() {
        println!("Element is {}", element);
    }
}
