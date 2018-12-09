use std::cmp::PartialOrd;
use std::fmt::Display;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Ok(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more ...")
    }
}

struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("headline: {}, content: {}", self.headline, self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y  = {}", self.y);
        }
    }
}

fn main() {
    let list = vec![10, 20, 1, 42];
    println!("{}", largest_number(&list));

    let list = vec![123124, 1231231, 434242, 23424, 2355, 99860, 1838410];
    println!("{}", largest_number(&list));

    let point_int = Point { x: 1, y: 2 };
    let point_float = Point { x: 1.0, y: 2.0 };
    let point_int_and_float = Point { x: 1, y: 2.0 };

    println!("p.x = {}", point_int.x());
    println!("p.y = {}", point_int_and_float.y);

    let point_char = Point { x: 'a', y: 'b' };
    println!("{:?}", point_float.mixup(point_char));

    let pair = Pair::new(10, 9);
    pair.cmp_display();
}

fn largest_number<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest: T = list[0];

    for &element in list {
        if element > largest {
            largest = element;
        }
    }

    return largest;
}
