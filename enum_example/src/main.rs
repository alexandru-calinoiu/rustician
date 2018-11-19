enum IpAdrKind {
    V4(u8, u8,u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move  { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) { }
}

fn main() {
    let four = IpAdrKind::V4;
    let six = IpAdrKind::V6;

    let home = IpAdrKind::V4(127, 0, 0, 1);
    let loopback = IpAdrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    assert_eq!(some_number.unwrap(), 5);
    let some_string = Some("String");

    let absent_number: Option<i32> = None;
}

fn route(ip_type: IpAdrKind) { }