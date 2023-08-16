// Rust使用struct存放数据，impl存放方法

#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    // 关联方法，第一个参数不是&self
    fn new(width: f32, height: f32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }

    // 方法，第一个参数是&self
    fn area(&self) -> f32 {
        self.height * self.width
    }

    // 需要修改类本身用&mut self
    fn set_width(&mut self, new_width: f32) {
        self.width = new_width;
    }
}

fn main() {
    let mut rect = Rectangle::new(10.0, 10.0);
    println!("area = {}", rect.area());
    rect.set_width(20.0);
    println!("new area = {}", rect.area());
}
