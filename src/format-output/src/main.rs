#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

// 实现display
use std::fmt;
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "this is my display: username = {}, password = {}",
            self.username, self.password
        )
    }
}

fn main() {
    // print!单行，println!带换行符，format!格式化输出字符串
    let s1 = format!("{}, world", "hello");
    println!("{}", s1);
    // eprintln!输出错误信息
    eprintln!("Err: xxxx");
    // rust使用{}自动推导占位符类型
    // {}实现了std::fmt::Display，用于普通输出
    // {:?}实现了std::fmt::Debug，用于debug输出
    // {:#?}用于优美的debug输出
    let user: User = User {
        username: "aaa".to_string(),
        password: "123".to_string(),
    };
    println!("user = {:?}", user);
    println!("user = {:#?}", user);
    println!("{}", user);
    // pos param
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // named param
    println!("name = {name}", name = "openhe");
    // format output
    let v = 3.1415926;
    println!("{:.2}", v); // display
    println!("{:.2?}", v); // debug
    // more ...
}
