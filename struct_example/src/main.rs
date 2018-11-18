struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8);
struct Point(u32, u32, u32);

fn main() {
    let user1 = User {
        email: String::from("ionica@agilefreaks.com"),
        username: String::from("ionica"),
        active: true,
        sign_in_count: 42
    };

    let user2 = build_user(String::from(""), String::from(""));

    let user3 = User {
        username: String::from("Ionica"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("First color is {}", black.0);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0
    }
}
