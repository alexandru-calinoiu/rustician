fn main() {
    let string = String::from("Denisovici");
    
    let length = calculate_lenght(&string);

    println!("The lenght of {} is {}", string, length);

    let mut hello = String::from("Hello");
    muttable_reference(&mut hello);
    println!("{}", hello);

    muttable_reference(&mut hello);
}

fn calculate_lenght(string: &String) -> usize {
    string.len()
}

fn muttable_reference(string: &mut String) {
    string.push_str(", world!");
}

// can't have memory dangling
// fn dangle() -> &String {
//     let s = String::from("Hello");

//     &s
// }