use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    println!("{}", read_username_shortest("hello.txt").expect("Failed to read username"));
}

fn call_panic() {
    panic!("Crash and burn!");
}

fn panic_index() -> i32 {
    let v = vec![1, 2, 3];

    v[42]
}

fn open_file(file_name: String) {
    let f = File::open(&file_name);
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fc) => fc,
                Err(error) => panic!("Tryed to create the file but there was a problem {:?}", error)
            },
            other_error => panic!("There was a problem opening the file {:?}", other_error)
        }
    };
}

fn seasoned_open_file(file_name: String) {
    let _f = File::open(&file_name).map_err( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_name).unwrap_or_else( |error| {
                panic!("Tryed to create the file but there was a problem {:?}", error)
            })
        } else {
            panic!("There was a problem opening the file {:?}", error)
        }
    });
}

fn unwrap_open_file(file_name: &str) {
    File::open(file_name).unwrap();
}

fn expect_open_file(file_name: &str) {
    File::open(file_name).expect("Failed to open file");
}

fn read_username(file_name: &str) -> Result<String, io::Error> {
    let mut f = match File::open(file_name) {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut username = String::new();

    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_short(file_name: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut username = String::new();
    f.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_shortest(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}