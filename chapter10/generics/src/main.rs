#![allow(dead_code, unused_variables)]

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // the x method is available to any instance of Point
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    // the distance_from_origin method is only available to instances of Point that have the type f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointWithTwoGenerics<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointWithTwoGenerics<T, U> {
    fn mixup<V, W>(self, other: PointWithTwoGenerics<V, W>) -> PointWithTwoGenerics<T, W> {
        PointWithTwoGenerics {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };

    println!("Distance from origin: {}", float.distance_from_origin());

    let both_integer = PointWithTwoGenerics { x: 5, y: 10 };
    let both_float = PointWithTwoGenerics { x: 1.0, y: 4.0 };
    let integer_and_float = PointWithTwoGenerics { x: 5, y: 4.0 };

    let p1 = PointWithTwoGenerics { x: 5, y: 10.4 };
    let p2 = PointWithTwoGenerics { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
