#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

enum Option<T> {
    Ok(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(&self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
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

    let point_char = Point { x: 'a', y: 'b' };
    println!("{:?}", point_float.mixup(point_char));
}

fn largest_number<T>(list: &[T]) -> T {
    let mut largest: T = list[0];

    for &element in list {
        if element > largest {
            largest = element;
        }
    };

    return largest;    
}
