enum Option2<T> {
    Some(T),
    None,
}

enum Result2<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    _x: T,
    _y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self._x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    _x: T,
    _y: U,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            _x: self._x,
            _y: other._y,
        }
    }
}

fn main() {
    generic_functions();
    generic_structs();
    generic_enums();
    generic_implementations();
}

fn generic_functions() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_structs() {
    let _integer = Point { _x: 5, _y: 10 };
    let _float = Point { _x: 1.0, _y: 4.0 };

    let _both_integer = Point2 { _x: 5, _y: 10 };
    let _both_float = Point2 { _x: 1.0, _y: 4.0 };
    let _integer_and_float = Point2 { _x: 5, _y: 4.0 };
}

fn generic_enums() {
    let _ok: Result2<&str, i32> = Result2::Ok("something");

    let _error: Result2<String, i32> = Result2::Err(3);

    let _maybe = Option2::Some(3.14);

    let _none: Option2<u32> = Option2::None;
}

fn generic_implementations() {
    let p = Point { _x: 5, _y: 10 };

    println!("p.x = {}", p.x());

    let p: Point<f32> = Point { _x: 5.0, _y: 10.0 };

    println!("Distance from origin: {}", p.distance_from_origin());

    let p1 = Point2 { _x: 5, _y: 10.4 };
    let p2 = Point2 { _x: "Hello", _y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3._x, p3._y);
}