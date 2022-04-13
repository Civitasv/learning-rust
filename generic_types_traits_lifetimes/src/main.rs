mod summary;
use summary::{NewArticle, Summary};

enum Test<T> {
    Why(T),
    What(T),
}

#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let number_list = vec![1, 2, 3, 4, 5, 6];

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 1.0 };
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    let article = NewArticle {
        headline: String::from("head"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };
    println!("{}", article.summarize());
    let number_list = vec![34, 50, 25, 100, 65];
    let result = function_generic(&number_list);
    println!("The largest number is {}", result);
    lifetimes();
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn function_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn lifetimes() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // } // after this, x doesn't make sense, cause x is allocated on the stack

    // println!("r: {}", r);
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let string3 = string2;
    println!("{}", string2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // this shows that return value have the same lifetime
    // with the smaller lifetime between x and y
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
