fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

fn display_arr<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn display_arr2<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

// trait
trait Summary {
    fn summarize(&self) {
        println!("default trait");
    }
}

struct Post {
    title: String,
    content: String,
}

struct Moment {
    title: String,
    content: String,
}

impl Summary for Post {}

impl Summary for Moment {
    fn summarize(&self) {
        println!("moment content = {}", self.content);
    }
}

// 使用特征作为函数参数，实现同一特征约束的函数共用
fn notify(item: &impl Summary) {
    item.summarize();
}

fn main() {
    // function genericse
    println!("res = {}", add(1, 2));
    println!("res = {}", add(10.0, 20.1));
    // method generics
    let pt = Point::new(10, 10);
    println!("x = {}", pt.x());

    let pt2 = Point::new(1.5, 2.3);
    println!("x = {}", pt2.x());

    // const generics： 针对数值的泛型
    // 1. use borrow
    let arr: [i32; 3] = [1, 2, 3];
    display_arr(&arr);

    let arr: [i32; 2] = [1, 2];
    display_arr(&arr);
    // 2. const generics
    let arr: [i32; 3] = [1, 2, 3];
    display_arr2(arr);

    let arr: [i32; 2] = [1, 2];
    display_arr2(arr);

    // trait demo
    let post = Post {
        title: String::from("xxx"),
        content: String::from("content1"),
    };
    let moment = Moment {
        title: String::from("xxx"),
        content: String::from("content2"),
    };

    post.summarize();
    moment.summarize();

    notify(&moment);
}
